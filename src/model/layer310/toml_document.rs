//! Document model.  
//! ドキュメント・モデル。  

use crate::model::layer210::LiteralValue;
use crate::model::layer225::RightValue;
use crate::model::layer230::Expression::Keyval;
use crate::model::{layer230::Expression, layer310::TomlDocument};
use chrono::prelude::{DateTime, Local, Utc};
use chrono::FixedOffset;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use std::fmt;

impl Default for TomlDocument {
    fn default() -> Self {
        TomlDocument {
            elements: Vec::new(),
        }
    }
}
impl TomlDocument {
    /// Right of `left = right`.  
    /// キー・バリューの右値。  
    pub fn get_right_value_by_key(&self, key: &str) -> Option<&Expression> {
        // println!("[trace22]");
        for elem in &self.elements {
            match elem {
                Expression::HeaderOfArrayOfTable(_) => {
                    // TODO
                    // println!("[trace27]");
                }
                Expression::EmptyLine(_ws, _comment) => {
                    // println!("[trace33]");
                }
                Expression::Keyval(m) => {
                    if m.key.to_string() == key.to_string() {
                        /*
                        println!(
                            // "[trace36 Hit m.key={} key={}]",
                            m.key.to_string(),
                            key.to_string()
                        ); // In development.
                        */
                        return Some(elem);
                    } else {
                        /*
                        println!(
                            // "[trace44 Not found m.key={} key={}]",
                            m.key.to_string(),
                            key.to_string()
                        ); // In development.
                        */
                    }
                }
                Expression::HeaderOfTable(_) => {
                    // TODO
                    // println!("[trace45]");
                }
            }
        }
        None
    }

    /// Right of `left = right`.  
    /// キー・バリューの右値。  
    pub fn get_literal_string_by_key(&self, key: &str) -> Option<&LiteralValue> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*keyval.value {
                        return Some(literal_value);
                    }
                }
            }
        }
        None
    }

    /// Contains key.  
    /// キーを含むか？  
    pub fn contains_key(&self, key: &str) -> bool {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                return keyval.key.to_string() == key.to_string();
            }
        }
        false
    }

    /// Right integer of `left = 123`.  
    /// キー・バリューの右の整数値。  
    pub fn get_i128_by_key(&self, key: &str) -> Option<i128> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*keyval.value {
                        // アンダースコアは除去しないと変換できない。
                        let s = literal_value.to_string().replace("_", "");

                        // 10進数ではないかも知れない。
                        let base_number = if s.starts_with("0b") {
                            2
                        } else if s.starts_with("0o") {
                            8
                        } else if s.starts_with("0x") {
                            16
                        } else {
                            10
                        };

                        if 10 != base_number {
                            // 頭の `0x` は除去しないと変換できない。
                            let s2 = &s[2..];
                            match i128::from_str_radix(s2, base_number) {
                                Ok(n) => return Some(n),
                                Err(why) => panic!("{}", why),
                            };
                        }

                        match s.parse() {
                            Ok(n) => return Some(n),
                            Err(_why) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// Right integer of `left = 123`.  
    /// キー・バリューの右の整数値。  
    pub fn get_isize_by_key(&self, key: &str) -> Option<isize> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*keyval.value {
                        // アンダースコアは除去しないと変換できない。
                        let s = literal_value.to_string().replace("_", "");

                        // 10進数ではないかも知れない。
                        let base_number = if s.starts_with("0b") {
                            2
                        } else if s.starts_with("0o") {
                            8
                        } else if s.starts_with("0x") {
                            16
                        } else {
                            10
                        };

                        if 10 != base_number {
                            // 頭の `0x` は除去しないと変換できない。
                            let s2 = &s[2..];
                            // println!("[trace91={}]", s2);
                            match isize::from_str_radix(s2, base_number) {
                                Ok(n) => return Some(n),
                                Err(why) => panic!("{}", why),
                            };
                        }

                        match s.parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// Right integer of `left = 123`.  
    /// キー・バリューの右の整数値。  
    pub fn get_u128_by_key(&self, key: &str) -> Option<u128> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*keyval.value {
                        // アンダースコアは除去しないと変換できない。
                        let s = literal_value.to_string().replace("_", "");

                        // 10進数ではないかも知れない。
                        let base_number = if s.starts_with("0b") {
                            2
                        } else if s.starts_with("0o") {
                            8
                        } else if s.starts_with("0x") {
                            16
                        } else {
                            10
                        };

                        if 10 != base_number {
                            // 頭の `0x` は除去しないと変換できない。
                            let s2 = &s[2..];
                            // println!("[trace91={}]", s2);
                            match u128::from_str_radix(s2, base_number) {
                                Ok(n) => return Some(n),
                                Err(why) => panic!("{}", why),
                            };
                        }

                        match s.parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// Right integer of `left = 123`.  
    /// キー・バリューの右の整数値。  
    pub fn get_usize_by_key(&self, key: &str) -> Option<usize> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*keyval.value {
                        // アンダースコアは除去しないと変換できない。
                        let s = literal_value.to_string().replace("_", "");

                        // 10進数ではないかも知れない。
                        let base_number = if s.starts_with("0b") {
                            2
                        } else if s.starts_with("0o") {
                            8
                        } else if s.starts_with("0x") {
                            16
                        } else {
                            10
                        };

                        if 10 != base_number {
                            // 頭の `0x` は除去しないと変換できない。
                            let s2 = &s[2..];
                            // println!("[trace91={}]", s2);
                            match usize::from_str_radix(s2, base_number) {
                                Ok(n) => return Some(n),
                                Err(why) => panic!("{}", why),
                            };
                        }

                        match s.parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// WIP. まだ `.` をパースできていません。  
    ///
    /// Right integer of `left = 1.2`.  
    /// キー・バリューの右の整数値。  
    pub fn get_f64_by_key(&self, key: &str) -> Option<f64> {
        // println!("[trace100 get_f64_by_key={}]", key);
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            // println!("[trace84]");
            if let Keyval(keyval) = doc_elm {
                // println!("[trace86]");
                if keyval.key.to_string() == key.to_string() {
                    // println!("[trace88]");
                    if let RightValue::LiteralValue(literal_value) = &*keyval.value {
                        // println!("[trace90]");
                        let s = literal_value.to_string();

                        match s.as_str() {
                            "nan" => {
                                return Some(f64::NAN);
                            }
                            "+nan" => {
                                return Some(f64::NAN);
                            }
                            "-nan" => {
                                return Some(-f64::NAN);
                            }
                            _ => {}
                        }

                        // アンダースコアは除去しないと変換できない。
                        match s.replace("_", "").parse() {
                            Ok(n) => {
                                // println!("[trace93]");
                                return Some(n);
                            }
                            Err(_why) => {
                                // println!("[trace97={}]", why);
                                return None;
                            }
                        }
                    }
                }
            }
        }
        None
    }

    #[deprecated(
        since = "0.1.10",
        note = "Please use the tomboy_toml_dom::model::layer310::document::get_string_by_key() method instead"
    )]
    pub fn get_str_by_key(&self, _key: &str) -> Option<&str> {
        panic!("Obsoleted. Please use the tomboy_toml_dom::model::layer310::document::get_string_by_key() method instead.")
    }

    /// Right string of `left = "abc"`.  
    /// キー・バリューの右の文字列。  
    pub fn get_string_by_key(&self, key: &str) -> Option<String> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    match &*keyval.value {
                        RightValue::BasicString(basic_string) => {
                            return Some(basic_string.to_string());
                        }
                        RightValue::LiteralString(literal_string) => {
                            return Some(literal_string.to_string());
                        }
                        _ => {}
                    }
                }
            }
        }
        None
    }
    /// For this library developer.
    pub fn get_debug_string_by_key(&self, key: &str) -> String {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    match &*keyval.value {
                        RightValue::BasicString(basic_string) => {
                            return basic_string.to_debug_string();
                        }
                        RightValue::LiteralString(literal_string) => {
                            return literal_string.to_debug_string();
                        }
                        _ => {}
                    }
                }
            }
        }
        "".to_string()
    }

    /// Right boolean of `left = true`.  
    /// キー・バリューの右の論理値。  
    pub fn get_bool_by_key(&self, key: &str) -> Option<bool> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*keyval.value {
                        match literal_value.to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// DateTime. UTC.  
    /// 日付と時刻。協定世界時。  
    pub fn get_datetime_utc_by_key(&self, key: &str) -> Option<DateTime<Utc>> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*keyval.value {
                        match format!("{}", literal_value).to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// DateTime. Local.  
    /// 日付と時刻。ローカル時。  
    pub fn get_datetime_local_by_key(&self, key: &str) -> Option<DateTime<Local>> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*keyval.value {
                        match format!("{}", literal_value).to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// DateTime. Fixed offset.  
    /// 日付と時刻。オフセット。  
    pub fn get_datetime_fixed_offset_by_key(&self, key: &str) -> Option<DateTime<FixedOffset>> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*keyval.value {
                        match format!("{}", literal_value).to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// DateTime. Naive.  
    /// 日付と時刻。ナイーブ。  
    pub fn get_naive_datetime_by_key(&self, key: &str) -> Option<NaiveDateTime> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*keyval.value {
                        match format!("{}", literal_value).to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// Date. Naive.  
    /// 日付。ナイーブ。  
    pub fn get_naive_date_by_key(&self, key: &str) -> Option<NaiveDate> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*keyval.value {
                        match format!("{}", literal_value).to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    /// Time. Naive.  
    /// 日時。ナイーブ。  
    pub fn get_naive_time_by_key(&self, key: &str) -> Option<NaiveTime> {
        if let Some(doc_elm) = self.get_right_value_by_key(key) {
            if let Keyval(keyval) = doc_elm {
                if keyval.key.to_string() == key.to_string() {
                    if let RightValue::LiteralValue(literal_value) = &*keyval.value {
                        match format!("{}", literal_value).to_string().parse() {
                            Ok(n) => return Some(n),
                            Err(_) => return None,
                        }
                    }
                }
            }
        }
        None
    }

    pub fn push_element(&mut self, m: &Expression) {
        self.elements.push(m.clone());
    }
    pub fn to_debug_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
impl fmt::Display for TomlDocument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for elem in &self.elements {
            buf.push_str(&format!("{}", elem));
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for TomlDocument {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for elem in &self.elements {
            buf.push_str(&format!("{:?}", elem));
        }
        write!(f, "{}", buf)
    }
}
