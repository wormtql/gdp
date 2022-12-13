use std::collections::HashMap;
use crate::query::generic_queries::split_by::SplitBy;
use crate::runtime::frame::Frame;
use crate::runtime::value::Value as MyValue;

#[derive(Clone, Debug)]
pub enum VarOrValue<'a> {
    Var(String),
    Value(&'a MyValue),
}

impl<'a> VarOrValue<'a> {
    pub fn is_var(&self) -> bool {
        match self {
            VarOrValue::Value(_) => false,
            _ => true
        }
    }

    pub fn as_value(&self) -> Option<&'a MyValue> {
        match self {
            VarOrValue::Value(x) => Some(x),
            _ => None
        }
    }

    pub fn get_var_name(&self) -> Option<&str> {
        match self {
            VarOrValue::Value(_) => None,
            VarOrValue::Var(x) => Some(x.as_str())
        }
    }

    pub fn match_in_frame<'b>(&'a self, frame: &'b Frame) -> VarOrValue<'b> where 'a: 'b {
        match self {
            VarOrValue::Var(x) => {
                match frame.get(x.as_str()) {
                    Some(y) => VarOrValue::Value(y),
                    None => self.clone()
                }
            },
            VarOrValue::Value(_) => self.clone()
        }
    }
}

pub trait GenericQuery {
    fn query(&self, input: &[Frame], args: &[VarOrValue]) -> Option<Vec<Frame>>;
}

pub struct GenericQueries {
    pub entries: HashMap<String, Box<dyn GenericQuery>>,
}

impl GenericQueries {
    pub fn new() -> Self {
        Self {
            entries: HashMap::new()
        }
    }

    pub fn get(&self, name: &str) -> Option<&Box<dyn GenericQuery>> {
        self.entries.get(name)
    }
}

impl Default for GenericQueries {
    fn default() -> Self {
        let mut entries: HashMap<String, Box<dyn GenericQuery>> = HashMap::new();
        entries.insert(String::from("split_by"), Box::new(SplitBy));

        Self {
            entries
        }
    }
}
