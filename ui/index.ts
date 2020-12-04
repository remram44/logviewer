const form = document.getElementById('query-form')!;
const queryTextArea = document.getElementById('query-text') as HTMLTextAreaElement;
form.addEventListener('submit', (e) => {
  e.preventDefault();
  let query = queryTextArea.value;
});
