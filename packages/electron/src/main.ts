export {
  ScreenshotManager,
  type ScreenshotExecution,
  type ScreenshotBusyPolicy,
  type ScreenshotJobContext,
  type ScreenshotRunner,
  type ScreenshotManagerOptions,
  type ScreenshotManagerIpcMain,
} from './main/screenshot-manager.js';

export {
  ElectronCaptureAdapter,
  type ElectronCaptureAdapterOptions,
} from './main/electron-capture-adapter.js';

export { ScreenshotError } from './main/errors.js';

export {
  type ScreenshotDiagnosticContextValue,
  type ScreenshotDiagnosticEvent,
  type ScreenshotDiagnosticListener,
  type ScreenshotDiagnosticPhase,
  type ScreenshotDiagnosticStage,
} from './main/diagnostics.js';

export {
  ElectronOutputAdapter,
  type ElectronOutputAdapterOptions,
  type ScreenshotOutputExecutor,
} from './main/electron-output-adapter.js';

export {
  OverlayWindow,
  type OverlayWindowOptions,
  type ScreenshotOverlayWindow,
} from './main/overlay-window.js';

export {
  resolveHostPreloadPath,
  resolveOverlayResources,
  resolvePinnedResources,
  assertOverlayResources,
  assertPinnedResources,
  PackagedResourceError,
  type MissingPackagedResource,
  type OverlayResources,
  type PinnedResources,
  type PackagedResourceExists,
} from './main/resource-paths.js';

export {
  ScreenshotSession,
  type ScreenshotOverlayFactory,
  type ScreenshotSessionOptions,
  type ScreenshotSessionState,
} from './main/screenshot-session.js';

export {
  registerScreenshotIpc,
  setupElectronSnapora,
  type RegisterScreenshotIpcOptions,
  type SetupElectronSnaporaOptions,
  type SetupElectronSnaporaResult,
  type ValidateScreenshotIpcSender,
} from './main/register-host-ipc.js';

export {
  DEFAULT_HOST_CANCEL_CHANNEL,
  DEFAULT_HOST_CAPTURE_CHANNEL,
  OVERLAY_CHANNELS,
  PINNED_CHANNELS,
} from './protocol/channels.js';

export * from '@open-snapora/shared';
