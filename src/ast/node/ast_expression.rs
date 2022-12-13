use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::value::Value;

type Wrap<T> = Rc<RefCell<T>>;
type WrapExpression = Wrap<ASTExpression>;

#[derive(Debug)]
pub enum ExpressionType {
    Number(ASTNumber),
    Variable(ASTVariable),
    String(ASTString),
    PrimaryExpression(ASTPrimaryExpression),
    OrExpression(ASTOrExpression),
    AndExpression(ASTAndExpression),
}

#[derive(Debug)]
pub struct ASTExpression {
    pub ast_type: ExpressionType,
}

impl ASTExpression {
    pub fn as_string(&self) -> Option<String> {
        match &self.ast_type {
            ExpressionType::String(s) => Some(s.value.clone()),
            _ => None
        }
    }

    pub fn as_loose_string(&self) -> Option<String> {
        match &self.ast_type {
            ExpressionType::String(s) => Some(s.value.clone()),
            ExpressionType::Number(n) => Some(f64::to_string(&n.value)),
            _ => None,
        }
    }

    pub fn is_var(&self) -> bool {
        match &self.ast_type {
            ExpressionType::Variable(_) => true,
            _ => false
        }
    }

    pub fn get_var_name(&self) -> Option<String> {
        match &self.ast_type {
            ExpressionType::Variable(x) => Some(x.name.clone()),
            _ => None
        }
    }

    pub fn try_to_value(&self) -> Option<Value> {
        match &self.ast_type {
            ExpressionType::Number(n) => {
                let number = n.value;
                Some(Value::from_number(number))
            },
            ExpressionType::String(s) => {
                Some(Value::from_string(s.value.as_str()))
            }
            _ => None
        }
    }

    pub fn from_or_expression(or_expression: ASTOrExpression) -> Self {
        ASTExpression {
            ast_type: ExpressionType::OrExpression(or_expression)
        }
    }

    pub fn from_and_expression(or_expression: ASTAndExpression) -> Self {
        ASTExpression {
            ast_type: ExpressionType::AndExpression(or_expression)
        }
    }

    pub fn from_primary_expression(expr: ASTPrimaryExpression) -> Self {
        ASTExpression {
            ast_type: ExpressionType::PrimaryExpression(expr)
        }
    }

    pub fn from_number(number: f64) -> Self {
        ASTExpression {
            ast_type: ExpressionType::Number(ASTNumber {
                value: number
            })
        }
    }

    pub fn from_str(s: &str) -> Self {
        ASTExpression {
            ast_type: ExpressionType::String(ASTString {
                value: String::from(s)
            })
        }
    }

    pub fn from_variable(name: &str, is_path: bool) -> Self {
        ASTExpression {
            ast_type: ExpressionType::Variable(ASTVariable {
                name: String::from(name),
                is_path
            })
        }
    }
}

#[derive(Debug)]
pub struct ASTNumber {
    pub value: f64,
}

#[derive(Debug)]
pub struct ASTVariable {
    pub name: String,
    pub is_path: bool,
}

#[derive(Debug)]
pub struct ASTString {
    pub value: String,
}

#[derive(Debug)]
pub struct ASTPrimaryExpression {
    pub predicate: Vec<String>,
    pub args: Vec<WrapExpression>,
}

#[derive(Debug)]
pub struct ASTAndExpression {
    pub left: WrapExpression,
    pub right: WrapExpression,
}

impl ASTAndExpression {
    pub fn new(left: WrapExpression, right: WrapExpression) -> Self {
        Self {
            left, right
        }
    }
}

#[derive(Debug)]
pub struct ASTOrExpression {
    pub left: WrapExpression,
    pub right: WrapExpression,
}

impl ASTOrExpression {
    pub fn new(left: WrapExpression, right: WrapExpression) -> ASTOrExpression {
        ASTOrExpression {
            left, right
        }
    }
}