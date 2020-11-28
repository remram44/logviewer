pub enum Operation {
    If(Condition, Vec<Operation>, Vec<Operation>),
    Set(String, Expression),
    ColorBy(Expression),
    SkipRecord,
}

pub enum Expression {
    Record,
    Var(String),
    LastVarValue(String),
    Constant(String),
}

pub enum Condition {
    Match(Expression, Pattern),
}

pub struct Pattern {
    pub regex: String,
    pub groups: Vec<String>,
}

pub struct View(pub Vec<Operation>);

impl View {
    pub fn from_json() -> View {
        todo!() // JSON
    }

    pub fn to_json(&self) {
        todo!() // JSON
    }
}
