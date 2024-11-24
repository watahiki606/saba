use alloc::string::String;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: u16,
    path: String,
    searchpart: String,
}
