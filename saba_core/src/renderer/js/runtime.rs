use crate::renderer::js::ast::Node;
use crate::renderer::js::ast::Program;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::cell::RefCell;
use core::ops::Add;
use core::ops::Sub;
#[derive(Debug, Clone)]
pub struct JsRuntime {
    env: Rc<RefCell<Environment>>,
}

type VariableMap = Vec<(String, Option<RuntimeValue>)>;

#[derive(Debug, Clone)]
pub struct Environment {
    variables: VariableMap,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new(outer: Option<Rc<RefCell<Environment>>>) -> Self {
        Self {
            variables: Vec::new(),
            outer,
        }
    }
}

impl JsRuntime {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Environment::new(None))),
        }
    }

    pub fn execute(&mut self, program: &Program) {
        for node in program.body() {
            self.eval(&Some(node.clone()));
        }
    }

    fn eval(&mut self, node: &Option<Rc<Node>>) -> Option<RuntimeValue> {
        let node = match node {
            Some(n) => n,
            None => return None,
        };

        match node.borrow() {
            Node::ExpressionStatement(expr) => return self.eval(&expr),
            Node::AdditiveExpression {
                operator,
                left,
                right,
            } => {
                let left_value = match self.eval(&left) {
                    Some(value) => value,
                    None => return None,
                };
                let right_value = match self.eval(&right) {
                    Some(value) => value,
                    None => return None,
                };

                if operator == &'+' {
                    Some(left_value + right_value)
                } else if operator == &'-' {
                    Some(left_value - right_value)
                } else {
                    None
                }
            }
            Node::AssignmentExpression {
                operator: _,
                left: _,
                right: _,
            } => {
                // 後ほど実装
                None
            }
            Node::MemberExpression { object, property } => {
                // 後ほど実装
                None
            }
            Node::NumericLiteral(value) => Some(RuntimeValue::Number(*value)),
            // ノードの種類を追加したので、デフォルトアームを追加
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Number(u64),
}

impl Add<RuntimeValue> for RuntimeValue {
    type Output = RuntimeValue;

    fn add(self, rhs: RuntimeValue) -> RuntimeValue {
        let (RuntimeValue::Number(left_num), RuntimeValue::Number(right_num)) = (&self, &rhs);
        return RuntimeValue::Number(left_num + right_num);
    }
}

impl Sub<RuntimeValue> for RuntimeValue {
    type Output = RuntimeValue;

    fn sub(self, rhs: RuntimeValue) -> RuntimeValue {
        let (RuntimeValue::Number(left_num), RuntimeValue::Number(right_num)) = (&self, &rhs);
        return RuntimeValue::Number(left_num - right_num);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderer::js::ast::JsParser;
    use crate::renderer::js::token::JsLexer;
    use alloc::string::ToString;

    #[test]
    fn test_num() {
        let input = "42".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new();
        let expected = [Some(RuntimeValue::Number(42))];
        let mut i = 0;

        for node in ast.body() {
            let result = runtime.eval(&Some(node.clone()));
            assert_eq!(expected[i], result);
            i += 1;
        }
    }

    #[test]
    fn test_add_nums() {
        let input = "1 + 2".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new();
        let expected = [Some(RuntimeValue::Number(3))];
        let mut i = 0;

        for node in ast.body() {
            let result = runtime.eval(&Some(node.clone()));
            assert_eq!(expected[i], result);
            i += 1;
        }
    }

    #[test]
    fn test_sub_nums() {
        let input = "2 - 1".to_string();
        let lexer = JsLexer::new(input);
        let mut parser = JsParser::new(lexer);
        let ast = parser.parse_ast();
        let mut runtime = JsRuntime::new();
        let expected = [Some(RuntimeValue::Number(1))];
        let mut i = 0;

        for node in ast.body() {
            let result = runtime.eval(&Some(node.clone()));
            assert_eq!(expected[i], result);
            i += 1;
        }
    }
}
