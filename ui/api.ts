interface Query {
}

interface Results {
}

function query(req: Query): Promise<Results> {
  const options = {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json; charset=utf-8',
    },
    body: JSON.stringify(req),
  };
  return fetch('/api/query', options);
}
