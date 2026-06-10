/// The world's smallest JSON serializer and deserializer, built for Deebee. It is cheap, quick, and has questionable quality, just like Temu!
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl From<bool> for Value {
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}
impl From<i64> for Value {
    fn from(v: i64) -> Self {
        Value::Int(v)
    }
}
impl From<u32> for Value {
    fn from(v: u32) -> Self {
        Value::Int(v as i64)
    }
}
impl From<i32> for Value {
    fn from(v: i32) -> Self {
        Value::Int(v as i64)
    }
}
impl From<isize> for Value {
    fn from(v: isize) -> Self {
        Value::Int(v as i64)
    }
}
impl From<f64> for Value {
    fn from(v: f64) -> Self {
        Value::Float(v)
    }
}
impl From<f32> for Value {
    fn from(v: f32) -> Self {
        Value::Float(v as f64)
    }
}
impl From<&str> for Value {
    fn from(v: &str) -> Self {
        Value::Text(v.to_string())
    }
}
impl From<String> for Value {
    fn from(v: String) -> Self {
        Value::Text(v)
    }
}
impl From<Vec<u8>> for Value {
    fn from(v: Vec<u8>) -> Self {
        Value::Bytes(v)
    }
}
impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self {
        Value::List(v)
    }
}
impl From<HashMap<String, Value>> for Value {
    fn from(v: HashMap<String, Value>) -> Self {
        Value::Map(v)
    }
}
impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Null
    }
}


impl Value {
    pub fn serialize(&self) -> String {
        match &self {
            Self::Null => "null".to_string(),
            Self::Bool(data) => data.to_string(),
            Self::Int(data) => data.to_string(),
            Self::Float(data) => {
                if data.is_nan() || data.is_infinite() {
                    "null".to_string()
                } else {
                    data.to_string()
                }
            }
            Self::Text(data) => {
                let mut data_string = String::with_capacity(data.len());
                data_string.push('"');

                for str_char in data.chars() {
                    match str_char {
                        '"' => data_string.push_str(r#"\""#),
                        '\\' => data_string.push_str(r"\\"),
                        '\n' => data_string.push_str(r"\n"),
                        '\r' => data_string.push_str(r"\r"),
                        '\t' => data_string.push_str(r"\t"),
                        str_char if (str_char as u32) < 0x20 => {
                            data_string.push_str(&format!("\\u{:04x}", str_char as u32)); // Zero-padded 4 char hex
                        }
                        _ => data_string.push(str_char),
                    }
                }
                
                data_string.push('"');
                data_string
            }
            Self::Bytes(data) => format!("{data:?}"),
            Self::List(data) => {
                format!(
                    "[{}]",
                    data.iter()
                        .map(|item| item.serialize())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Self::Map(data) => {
                format!(
                    "{{{}}}", // Outer braces are escaped
                    data.iter()
                        .map(|(key, val)| format!("\"{}\": {}", key, val.serialize()))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }

    // pub fn from_json(data) -> Value {

    // }
}
