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
impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(v: Vec<T>) -> Self {Value::List(v.into_iter().map(|item| item.into()).collect())}
}
impl<T: Into<Value>> From<HashMap<String, T>> for Value {
    fn from(v: HashMap<String, T>) -> Self {Value::Map(v.into_iter().map(|(k, v)| (k.into(), v.into())).collect())}
}

impl Value {
    const DESERIALIZE_ERR: &str = "Could not deserialize object (Invalid JSON).";

    /// Serializes Value into a JSON-compliant string that can be inserted into the value of a JSON object. Note that the process is not lossless as NaN and Infinity will be converted to null.
    pub fn serialize(&self) -> String {
        match &self {
            Self::Null => "null".to_string(),
            Self::Bool(data) => data.to_string(),
            Self::Int(data) => data.to_string(),
            Self::Float(data) => {if data.is_nan() || data.is_infinite() {"null".to_string()} else {data.to_string()}}
            Self::Text(data) => {Self::escape(data)},
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

    /// Deserializes JSON object (in the form of a String) to a Value object, returining Err in case of invalid JSON.
    /// This process is lossy, as numbers that do not fit into an i64 or f64 will be truncated.
    pub fn deserialize(obj: &str) -> Result<Self, &'static str> {
        match obj {
            "null" => Ok(Value::Null),
            "true" => Ok(Value::Bool(true)),
            "false" => Ok(Value::Bool(false)),
            _ if obj.starts_with('"') && obj.ends_with('"') => {
                let data = obj.strip_prefix('"').and_then(|s| s.strip_suffix('"')).ok_or(Self::DESERIALIZE_ERR)?;
                Ok(Value::Text(Self::unescape(data)?))
            },
            _ if obj.starts_with('[') && obj.ends_with(']') => {
                let data = obj.strip_prefix('[').and_then(|s| s.strip_suffix(']')).ok_or(Self::DESERIALIZE_ERR)?;
                Ok(Value::List(Self::parse_collection(data)?.into_iter().map(|(_, v)| v).collect::<Vec<Value>>()))
            },
            _ if obj.starts_with('{') && obj.ends_with('}') => {
                let data = obj.strip_prefix('{').and_then(|s| s.strip_suffix('}')).ok_or(Self::DESERIALIZE_ERR)?;
                Ok(Value::Map(Self::parse_collection(data)?.into_iter().collect::<HashMap<String, Value>>()))
            }
            _ if obj.contains(".") => {
                let data = obj.parse::<f64>().map_err(|_| Self::DESERIALIZE_ERR)?;
                Ok(Value::Float(data))
            }
            _ => {
                let data = obj.parse::<i64>().map_err(|_| Self::DESERIALIZE_ERR)?;
                Ok(Value::Int(data))
            }
        }

    }

    fn unescape(data: &str) -> Result<String, &'static str> {
        let mut data_string = String::with_capacity(data.len());

        let mut char_iter = data.chars();
        while let Some(char) = char_iter.next() {
            if char == '\\' {
                let next_char = char_iter.next().ok_or(Self::DESERIALIZE_ERR)?;

                match next_char {
                    'n' => data_string.push('\n'),
                    'r' => data_string.push('\r'),
                    't' => data_string.push('\t'),
                    '\\' => data_string.push('\\'),
                    '"' => data_string.push('"'),
                    'u' => {
                        let hex_str: String = (0..4).map(|_| char_iter.next().ok_or(Self::DESERIALIZE_ERR))
                            .collect::<Result<String, _>>()?; // Collecting into Result short-circuits on Err
                        let hex = u32::from_str_radix(&hex_str, 16).map_err(|_| Self::DESERIALIZE_ERR)?;
            
                        data_string.push(char::from_u32(hex).ok_or(Self::DESERIALIZE_ERR)?);
                    },
                    _ => Err(Self::DESERIALIZE_ERR)?
                };
            } else {
                data_string.push(char);
            }
        }

        Ok(data_string)
    }

   pub(crate) fn parse_collection(data: &str) -> Result<Vec<(String, Value)>, &'static str> {
        let (mut brackets, mut braces, mut quotes) = (0, 0, 0);
        let mut data_chars = data.chars();

        let mut current_key = String::with_capacity(data.len());
        let mut current_data = String::with_capacity(data.len());
        let mut pieces: Vec<(String, Value)> = vec![];

        while let Some(char) = data_chars.next() {
            match char  {
                '[' if quotes == 0 => {brackets += 1;},
                ']' if quotes == 0 => {brackets -= 1;},
                '{' if quotes == 0 => {braces += 1;},
                '}' if quotes == 0 => {braces -= 1;},
                '"' if quotes == 0 => {quotes += 1;},
                '"' => {quotes -= 1;}
                '\\' => {
                    current_data.push(char);
                    current_data.push(data_chars.next().ok_or(Self::DESERIALIZE_ERR)?);
                    continue; 
                }
                ',' if brackets == 0 && braces == 0 && quotes == 0 => {
                    let key = current_key.trim();
                    let key = key.strip_prefix('"').and_then(|s| s.strip_suffix('"')).unwrap_or(key);

                    pieces.push((Self::unescape(&key)?, Self::deserialize(&current_data.trim())?));
                    current_key.clear();
                    current_data.clear();
                    continue;
                }
                ':' if brackets == 0 && braces == 0 && quotes == 0 => {
                    current_key.push_str(&current_data);
                    current_data.clear();
                    continue;
                }
                _ => {}
            }
            current_data.push(char);
            
        }
        if !current_data.trim().is_empty() {
            let key = current_key.trim();
            let key = key.strip_prefix('"').and_then(|s| s.strip_suffix('"')).unwrap_or(key);
            pieces.push((Self::unescape(&key)?, Self::deserialize(&current_data.trim())?));
        }

        Ok(pieces)
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_from_conversions() {
        assert_eq!(Value::from(()), Value::Null);
        assert_eq!(Value::from(true), Value::Bool(true));
        assert_eq!(Value::from(42), Value::Int(42));
        assert_eq!(Value::from(3.14), Value::Float(3.14));
        assert_eq!(Value::from("hello"), Value::Text("hello".to_string()));
        
        let vec_val: Value = vec![1, 2, 3].into();
        assert_eq!(vec_val, Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]));
    }

    #[test]
    fn test_serialization_primitives() {
        assert_eq!(Value::Null.serialize(), "null");
        assert_eq!(Value::Bool(true).serialize(), "true");
        assert_eq!(Value::Int(-100).serialize(), "-100");
        assert_eq!(Value::Float(0.5).serialize(), "0.5");
        assert_eq!(Value::Float(f64::NAN).serialize(), "null");
    }

    #[test]
    fn test_escaping_and_unescaping() {
        let original = "Line1\nLine2\t\"Quotes\"\\Slash";
        let escaped = Value::escape(original);
        assert_eq!(escaped, r#""Line1\nLine2\t\"Quotes\"\\Slash""#);
        
        // Strip the outer quotes for unescape testing as `deserialize` handles the outer quotes
        let unescaped = Value::unescape(&escaped[1..escaped.len()-1]).unwrap();
        assert_eq!(unescaped, original);
    }

    #[test]
    fn test_deserialization_success() {
        assert_eq!(Value::deserialize("null").unwrap(), Value::Null);
        assert_eq!(Value::deserialize("false").unwrap(), Value::Bool(false));
        assert_eq!(Value::deserialize("12345").unwrap(), Value::Int(12345));
        assert_eq!(Value::deserialize("12.34").unwrap(), Value::Float(12.34));
        assert_eq!(Value::deserialize(r#""hello""#).unwrap(), Value::Text("hello".to_string()));
    }

    #[test]
    fn test_roundtrip_complex_structures() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), Value::Text("value1".to_string()));
        map.insert("key2".to_string(), Value::List(vec![Value::Int(1), Value::Null]));
        
        let original_val = Value::Map(map);
        let serialized = original_val.serialize();
        let deserialized = Value::deserialize(&serialized).unwrap();
        
        assert_eq!(original_val, deserialized);
    }
}