export class QueryBuilder {
  form: HTMLElement;
  queryTextArea: HTMLTextAreaElement;
  callback: (query: Query) => void;

  constructor(form: HTMLElement, callback: (query: Query) => void) {
    this.form = form;
    this.callback = callback;
    this.queryTextArea = this.form.querySelector('textarea') as HTMLTextAreaElement;

    this.form.addEventListener('submit', (e) => {
      e.preventDefault();
      let text = this.queryTextArea.value;
      let query: Query = JSON.parse(text);
      this.callback(query);
    });
  }
}
