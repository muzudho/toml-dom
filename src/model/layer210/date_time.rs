//! Comment model.  
//! コメント・モデル。  
//!
//! # Examples
//!
//! ```
//! // # Toml's comment（トムルのコメント）
//! ```

use crate::model::{layer110::Token, layer210::DateTime};
use std::fmt;

impl Default for DateTime {
    fn default() -> Self {
        DateTime { tokens: Vec::new() }
    }
}
impl DateTime {
    pub fn extend_tokens(&mut self, tokens: &Vec<Token>) {
        self.tokens.extend(tokens.clone());
    }
    pub fn push_token(&mut self, token: &Token) {
        self.tokens.push(token.clone());
    }
    pub fn to_debug_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&token.to_string());
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for token in &self.tokens {
            buf.push_str(&token.to_debug_string());
        }
        write!(f, "{}", buf)
    }
}
