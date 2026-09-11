/**
 * 撤销/重做命令接口
 */
export interface ScreenshotCommand<TDocument> {
  execute(document: TDocument): TDocument;
  undo(document: TDocument): TDocument;
}

/**
 * 撤销重做历史记录栈管理器
 */
export class CommandStack<TDocument> {
  readonly #undoStack: ScreenshotCommand<TDocument>[] = [];
  readonly #redoStack: ScreenshotCommand<TDocument>[] = [];

  /**
   * 执行新命令并压入撤销栈，同时清空重做栈
   */
  execute(command: ScreenshotCommand<TDocument>, document: TDocument): TDocument {
    const nextDocument = command.execute(document);
    this.#undoStack.push(command);
    this.#redoStack.length = 0;
    return nextDocument;
  }

  /**
   * 撤销上一步操作
   */
  undo(document: TDocument): TDocument {
    const command = this.#undoStack.pop();
    if (!command) return document;

    const previousDocument = command.undo(document);
    this.#redoStack.push(command);
    return previousDocument;
  }

  /**
   * 重做已撤销的操作
   */
  redo(document: TDocument): TDocument {
    const command = this.#redoStack.pop();
    if (!command) return document;

    const nextDocument = command.execute(document);
    this.#undoStack.push(command);
    return nextDocument;
  }

  /**
   * 清空历史记录
   */
  clear(): void {
    this.#undoStack.length = 0;
    this.#redoStack.length = 0;
  }

  /**
   * 是否可以执行撤销
   */
  get canUndo(): boolean {
    return this.#undoStack.length > 0;
  }

  /**
   * 是否可以执行重做
   */
  get canRedo(): boolean {
    return this.#redoStack.length > 0;
  }
}
