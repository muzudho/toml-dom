//! Escape sequence parser.  
//! エスケープ・シーケンス・パーサー。  

use crate::model::layer110::token::tokens_stringify;
use crate::model::layer110::{CharacterType, Token, TokenType};
use crate::parser::phase200::error;
use crate::parser::phase200::error_via;
use crate::parser::phase200::layer210::PositionalNumeralStringP;
use crate::parser::phase200::layer210::{EscapeSequenceP, PResult};
use crate::parser::phase200::LookAheadCharacters;
use casual_logger::Table;
use std::char::from_u32;

/// Syntax machine state.  
/// 構文状態遷移。  
#[derive(Debug, Clone)]
pub enum State {
    End,
    // After double quotation.
    // 二重引用符の後。
    First,
    // After `\`.
    // `\` の後。
    EscapedCharacter,
    UnicodeDigits,
}

impl Default for EscapeSequenceP {
    fn default() -> Self {
        EscapeSequenceP {
            positional_numeral_string_p: None,
            buffer: Vec::new(),
            state: State::First,
            string_buffer: String::new(),
        }
    }
}
impl EscapeSequenceP {
    pub fn flush(&mut self) -> Vec<Token> {
        let m = self.buffer.clone();
        self.buffer.clear();
        m
    }
    /// # Arguments
    ///
    /// * `characters` - Tokens contains look ahead.  
    ///             先読みを含むトークン。  
    /// # Returns
    ///
    /// * `PResult` - Result.  
    ///               結果。
    pub fn parse(&mut self, characters: &LookAheadCharacters) -> PResult {
        let character0 = characters.current.as_ref().unwrap();
        match self.state {
            State::End => {
                return error(&mut self.log(), &characters, "escape_sequence_p.rs.66.");
            }
            State::First => {
                // Look-ahead.
                // 先読み。
                if let Some(token_1_ahead) = characters.one_ahead.as_ref() {
                    match token_1_ahead.type_ {
                        CharacterType::Alpha
                        | CharacterType::Backslash
                        | CharacterType::DoubleQuotation => {
                            // print!("[trace1 (IgnoreBackslash) ahead={:?}]", token_1_ahead);
                            self.state = State::EscapedCharacter;
                        }
                        CharacterType::Newline => {
                            // 行末に \ があったケース。
                            // println!("[trace3 行末にEOLがあったケース]");
                            self.state = State::End;
                            return PResult::End;
                        }
                        _ => {
                            return error(
                                &mut self.log(),
                                &characters,
                                "escape_sequence_p.rs.136.",
                            );
                        }
                    }
                } else {
                    return error(&mut self.log(), &characters, "escape_sequence_p.rs.112.");
                }
            }
            State::EscapedCharacter => {
                // println!("[trace196={:?}]", character0);
                // Escaped.
                match character0.type_ {
                    // `"`
                    CharacterType::Alpha => {
                        // TODO 汎用的に書けないか？
                        // https://doc.rust-lang.org/reference/characters.html
                        let mut code = None;
                        match character0.to_string().as_str() {
                            "n" => code = Some("\n"),
                            "r" => code = Some("\r"),
                            "t" => code = Some("\t"),
                            "u" => {
                                self.state = State::UnicodeDigits;
                                self.string_buffer = String::new();
                                self.positional_numeral_string_p = Some(
                                    PositionalNumeralStringP::new("0x")
                                        .set_expected_digits(4)
                                        .clone(),
                                );
                            }
                            "U" => {
                                self.state = State::UnicodeDigits;
                                self.string_buffer = String::new();
                                self.positional_numeral_string_p = Some(
                                    PositionalNumeralStringP::new("0x")
                                        .set_expected_digits(8)
                                        .clone(),
                                );
                            }
                            _ => {
                                return error(
                                    &mut self.log(),
                                    &characters,
                                    "escape_sequence_p.rs.206.",
                                )
                            }
                        }
                        if let Some(code) = code {
                            self.buffer.push(Token::new(
                                character0.column_number,
                                code,
                                TokenType::EscapeSequence, // TODO EscapeSequence
                            ));
                            self.state = State::End;
                            return PResult::End;
                        }
                    }
                    CharacterType::Backslash => {
                        self.buffer.push(Token::new(
                            character0.column_number,
                            "\\",
                            TokenType::EscapeSequence, // TODO EscapeSequence
                        ));
                        self.state = State::End;
                        return PResult::End;
                    }
                    // "
                    CharacterType::DoubleQuotation => {
                        self.buffer.push(Token::new(
                            character0.column_number,
                            "\"",
                            TokenType::EscapeSequence, // TODO EscapeSequence
                        ));
                        self.state = State::End;
                        return PResult::End;
                    }
                    _ => {
                        return error(&mut self.log(), &characters, "escape_sequence_p.rs.212.");
                    }
                }
            }
            State::UnicodeDigits => {
                let p = self.positional_numeral_string_p.as_mut().unwrap();
                match p.parse(characters) {
                    PResult::End => {
                        // Filled.
                        // 満ちたなら。
                        let string_buffer = tokens_stringify(&p.flush());
                        // println!("[trace157={}]", string_buffer);
                        let hex = match u32::from_str_radix(&string_buffer, 16) {
                            Ok(n) => n,
                            Err(why) => panic!("{}", why),
                        };
                        self.buffer.push(Token::new(
                            character0.column_number,
                            &from_u32(hex).unwrap().to_string(),
                            TokenType::EscapeSequence, // TODO EscapeSequence
                        ));
                        self.state = State::End;
                        self.positional_numeral_string_p = None;
                        return PResult::End;
                    }
                    PResult::Err(mut table) => {
                        return error_via(
                            &mut table,
                            &mut self.log(),
                            &characters,
                            "escape_sequence_p.rs.165.",
                        );
                    }
                    PResult::Ongoing => {}
                }
            }
        }

        PResult::Ongoing
    }

    /// Log.  
    /// ログ。  
    pub fn log(&self) -> Table {
        let mut t = Table::default().clone();

        let mut buf = String::new();
        for token in &self.buffer {
            buf.push_str(&token.to_string());
        }

        t.str("value", &buf);
        t
    }
}
