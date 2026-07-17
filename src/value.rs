use std::collections::{BTreeMap, HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    List(Vec<Value>),
    Map(Box<HashMap<String, Value>>),
    BTree(BTreeMap<String, Value>),
    Custom(Box<dyn CustomValue>)
}

pub trait CustomValue: std::fmt::Debug + 'static {
    fn clone_box(&self) -> Box<dyn CustomValue>;
    fn eq_box(&self, other: &dyn CustomValue) -> bool;
    fn as_any(&self) -> &dyn std::any::Any;
}

impl Clone for Box<dyn CustomValue> {fn clone(&self) -> Self {self.clone_box()}}
impl PartialEq for Box<dyn CustomValue> {fn eq(&self, other: &Self) -> bool {self.eq_box(other)}}

impl<T: 'static + std::fmt::Debug + Clone + PartialEq> CustomValue for T {
    fn clone_box(&self) -> Box<dyn CustomValue> {
        Box::new(self.clone())
    }
    fn eq_box(&self, other: &dyn CustomValue) -> bool {
        other.as_any().downcast_ref::<Self>().is_some_and(|oth| self == oth)
    }
    fn as_any(&self) -> &dyn std::any::Any {self}
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
    fn from(v: HashMap<String, T>) -> Self {
        Value::Map(Box::new(v.into_iter().map(|(k, v)| (k.into(), v.into())).collect()))
    }
}