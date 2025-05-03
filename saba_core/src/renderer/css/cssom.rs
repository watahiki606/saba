use crate::renderer::css::token::CssTokenizer;
use core::iter::Peekable;

#[derive(Debug, Clone)]
pub struct CssParser {
    t: Peekable<CssTokenizer>,
}

impl CssParser {
    pub fn new(t: CssTokenizer) -> Self {
        Self { t: t.peekable() }
    }
}
