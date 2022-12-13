use std::path::{Path, PathBuf};
use crate::ast::node::ast_expression::{ASTAndExpression, ASTExpression, ASTOrExpression, ASTPrimaryExpression, ExpressionType};
use crate::ast::parser::MyParser;
use crate::file_system::cached_file_system::CachedFileSystem;
use crate::file_system::file_system::FileSystem;
use crate::file_system::naive_file_system::NaiveFileSystem;
use crate::query::generic_query::{GenericQueries, VarOrValue};
use crate::runtime::frame::Frame;
use crate::runtime::value::{NonVariableValue, Value};

pub fn is_match_n(f1: &Frame, ast: &ASTPrimaryExpression, v: &[&Value]) -> Option<Frame> {
    let mut new_frame = f1.clone();
    let n = ast.args.len();

    for i in 0..n {
        let handle = ast.args[i].borrow();
        if handle.is_var() {
            let var_name = handle.get_var_name().unwrap();
            if let Some(x) = f1.get(&var_name) {
                if x.hash != v[i].hash {
                    return None;
                }
            }
            new_frame.add(&var_name, v[i].clone());
        } else {
            let v2 = handle.try_to_value()?;
            if v2.hash != v[i].hash {
                return None;
            }
        }
    }

    Some(new_frame)
}

pub fn access_serde(v: &serde_json::Value, accessor: &[&str]) -> Option<Value> {
    let mut temp = v;
    for a in accessor.iter() {
        if !temp.is_object() {
            return None;
        }
        let t = temp.as_object().unwrap().get(*a)?;
        temp = t;
    }

    let ret = NonVariableValue::from_serde(temp.clone()).to_value();
    Some(ret)
}

pub struct QueryProgram {
    pub generic_query: GenericQueries,
    pub file_system: Box<dyn FileSystem>,
}

impl Default for QueryProgram {
    fn default() -> Self {
        let fs1 = NaiveFileSystem::new(Path::new("E:\\rust\\gdp\\sub").to_path_buf());
        let fs2 = CachedFileSystem::new(Box::new(fs1));

        QueryProgram {
            generic_query: GenericQueries::default(),
            file_system: Box::new(fs2),
        }
    }
}

impl QueryProgram {
    // pub fn is_match(&self, )

    pub fn query(&self, q: &str) -> Option<Vec<Frame>> {
        let parser = MyParser;
        let ast = parser.parsestring_expression(q)?;
        // println!("{:?}", ast);

        let empty_frame = vec![Frame::new()];
        let x = self.query_internal(&empty_frame, &ast.borrow());
        x
    }

    fn query_internal(&self, input: &[Frame], ast: &ASTExpression) -> Option<Vec<Frame>> {
        match &ast.ast_type {
            ExpressionType::PrimaryExpression(p) => {
                let predicate = p.predicate[0].as_str();
                let locales = vec!["CHS", "CHT", "DE", "EN", "ES", "FR", "ID", "JP", "KR", "PT", "RU", "TH", "VI"];

                if locales.iter().find(|x| **x == predicate).is_some() {
                    self.query_locale(input, p)
                } else {
                    self.query_simple(input, p)
                }
            },
            ExpressionType::AndExpression(a) => self.query_and(input, a),
            ExpressionType::OrExpression(a) => self.query_or(input, a),
            _ => None
        }
    }

    pub fn query_and(&self, input: &[Frame], ast: &ASTAndExpression) -> Option<Vec<Frame>> {
        let r1 = self.query_internal(input, &ast.left.borrow())?;
        let r2 = self.query_internal(&r1, &ast.right.borrow());
        r2
    }

    pub fn query_or(&self, input: &[Frame], ast: &ASTOrExpression) -> Option<Vec<Frame>> {
        let mut r1 = self.query_internal(input, &ast.left.borrow()).unwrap_or(Vec::new());
        let mut r2 = self.query_internal(input, &ast.right.borrow()).unwrap_or(Vec::new());
        r1.append(&mut r2);
        if r1.len() == 0 {
            None
        } else {
            Some(r1)
        }
    }

    pub fn query_simple(&self, input: &[Frame], ast: &ASTPrimaryExpression) -> Option<Vec<Frame>> {
        let mut result = Vec::new();

        if let Some(mut x) = self.query_file_data_1(input, ast) {
            result.append(&mut x);
        }

        if let Some(mut x) = self.query_file_data_other(input, ast) {
            result.append(&mut x);
        }

        if let Some(mut x) = self.query_global_function(input, ast) {
            result.append(&mut x);
        }

        if result.len() == 0 {
            None
        } else {
            Some(result)
        }
    }

    pub fn query_file_data_1(&self, input: &[Frame], ast: &ASTPrimaryExpression) -> Option<Vec<Frame>> {
        let filename = &ast.predicate[0];
        let path = format!("ExcelBinOutput/{}.json", filename);
        let file_content = self.file_system.read(&path);

        if file_content.is_none() {
            None
        } else {
            if ast.args.len() != 1 {
                None
            } else {
                let mut result: Vec<Frame> = Vec::new();
                let json_value: serde_json::Value = serde_json::from_str(&file_content.as_ref().unwrap()).ok()?;
                if !json_value.is_array() {
                    return None;
                }
                let arr = json_value.as_array().unwrap();

                for frame in input.iter() {
                    for item in arr {
                        let wrapped_value: Value = NonVariableValue::from_serde(item.clone()).to_value();
                        if let Some(x) = is_match_n(frame, ast, &vec![&wrapped_value]) {
                            result.push(x);
                        }
                    }
                }

                Some(result)
            }
        }
    }

    pub fn query_file_data_other(&self, input: &[Frame], ast: &ASTPrimaryExpression) -> Option<Vec<Frame>> {
        let filename = &ast.predicate[0];
        let path = format!("ExcelBinOutput/{}.json", filename);
        let file_content = self.file_system.read(&path);

        if file_content.is_none() {
            None
        } else {
            if ast.args.len() != 2 {
                return None;
            }

            let mut result: Vec<Frame> = Vec::new();
            let json_value: serde_json::Value = serde_json::from_str(&file_content.as_ref().unwrap()).ok()?;
            if !json_value.is_array() {
                return None;
            }
            let arr = json_value.as_array().unwrap();

            for frame in input.iter() {
                for item in arr {
                    let accessor: Vec<_> = ast.predicate.iter().skip(1).map(|x| x.as_str()).collect();
                    let final_value = access_serde(item, &accessor)?;
                    let wrapped_item = NonVariableValue::from_serde(item.clone()).to_value();
                    if let Some(x) = is_match_n(frame, ast, &vec![&wrapped_item, &final_value]) {
                        result.push(x);
                    }
                }
            }

            Some(result)
        }
    }

    pub fn query_locale(&self, input: &[Frame], ast: &ASTPrimaryExpression) -> Option<Vec<Frame>> {
        let locale = ast.predicate[0].as_str();
        let content = self.file_system.read_serde(&format!("TextMap/TextMap{}.json", locale))?;

        if !content.is_object() {
            return None;
        }
        let obj = content.as_object().unwrap();

        if ast.args.len() != 2 {
            return None;
        }

        let mut result = Vec::new();
        // if either side of the query param is constant, use it to speed up.
        if !ast.args[0].borrow().is_var() {
            // key is constant
            let key = ast.args[0].borrow().as_loose_string()?;

            let k = Value::from_string(&key);
            let v = NonVariableValue::from_serde(obj.get(&key).unwrap().clone()).to_value();

            for f in input.iter() {
                if let Some(x) = is_match_n(f, ast, &vec![&k, &v]) {
                    result.push(x);
                }
            }
        } else if !ast.args[1].borrow().is_var() {
            // value is constant
            let text = ast.args[1].borrow().as_loose_string()?;
            // println!("tex: {}", text);

            let v = Value::from_string(&text);
            for key in obj.keys() {
                let value = obj.get(key.as_str()).unwrap();
                let value_str = value.as_str()?;
                if value_str == "" {
                    continue;
                }
                if value_str == text {
                    let k = Value::from_string(key.as_str());
                    for f in input.iter() {
                        if let Some(x) = is_match_n(f, ast, &vec![&k, &v]) {
                            result.push(x);
                        }
                    }
                }
            }
        } else {
            // this is slow, avoid using two vars in a locale query
            for frame in input.iter() {
                for key in obj.keys() {
                    let value = obj.get(key.as_str()).unwrap();
                    if value.as_str()? == "" {
                        continue;
                    }
                    let v2 = NonVariableValue::from_serde(value.clone()).to_value();
                    if let Some(x) = is_match_n(frame, ast, &vec![&Value::from_string(key.as_str()), &v2]) {
                        result.push(x);
                    }
                }
            }
        }

        Some(result)
    }

    pub fn query_global_function(&self, input: &[Frame], ast: &ASTPrimaryExpression) -> Option<Vec<Frame>> {
        let name = ast.predicate[0].as_str();
        let generic_query = self.generic_query.get(name)?;

        let handles: Vec<_> = ast.args.iter().map(|x| x.borrow()).collect();
        let values: Vec<_> = ast.args.iter().map(|x| x.borrow().try_to_value()).collect();

        let mut args = Vec::new();
        for (index, arg) in handles.iter().enumerate() {
            if arg.is_var() {
                let var_name = arg.get_var_name().unwrap();
                args.push(VarOrValue::Var(var_name));
            } else {
                args.push(VarOrValue::Value(values[index].as_ref().unwrap()));
            }
        }

        generic_query.query(input, &args)
    }
}
