import './pinned.css';
import type { PinnedImageApi } from '@open-snapora/shared';

function requireElement<T extends HTMLElement>(selector: string): T {
  const element = document.querySelector<T>(selector);
  if (!element) {
    throw new Error(`[open-snapora] Pinned window element is missing: ${selector}`);
  }
  return element;
}

let activePinnedBridge: PinnedImageApi | null = null;

function getPinnedBridge(): PinnedImageApi {
  const bridge =
    activePinnedBridge ??
    (typeof window !== 'undefined' ? window.snaporaPinned : undefined);
  if (!bridge) {
    throw new Error('[open-snapora] PinnedImageApi bridge is missing.');
  }
  return bridge;
}

export interface MountPinnedOptions {
  bridge?: PinnedImageApi;
}

/**
 * 显式挂载置顶贴图界面并连接宿主
 */
export function mountPinned(options: MountPinnedOptions = {}): () => void {
  if (options.bridge) {
    activePinnedBridge = options.bridge;
  }
  const bridge = getPinnedBridge();

  const surface = requireElement<HTMLElement>('.pinned-surface');
  const image = requireElement<HTMLImageElement>('.pinned-image');
  const closeButton = requireElement<HTMLButtonElement>('.pinned-close');
  const copyFeedback = requireElement<HTMLElement>('.pinned-copy-feedback');
  const copyFeedbackLabel = requireElement<HTMLElement>('.pinned-copy-feedback-label');
  const contextMenu = requireElement<HTMLElement>('.pinned-context-menu');
  const copyButton = requireElement<HTMLButtonElement>('.pinned-copy');
  const saveButton = requireElement<HTMLButtonElement>('.pinned-save');
  const menuCloseButton = requireElement<HTMLButtonElement>('.pinned-menu-close');
  const copyLabel = requireElement<HTMLElement>('.pinned-copy-label');
  const saveLabel = requireElement<HTMLElement>('.pinned-save-label');
  const menuCloseLabel = requireElement<HTMLElement>('.pinned-menu-close-label');
  const menuButtons = [copyButton, saveButton, menuCloseButton];

  let imageUrl: string | undefined;
  let activePointerId: number | undefined;
  let copyFeedbackTimer: number | undefined;

  function showCopyFeedback(): void {
    if (copyFeedbackTimer !== undefined) {
      window.clearTimeout(copyFeedbackTimer);
    }
    copyFeedback.hidden = false;
    copyFeedbackTimer = window.setTimeout(() => {
      copyFeedback.hidden = true;
      copyFeedbackTimer = undefined;
    }, 1500);
  }

  function hideContextMenu(): void {
    contextMenu.hidden = true;
  }

  function showContextMenu(point: { x: number; y: number }): void {
    contextMenu.hidden = false;
    const edge = 8;
    const maximumLeft = Math.max(
      edge,
      window.innerWidth - contextMenu.offsetWidth - edge
    );
    const maximumTop = Math.max(
      edge,
      window.innerHeight - contextMenu.offsetHeight - edge
    );
    contextMenu.style.left = `${Math.min(Math.max(point.x, edge), maximumLeft)}px`;
    contextMenu.style.top = `${Math.min(Math.max(point.y, edge), maximumTop)}px`;
    copyButton.focus();
  }

  const unsubscribeInit = bridge.onInitialize((payload) => {
    if (imageUrl) {
      URL.revokeObjectURL(imageUrl);
    }
    imageUrl = URL.createObjectURL(
      new Blob([Uint8Array.from(payload.data)], { type: payload.mimeType })
    );
    image.src = imageUrl;
    document.documentElement.lang = payload.locale;
    contextMenu.setAttribute('aria-label', payload.menuLabels.actions);
    copyLabel.textContent = payload.menuLabels.copy;
    copyFeedbackLabel.textContent = payload.menuLabels.copied;
    saveLabel.textContent = payload.menuLabels.save;
    menuCloseLabel.textContent = payload.menuLabels.close;
  });

  const unsubscribeCopied = bridge.onCopied(showCopyFeedback);

  const handleClose = () => bridge.close();
  const handleCopy = () => {
    hideContextMenu();
    bridge.copy();
  };
  const handleSave = () => {
    hideContextMenu();
    bridge.save();
  };

  closeButton.addEventListener('click', handleClose);
  copyButton.addEventListener('click', handleCopy);
  saveButton.addEventListener('click', handleSave);
  menuCloseButton.addEventListener('click', handleClose);

  const handleContextMenu = (event: MouseEvent) => {
    event.preventDefault();
    event.stopPropagation();
    showContextMenu({ x: event.clientX, y: event.clientY });
  };
  surface.addEventListener('contextmenu', handleContextMenu);

  const handleMenuKeydown = (event: KeyboardEvent) => {
    const currentIndex = menuButtons.indexOf(document.activeElement as HTMLButtonElement);
    if (event.key === 'ArrowDown' || event.key === 'ArrowUp') {
      event.preventDefault();
      const direction = event.key === 'ArrowDown' ? 1 : -1;
      const nextIndex =
        currentIndex < 0
          ? direction > 0
            ? 0
            : menuButtons.length - 1
          : (currentIndex + direction + menuButtons.length) % menuButtons.length;
      menuButtons[nextIndex]?.focus();
    } else if (event.key === 'Home' || event.key === 'End') {
      event.preventDefault();
      menuButtons[event.key === 'Home' ? 0 : menuButtons.length - 1]?.focus();
    }
  };
  contextMenu.addEventListener('keydown', handleMenuKeydown);

  // 悬停状态更新，保证右上角关闭按钮及右下角拖拽抓手及时显现
  const handlePointerEnter = () => {
    surface.dataset.hovered = 'true';
  };
  const handlePointerLeave = () => {
    delete surface.dataset.hovered;
  };
  surface.addEventListener('pointerenter', handlePointerEnter);
  surface.addEventListener('pointerleave', handlePointerLeave);

  const resizeHandle = document.querySelector<HTMLElement>('.pinned-resize-handle');
  let isResizing = false;
  let resizeStartScreenX = 0;
  let resizeStartWidth = 0;
  // 最小宽度不得小于右键菜单（160px）
  const PINNED_MIN_WIDTH = 160;

  if (resizeHandle) {
    resizeHandle.addEventListener('pointerdown', (e: PointerEvent) => {
      if (e.button !== 0) return;
      e.stopPropagation();
      e.preventDefault();
      isResizing = true;
      resizeStartScreenX = e.screenX;
      resizeStartWidth = window.innerWidth;
      resizeHandle.setPointerCapture(e.pointerId);
    });

    resizeHandle.addEventListener('pointermove', (e: PointerEvent) => {
      if (!isResizing) return;
      const naturalW = image.naturalWidth || window.innerWidth;
      const naturalH = image.naturalHeight || window.innerHeight;
      const aspectRatio = naturalW / naturalH;

      const deltaX = e.screenX - resizeStartScreenX;
      const newWidth = Math.max(PINNED_MIN_WIDTH, Math.round(resizeStartWidth + deltaX));
      const newHeight = Math.round(newWidth / aspectRatio);

      bridge.resize?.({ width: newWidth, height: newHeight });
    });

    const endResize = (e: PointerEvent) => {
      if (!isResizing) return;
      isResizing = false;
      try {
        resizeHandle.releasePointerCapture(e.pointerId);
      } catch {}
    };

    resizeHandle.addEventListener('pointerup', endResize);
    resizeHandle.addEventListener('pointercancel', endResize);
  }

  // 支持滚轮对贴图进行自由等比例缩放
  const handleWheel = (e: WheelEvent) => {
    e.preventDefault();
    const naturalW = image.naturalWidth || window.innerWidth;
    const naturalH = image.naturalHeight || window.innerHeight;
    const aspectRatio = naturalW / naturalH;

    const zoomFactor = e.deltaY < 0 ? 1.06 : 0.94;
    const currentW = window.innerWidth;
    const newWidth = Math.max(PINNED_MIN_WIDTH, Math.round(currentW * zoomFactor));
    const newHeight = Math.round(newWidth / aspectRatio);

    bridge.resize?.({ width: newWidth, height: newHeight });
  };
  surface.addEventListener('wheel', handleWheel, { passive: false });

  const handlePointerDown = (event: PointerEvent) => {
    const target = event.target as Element;
    if (!contextMenu.hidden && !contextMenu.contains(target)) {
      event.preventDefault();
      hideContextMenu();
      return;
    }
    if (
      event.button !== 0 ||
      event.target === closeButton ||
      closeButton.contains(target) ||
      contextMenu.contains(target) ||
      (resizeHandle && (event.target === resizeHandle || resizeHandle.contains(target)))
    ) {
      return;
    }
    activePointerId = event.pointerId;
    surface.setPointerCapture(event.pointerId);
    surface.dataset.dragging = 'true';
    bridge.startDrag({ x: event.screenX, y: event.screenY });
  };
  surface.addEventListener('pointerdown', handlePointerDown);

  const handlePointerMove = (event: PointerEvent) => {
    if (event.pointerId === activePointerId) {
      bridge.moveDrag({ x: event.screenX, y: event.screenY });
    }
  };
  surface.addEventListener('pointermove', handlePointerMove);

  const endDrag = (event: PointerEvent) => {
    if (event.pointerId !== activePointerId) {
      return;
    }
    activePointerId = undefined;
    delete surface.dataset.dragging;
    bridge.endDrag();
  };
  surface.addEventListener('pointerup', endDrag);
  surface.addEventListener('pointercancel', endDrag);

  const handleKeydown = (event: KeyboardEvent) => {
    if (event.key === 'Escape') {
      event.preventDefault();
      if (contextMenu.hidden) {
        bridge.close();
      } else {
        hideContextMenu();
      }
    }
  };
  window.addEventListener('keydown', handleKeydown);
  window.addEventListener('blur', hideContextMenu);

  return () => {
    unsubscribeInit();
    unsubscribeCopied();
    closeButton.removeEventListener('click', handleClose);
    copyButton.removeEventListener('click', handleCopy);
    saveButton.removeEventListener('click', handleSave);
    menuCloseButton.removeEventListener('click', handleClose);
    surface.removeEventListener('contextmenu', handleContextMenu);
    contextMenu.removeEventListener('keydown', handleMenuKeydown);
    surface.removeEventListener('pointerdown', handlePointerDown);
    surface.removeEventListener('pointermove', handlePointerMove);
    surface.removeEventListener('pointerup', endDrag);
    surface.removeEventListener('pointercancel', endDrag);
    window.removeEventListener('keydown', handleKeydown);
    window.removeEventListener('blur', hideContextMenu);
    if (copyFeedbackTimer !== undefined) {
      window.clearTimeout(copyFeedbackTimer);
    }
    if (imageUrl) {
      URL.revokeObjectURL(imageUrl);
    }
  };
}

// 自动检测宿主桥接环境挂载
if (typeof window !== 'undefined' && window.snaporaPinned) {
  mountPinned();
}
