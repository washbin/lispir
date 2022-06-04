#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Object {
    Void,
    Bool(bool),
    Integer(i64),
    Lambda(Vec<String>, Vec<Object>),
    Symbol(String),
    List(Vec<Object>),
}
