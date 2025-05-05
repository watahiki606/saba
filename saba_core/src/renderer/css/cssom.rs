use crate::alloc::string::ToString;
use crate::renderer::css::token::CssToken;
use crate::renderer::css::token::CssTokenizer;
use alloc::string::String;
use alloc::vec::Vec;
use core::iter::Peekable;

#[derive(Debug, Clone, PartialEq)]
pub struct StyleSheet {
    pub rules: Vec<QualifiedRule>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QualifiedRule {
    pub selector: Selector,
    pub declarations: Vec<Declaration>,
}

impl QualifiedRule {
    pub fn new() -> Self {
        Self {
            selector: Selector::TypeSelector("".to_string()),
            declarations: Vec::new(),
        }
    }

    pub fn set_selector(&mut self, selector: Selector) {
        self.selector = selector;
    }

    pub fn set_declarations(&mut self, declarations: Vec<Declaration>) {
        self.declarations = declarations;
    }
}

pub enum Selector {
    TypeSelector(String),
    ClassSelector(String),
    IdSelector(String),
    UnknownSelector,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub property: String,
    pub value: ComponentValue,
}

impl Declaration {
    pub fn new() -> Self {
        Self {
            property: String::new(),
            value: ComponentValue::Ident(String::new()),
        }
    }

    pub fn set_property(&mut self, property: String) {
        self.property = property;
    }

    pub fn set_value(&mut self, value: ComponentValue) {
        self.value = value;
    }
}

pub type ComponentValue = CssToken;

impl StyleSheet {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub fn set_rules(&mut self, rules: Vec<QualifiedRule>) {
        self.rules = rules;
    }
}

#[derive(Debug, Clone)]
pub struct CssParser {
    t: Peekable<CssTokenizer>,
}

impl CssParser {
    pub fn new(t: CssTokenizer) -> Self {
        Self { t: t.peekable() }
    }

    pub fn parse_stylesheet(&mut self) -> StyleSheet {
        // StyleSheet 構造体のインスタンスを作成
        let mut sheet = StyleSheet::new();

        // トークン列からルールのリストを作成し、StyleSheet のフィールドに設定
        sheet.set_rules(self.consume_list_of_rules());
        sheet
    }

    fn consume_list_of_rules(&mut self) -> Vec<QualifiedRule> {
        // 空のベクタを作成
        let mut rules = Vec::new();

        loop {
            let token = match self.t.peek() {
                Some(t) => t,
                None => return rules,
            };

            match token {
                // AtKeyword トークンが出てくた場合、ほかの CSS をインポートする
                // @import 、メディアクエリを表す @media などのルールが始まることを表す
                CssToken::AtKeyword(_keyword) => {
                    let _rule = self.consume_qualified_rule();
                    // しかし、本書のブラウザでは @ から始まるルールはサポートしないので、無視する
                }
                _ => {
                    // １つのるーる　を解釈し、ベクタに追加する
                    let rule = self.consume_qualified_rule();
                    match rule {
                        Some(r) => rules.push(r),
                        None => return rules,
                    }
                }
            }
        }
    }

    fn consume_qualified_rule(&mut self) -> Option<QualifiedRule> {
        let mut rule = QualifiedRule::new();

        loop {
            let token = match self.t.peek() {
                Some(t) => t,
                None => return None,
            };

            match token {
                CssToken::OpenCurly => {
                    assert_eq!(self.t.next(), Some(CssToken::OpenCurly));
                    rule.set_declarations(self.consume_list_of_declarations());
                    return Some(rule);
                }
                _ => {
                    rule.set_selector(self.consume_selector());
                }
            }
        }
    }

    fn consume_selector(&mut self) -> Selector {
        let token = match self.t.next() {
            Some(t) => t,
            None => panic!("should have a token but got None"),
        };

        match token {
            CssToken::HashToken(value) => Selector::IdSelector(value[1..].to_string()),
            CssToken::Delim(delim) => {
                if delim == '.' {
                    return Selector::ClassSelector(self.consume_ident());
                }
                panic!("Parse error: {:?} is an unexpected token.", token);
            }
            CssToken::Ident(ident) => {
                // a:hover のようなセレクタはタイプセレクタとして扱うため、もし
                // コロン(:) が出てきた場合は宣言ブロックの開始直前までトークンを進める
                if self.t.peek() == Some(&CssToken::Colon) {
                    while self.t.peek() != Some(&CssToken::OpenCurly) {
                        self.t.next();
                    }
                }
                Selector::TypeSelector(ident.to_string())
            }
            CssToken::AtKeyword(_keyword) => {
                // @ から始まるルールを無視するために、宣言ブロックの開始直前までトークンを進める
                while self.t.peek() != Some(&CssToken::OpenCurly) {
                    self.t.next();
                }
                Selector::UnknownSelector
            }
            _ => {
                self.t.next();
                Selector::UnknownSelector
            }
        }
    }

    fn consume_list_of_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::new();

        loop {
            let token = match self.t.peek() {
                Some(t) => t,
                None => return declarations,
            };

            match token {
                CssToken::OpenCurly => {
                    assert_eq!(self.t.next(), Some(CssToken::CloseCurly));
                    return declarations;
                }
                CssToken::SemiColon => {
                    assert_eq!(self.t.next(), Some(CssToken::SemiColon));
                    // 一つの宣言が終了。なにもしない。
                }
                CssToken::Ident(ref _ident) => {
                    if let Some(declaration) = self.consume_declaration() {
                        declarations.push(declaration);
                    }
                }
                _ => {
                    self.t.next();
                }
            }
        }
    }
}
