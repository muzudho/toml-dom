//! Inline model.  
//! インライン・モデル。  

use crate::model::{InlineTable, ItemValue, KeyValue};
use std::fmt;

impl Default for InlineTable {
    fn default() -> Self {
        InlineTable { items: Vec::new() }
    }
}
impl InlineTable {
    pub fn push_key_value(&mut self, m: &KeyValue) {
        self.items.push(ItemValue::KeyValue(m.clone()));
    }
}
impl fmt::Debug for InlineTable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!("{:?},", item))
        }
        write!(f, "{{ {} }}", buf)
    }
}