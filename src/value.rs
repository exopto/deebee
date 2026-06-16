/// The world's smallest JSON serializer and deserializer, built for Deebee. It is cheap, quick, and has questionable quality, just like Temu!
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl From<()> for Value {fn from(_: ()) -> Self {Value::Null}}
impl From<bool> for Value {fn from(v: bool) -> Self {Value::Bool(v)}}
impl From<i64> for Value {fn from(v: i64) -> Self {Value::Int(v)}}
impl From<u32> for Value {fn from(v: u32) -> Self {Value::Int(v as i64)}}
impl From<i32> for Value {fn from(v: i32) -> Self {Value::Int(v as i64)}}
impl From<isize> for Value {fn from(v: isize) -> Self {Value::Int(v as i64)}}
impl From<f64> for Value {fn from(v: f64) -> Self {Value::Float(v)}}
impl From<f32> for Value {fn from(v: f32) -> Self {Value::Float(v as f64)}}
impl From<&str> for Value {fn from(v: &str) -> Self {Value::Text(v.to_string())}}
impl From<String> for Value {fn from(v: String) -> Self {Value::Text(v)}}
impl From<HashMap<String, Value>> for Value {fn from(v: HashMap<String, Value>) -> Self {Value::Map(v)}}
impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(v: Vec<T>) -> Self {Value::List(v.into_iter().map(|item| item.into()).collect())}
}

impl Value {
    /// Serializes Value into a JSON-compliant string that can be inserted into the value of a JSON object. Note that the process is not lossless as NaN and Infinity will be converted to null.
    pub fn serialize(&self) -> String {
        match &self {
            Self::Null => "null".to_string(),
            Self::Bool(data) => data.to_string(),
            Self::Int(data) => data.to_string(),
            Self::Float(data) => {if data.is_nan() || data.is_infinite() {"null".to_string()} else {data.to_string()}}
            Self::Text(data) => Self::escape(data),
            Self::List(data) => {format!("[{}]", data.iter().map(|item| item.serialize()).collect::<Vec<_>>().join(", "))}
            Self::Map(data) => {format!(
                    "{{{}}}", // Outer braces are escaped
                    data.iter()
                        .map(|(key, val)| format!("{}: {}", Self::escape(key), val.serialize()))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
    }

    fn escape(data: &str) -> String {
        let mut data_string = String::with_capacity(data.len() + 2);
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

    // Deserializes JSON object (in the form of a String) to a Value object. This process is lossless.
    pub fn deserialize(obj: &str) -> Self {
        match obj {
            "null" => Value::Null,
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            data if obj.starts_with('"') && obj.ends_with('"') => {
                Value::Text(data.trim_matches(|c| c == '"' || c == '"').to_string())
            }
            data if obj.starts_with('[') && obj.ends_with(']') => {
                Value::List(data.trim_matches(|c| c == '[' || c == ']')
                                .split(',')
                                .map(|x| Self::deserialize(x.trim()))
                                .collect::<Vec<Value>>())
            }
            _ => todo!()
        }
    }

    // fn unescape(data: &str)

    // Self::Int(data) => data.to_string(),
    //         Self::Float(data) => {if data.is_nan() || data.is_infinite() {"null".to_string()} else {data.to_string()}}
    //         Self::Text(data) => Self::escape(data),
    //         Self::List(data) => {format!("[{}]", data.iter().map(|item| item.serialize()).collect::<Vec<_>>().join(", "))}
    //         Self::Map(data) => {format!(
    //                 "{{{}}}", // Outer braces are escaped
    // Map and Text, then
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(data) => write!(f, "{data}"),
            Value::Int(data) => write!(f, "{data}"),
            Value::Float(data) => write!(f, "{data}"),
            Value::Text(data) => write!(f, "{data}"),
            Value::List(data) => write!(f, "{data:?}"),
            Value::Map(data) => write!(f, "{data:?}"),
        }
    }
}