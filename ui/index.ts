import { greeter } from './greeter';

const element = document.getElementById('app');
if(element) {
  element.textContent = greeter("Remi");
}
