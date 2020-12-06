type _ExprRecord = 'record';

interface _ExprVar {
  'var': string;
}

interface _ExprLastVarValue {
  'lastVarValue': string;
}

interface _ExprConstant {
  'constant': string;
}

type Expression = _ExprRecord | _ExprVar | _ExprLastVarValue | _ExprConstant;

interface Condition {
  match: {
    expression: Expression;
    pattern: string;
  };
}

interface _OperationIf {
  if: {
    condition: Condition;
    'then': Operation[];
    'else': Operation[];
  };
}

interface _OperationSet {
  set: {
    target: string;
    expression: Expression;
  };
}

interface _OperationColorBy {
  colorBy: Expression;
}

type _OperationSkipRecord = 'skipRecord';

type Operation = _OperationIf | _OperationSet | _OperationColorBy | _OperationSkipRecord;

interface Query {
  operations: Operation[];
}

interface LogRecord {
}

interface Results {
  records: LogRecord[];
}

class ApiError extends Error {
  status: number;
  apiMessage?: string;

  constructor(status: number, apiMessage?: string) {
    super(apiMessage || '');
    this.status = status;
    this.apiMessage = apiMessage;
  }
}

async function query(req: Query): Promise<Results> {
  const options = {
    method: 'POST',
    headers: {
      'Accept': 'application/json',
      'Content-Type': 'application/json; charset=utf-8',
    },
    body: JSON.stringify(req),
  };
  const response = await fetch('/api/query', options);
  if(response.status !== 200) {
    let json;
    try {
      json = await response.json();
    } catch(e) {
      throw new ApiError(response.status);
    }
    throw new ApiError(response.status, json.error);
  } else {
    return await response.json() as Results;
  }
}
