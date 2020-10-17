//! Divide into words.  
//! 単語に分けます。  
use crate::model::layer110::{Token, TokenLine, TokenType};
use std::fmt;

#[derive(Debug)]
enum State {
    AlphabetCharacter,
    AlphabetString,
    First,
    NumeralString,
    /// Whitespace means tab ('\t' 0x09) or space (' ' 0x20).  
    /// ホワイトスペースは タブ ('\t', 0x09) と 半角スペース (' ' 0x20) です。  
    WhiteSpaceString,
}

/// Lexical parser.  
/// 字句解析器。  
pub struct LexicalParser {
    state: State,
    product: TokenLine,
    buf_token_type: TokenType,
    buf: String,
}
impl LexicalParser {
    pub fn new(row_number: usize) -> Self {
        LexicalParser {
            state: State::First,
            product: TokenLine::new(row_number),
            buf_token_type: TokenType::WhiteSpaceString,
            buf: String::new(),
        }
    }
    /// Flush.
    fn flush(&mut self, column_number: usize) {
        if !self.buf.is_empty() {
            self.product
                .tokens
                .push(Token::new(column_number, &self.buf, self.buf_token_type));
            self.buf.clear();
            self.state = State::First;
        }
    }
    pub fn product(&self) -> &TokenLine {
        &self.product
    }
    pub fn parse_line(&mut self, line: &str) {
        let ch_vec: Vec<char> = line.chars().collect();
        let mut column_number_of_token = 0;
        let mut j = 0;
        let mut chars: (Option<&char>, Option<&char>) = (None, None);
        for (i, ch) in ch_vec.iter().enumerate() {
            // Shift.
            // The current char is the look-ahead char, and the previous look-ahead char is the current char.
            // ずらします。
            // 現在の文字は先読み文字、前回の先読み文字は今回の文字です。
            chars.0 = chars.1;
            chars.1 = Some(ch);
            if let Some(_) = chars.0 {
                self.one_delay_loop(i - 1, chars, &mut column_number_of_token);
            }
            j = i;
        }

        // Last 1 char.
        // 最後の１文字。
        chars.0 = chars.1;
        chars.1 = None;
        if let Some(_) = chars.0 {
            self.one_delay_loop(j, chars, &mut column_number_of_token);
        }

        // Log::info("End of line.");
        self.flush(column_number_of_token);
        // Append an end of line.
        // 行末を追加します。
        self.product.tokens.push(Token::new(
            ch_vec.len(),
            "
",
            TokenType::EndOfLine,
        ));
    }
    fn one_delay_loop(
        &mut self,
        i: usize,
        chars: (Option<&char>, Option<&char>),
        column_number_of_token: &mut usize,
    ) {
        let ch0 = chars.0.unwrap();
        let column_number = i + 1;
        match self.state {
            State::AlphabetCharacter => {
                self.flush(column_number);
                self.buf.push(*ch0);
                self.state = State::First;
            }
            State::AlphabetString => {
                self.flush(column_number);
                self.buf.push(*ch0);
                if let Some(ch1) = chars.1 {
                    match ch1 {
                        'A'..='Z' | 'a'..='z' => {}
                        _ => {
                            self.state = State::First;
                        }
                    }
                }
            }
            State::First => {
                self.flush(column_number);
                self.buf.push(*ch0);
                *column_number_of_token = column_number;
                match ch0 {
                    // A ～ Z, a ～ z.
                    'A'..='Z' | 'a'..='z' => {
                        self.buf_token_type = TokenType::AlphabetString;
                        if let Some(ch1) = chars.1 {
                            match ch1 {
                                'A'..='Z' | 'a'..='z' => {
                                    self.state = State::AlphabetString;
                                }
                                _ => {}
                            }
                        }
                    }
                    // \
                    '\\' => {
                        self.buf_token_type = TokenType::Backslash;
                        if let Some(ch1) = chars.1 {
                            match ch1 {
                                'A'..='Z' | 'a'..='z' => {
                                    self.state = State::AlphabetCharacter;
                                }
                                _ => {}
                            }
                        }
                    }
                    // :
                    ':' => {
                        self.buf_token_type = TokenType::Colon;
                    }
                    // ,
                    ',' => {
                        self.buf_token_type = TokenType::Comma;
                    }
                    // .
                    '.' => {
                        self.buf_token_type = TokenType::Dot;
                    }
                    // "
                    '"' => {
                        self.buf_token_type = TokenType::DoubleQuotation;
                    }
                    // =
                    '=' => {
                        self.buf_token_type = TokenType::Equals;
                    }
                    // -
                    '-' => {
                        self.buf_token_type = TokenType::Hyphen;
                    }
                    // {
                    '{' => {
                        self.buf_token_type = TokenType::LeftCurlyBracket;
                    }
                    // [
                    '[' => {
                        self.buf_token_type = TokenType::LeftSquareBracket;
                    }
                    '0'..='9' => {
                        self.buf_token_type = TokenType::NumeralString;
                        if let Some(ch1) = chars.1 {
                            match ch1 {
                                '0'..='9' => {
                                    self.state = State::NumeralString;
                                }
                                _ => {}
                            }
                        }
                    }
                    // +
                    '+' => {
                        self.buf_token_type = TokenType::Plus;
                    }
                    // }
                    '}' => {
                        self.buf_token_type = TokenType::RightCurlyBracket;
                    }
                    // ]
                    ']' => {
                        self.buf_token_type = TokenType::RightSquareBracket;
                    }
                    // #
                    '#' => {
                        self.buf_token_type = TokenType::Sharp;
                    }
                    // '
                    '\'' => {
                        self.buf_token_type = TokenType::SingleQuotation;
                    }
                    // _
                    '_' => {
                        self.buf_token_type = TokenType::Underscore;
                    }
                    // Whitespace.
                    '\t' | ' ' => {
                        self.buf_token_type = TokenType::WhiteSpaceString;
                        if let Some(ch1) = chars.1 {
                            match ch1 {
                                '\t' | ' ' => {
                                    self.state = State::WhiteSpaceString;
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {
                        self.buf_token_type = TokenType::Unknown;
                    }
                }
            }
            State::NumeralString => {
                self.buf.push(*ch0);
                if let Some(ch1) = chars.1 {
                    match ch1 {
                        '0'..='9' => {}
                        _ => {
                            self.state = State::First;
                        }
                    }
                }
            }
            State::WhiteSpaceString => {
                self.buf.push(*ch0);
                if let Some(ch1) = chars.1 {
                    match ch1 {
                        '\t' | ' ' => {}
                        _ => {
                            self.state = State::First;
                        }
                    }
                }
            }
        }
    }
}
impl fmt::Debug for LexicalParser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.product)
    }
}
