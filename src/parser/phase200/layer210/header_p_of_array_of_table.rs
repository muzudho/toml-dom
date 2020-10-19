//! Array of HeaderOfArrayOfTable syntax parser.  
//! テーブルの配列構文パーサー。  

use crate::model::{
    layer110::{Token, TokenType},
    layer230::HeaderOfArrayOfTable,
};
use crate::parser::phase200::layer210::{HeaderPOfArrayOfTable, PResult};
use crate::parser::phase200::LookAheadTokens;
// use casual_logger::Table;

impl HeaderPOfArrayOfTable {
    pub fn flush(&mut self) -> Option<HeaderOfArrayOfTable> {
        let m = self.buffer.clone();
        self.buffer = None;
        m
    }
    pub fn new() -> Self {
        HeaderPOfArrayOfTable {
            buffer: Some(HeaderOfArrayOfTable::default()),
        }
    }
    /// # Arguments
    ///
    /// * `tokens` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///                             結果。
    pub fn parse(
        &mut self,
        tokens_old: (Option<&Token>, Option<&Token>, Option<&Token>),
    ) -> PResult {
        let tokens = LookAheadTokens::from_old(tokens_old);
        let token0 = tokens.current.as_ref().unwrap();
        match token0.type_ {
            TokenType::DoubleQuotation => {
                // End of syntax.
                // 構文の終わり。
                return PResult::End;
            }
            _ => {
                let m = self.buffer.as_mut().unwrap();
                m.push_token(&token0);
            }
        }
        PResult::Ongoing
    }
    /* TODO
    /// Log.
    /// ログ。
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();
        if let Some(m) = &self.buffer {
            t.str("value", &m.to_string());
        }
        t
    }
    */
}
