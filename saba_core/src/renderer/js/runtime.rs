use crate::renderer::js::ast::Node;
use crate::renderer::js::ast::Program;
use alloc::rc::Rc;
use core::borrow::Borrow;
#[derive(Debug, Clone)]
pub struct JsRuntime {}

impl JsRuntime {
    pub fn new() -> Self {
        Self {}
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
        }
    }
}
