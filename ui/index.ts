import { LogViewer } from './viewer';
import { QueryBuilder } from './query';
import { Timeline } from './timeline';

function sendQuery(query: Query) {
  // TODO
  console.log(query);
}

function jumpToTime() {
  // TODO
}

const container = document.getElementById('logs') as HTMLElement;
const viewer = new LogViewer(container);

const form = document.getElementById('query-form') as HTMLElement;
new QueryBuilder(document.getElementById('query-form')!, sendQuery);

const timeline = document.getElementById('timeline') as HTMLElement;
new Timeline(timeline, jumpToTime);
