#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Punctuator(char),
    Number(u64),
}
