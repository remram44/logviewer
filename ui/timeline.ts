export class Timeline {
  container: HTMLElement;
  callback: () => void;

  constructor(container: HTMLElement, callback: () => void) {
    this.container = container;
    this.callback = callback;
  }
}
