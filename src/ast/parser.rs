use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use pest::iterators::Pair;
use pest::Parser;
use crate::ast::node::ast_expression::{ASTAndExpression, ASTExpression, ASTOrExpression, ASTPrimaryExpression};

#[derive(Parser)]
#[grammar = "gdp.pest"]
pub struct GDPParser;

pub fn parse<'i>(input: &'i str) -> Option<Pair<'i, Rule>> {
    // let string = "WeaponExcelConfigData.nameTextMapHash ?x ?y";
    let parsed = GDPParser::parse(Rule::expression, input).ok()?.next()?;
    // println!("{:?}", parsed);
    Some(parsed)
}

pub struct MyParser;

type ExpressionParseResult = Option<Rc<RefCell<ASTExpression>>>;

impl MyParser {
    pub fn parse_expression(&self, pair: Pair<Rule>) -> ExpressionParseResult {
        let rule = pair.as_rule();
        // println!("{:?}", rule);
        use Rule::*;
        match rule {
            primary_expression => self.parse_primary_expression(pair),
            and_expression => self.parse_and_expression(pair),
            or_expression => self.parse_or_expression(pair),
            expression => self.parse_expression(pair.into_inner().next().unwrap()),
            value => self.parse_value(pair),
            _ => {
                None
            }
        }
    }

    pub fn parse_or_expression(&self, pair: Pair<Rule>) -> ExpressionParseResult {
        let pairs: Vec<Pair<Rule>> = pair.into_inner().collect();
        if pairs.len() == 1 {
            self.parse_expression(pairs.into_iter().next().unwrap())
        } else {
            let left = self.parse_expression(pairs[0].clone())?;
            let right = self.parse_expression(pairs[1].clone())?;
            let or_expression = ASTOrExpression::new(left, right);
            let mut ast = Rc::new(RefCell::new(ASTExpression::from_or_expression(or_expression)));

            for i in 2..pairs.len() {
                let expression = self.parse_expression(pairs[i].clone())?;
                let or_expression = ASTOrExpression::new(ast.clone(), expression);
                ast = Rc::new(RefCell::new(ASTExpression::from_or_expression(or_expression)));
            }
            Some(ast)
        }
    }

    pub fn parse_and_expression(&self, pair: Pair<Rule>) -> ExpressionParseResult {
        let pairs: Vec<Pair<Rule>> = pair.into_inner().collect();
        if pairs.len() == 1 {
            self.parse_expression(pairs.into_iter().next().unwrap())
        } else {
            let left = self.parse_expression(pairs[0].clone())?;
            let right = self.parse_expression(pairs[1].clone())?;
            let and_expression = ASTAndExpression::new(left, right);
            let mut ast = Rc::new(RefCell::new(ASTExpression::from_and_expression(and_expression)));

            for i in 2..pairs.len() {
                let expression = self.parse_expression(pairs[i].clone())?;
                let and_expression = ASTAndExpression::new(ast.clone(), expression);
                ast = Rc::new(RefCell::new(ASTExpression::from_and_expression(and_expression)));
            }
            Some(ast)
        }
    }

    pub fn parse_value(&self, pair: Pair<Rule>) -> ExpressionParseResult {
        let p = pair.into_inner().next()?;
        let rule = p.as_rule();

        use Rule::*;
        match rule {
            number => self.parse_number(p),
            string => self.parse_string(p),
            variable => self.parse_variable(p),
            _ => None
        }
    }

    pub fn parse_variable(&self, pair: Pair<Rule>) -> ExpressionParseResult {
        let s = pair.as_str();
        Some(Rc::new(RefCell::new(
            ASTExpression::from_variable(s, false)
        )))
    }

    pub fn parse_string(&self, pair: Pair<Rule>) -> ExpressionParseResult {
        let s = pair.into_inner().next()?.as_str();
        Some(Rc::new(RefCell::new(
            ASTExpression::from_str(s)
        )))
    }

    pub fn parse_number(&self, pair: Pair<Rule>) -> ExpressionParseResult {
        let number = pair.as_str().parse::<f64>().ok()?;
        Some(Rc::new(RefCell::new(ASTExpression::from_number(number))))
    }

    pub fn parse_primary_expression(&self, pair: Pair<Rule>) -> ExpressionParseResult {
        let pairs: Vec<Pair<Rule>> = pair.into_inner().collect();

        if pairs.len() == 1 {
            // an expression
            self.parse_expression(pairs[0].clone())
        } else {
            let predicate = pairs[0].as_str();
            let predicate: Vec<String> = predicate.split(".").map(|x| String::from(x)).collect();

            let mut args = Vec::new();
            for i in 1..pairs.len() {
                let expr = self.parse_expression(pairs[i].clone())?;
                args.push(expr);
            }

            let ast = ASTPrimaryExpression {
                predicate,
                args
            };
            let ast = ASTExpression::from_primary_expression(ast);
            Some(Rc::new(RefCell::new(ast)))
        }
    }

    pub fn parsestring_expression(&self, s: &str) -> ExpressionParseResult {
        let pair = GDPParser::parse(Rule::expression, s).ok()?.next()?;
        self.parse_expression(pair)
    }
}