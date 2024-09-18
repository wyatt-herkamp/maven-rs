#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StringOrVariable {
    Variable(String),
    String(String),
}
