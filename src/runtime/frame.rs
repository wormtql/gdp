use std::collections::HashMap;
use crate::runtime::value::{NonVariableValue, Value};

#[derive(Debug, Clone)]
pub enum ConstraintTarget {
    Variable(String),
    NonVariable(Value),
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub constraints: HashMap<String, ConstraintTarget>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            constraints: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Value> {
        let target = self.constraints.get(name)?;
        match target {
            ConstraintTarget::Variable(_) => None,
            ConstraintTarget::NonVariable(v) => Some(v)
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.constraints.contains_key(name)
    }

    pub fn add(&mut self, name: &str, value: Value) {
        self.constraints.insert(String::from(name), ConstraintTarget::NonVariable(value));
    }

    pub fn add_serde(&mut self, name: &str, value: serde_json::Value) {
        let v = NonVariableValue::from_serde(value).to_value();
        let target = ConstraintTarget::NonVariable(v);
        self.constraints.insert(String::from(name), target);
    }

    pub fn is_resolved(&self) -> bool {
        for v in self.constraints.values() {
            if let ConstraintTarget::Variable(_) = v {
                return false;
            }
        }
        true
    }

    pub fn to_serde_map(&self) -> serde_json::Value {
        let mut result = serde_json::Map::new();
        for k in self.constraints.keys() {
            let v = self.constraints.get(k.as_str()).unwrap();
            if let ConstraintTarget::NonVariable(x) = v {
                result.insert(k.clone(), x.get_serde_value().clone());
            }
        }

        serde_json::Value::Object(result)
    }
}
