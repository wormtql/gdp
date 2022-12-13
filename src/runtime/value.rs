use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct NonVariableValue {
    pub v: serde_json::Value,
}

impl NonVariableValue {
    pub fn from_serde(value: serde_json::Value) -> Self {
        Self {
            v: value
        }
    }

    pub fn to_value(self) -> Value {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        let h = hasher.finish();
        Value {
            value_type: self,
            hash: h
        }
    }
}

fn hash_serde_value<H: Hasher>(state: &mut H, value: &serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();
            for &k in keys.iter() {
                k.hash(state);
                let value = map.get(k.as_str()).unwrap();
                hash_serde_value(state, value);
            }
        },
        serde_json::Value::Array(arr) => {
            for item in arr {
                hash_serde_value(state, item);
            }
        },
        serde_json::Value::String(s) => s.hash(state),
        serde_json::Value::Number(n) => {
            let f = n.as_f64().unwrap();
            if f.fract().abs() < 1e-6 {
                // is an integer, convert to string and hash
                let s = f64::to_string(&f.trunc());
                s.hash(state);
            } else {
                // the precision is 1e-6
                let i = (f * 1e6_f64) as i64;
                i.hash(state);
            }
        },
        serde_json::Value::Bool(b) => b.hash(state),
        serde_json::Value::Null => 0.hash(state),
    }
}

impl Hash for NonVariableValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_serde_value(state, &self.v);
    }
}

#[derive(Debug, Clone)]
pub struct Value {
    pub value_type: NonVariableValue,
    pub hash: u64,
}

impl Value {
    pub fn from_number(n: f64) -> Self {
        let v = serde_json::json!(n);
        NonVariableValue::from_serde(v).to_value()
    }

    pub fn from_string(s: &str) -> Self {
        let v = serde_json::json!(s);
        NonVariableValue::from_serde(v).to_value()
    }

    pub fn get_serde_value(&self) -> &serde_json::Value {
        &self.value_type.v
    }

    pub fn as_string(&self) -> Option<&str> {
        match &self.value_type.v {
            serde_json::Value::String(s) => Some(s.as_str()),
            _ => None
        }
    }

    pub fn as_loose_string(&self) -> Option<String> {
        match &self.value_type.v {
            serde_json::Value::String(s) => Some(s.clone()),
            serde_json::Value::Number(n) => {
                Some(n.as_f64().unwrap().to_string())
            }
            _ => None
        }
    }
}
