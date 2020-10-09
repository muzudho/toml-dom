//! Test key value.
//! キー値テスト。
//!
//! `cargo run --example key_value`

use casual_logger::{Log, Table};
use toml_menu::{
    model::{layer20::RightValue, layer30::BroadLine},
    Toml,
};

fn main() {
    println!("Start.");
    Log::set_file_name("key-value");
    Log::remove_old_logs();
    let doc = Toml::from_file("./resource/key-value.toml");
    Log::info_t(
        "Count elements.",
        Table::default().uint("BroadLineCount", doc.broad_lines.len() as u128),
    );
    for elem in doc.broad_lines {
        match elem {
            BroadLine::ArrayOfTable(m) => {
                Log::info_t(
                    "Scan a Broad-line.",
                    Table::default().str("ArrayOfTable", &format!("{:?}", m)),
                );
            }
            BroadLine::Comment(m) => {
                Log::info_t(
                    "Scan a Broad-line.",
                    Table::default().str("Comment", &format!("{:?}", m)),
                );
            }
            BroadLine::EmptyLine => {
                Log::info_t("Scan a Broad-line.", Table::default().str("EmptyLine", ""));
            }
            BroadLine::KeyValue(m) => {
                Log::info_t(
                    "Scan a Broad-line.",
                    Table::default().str("KeyValue", &format!("{:?}", m)),
                );
                match *m.value {
                    RightValue::Array(m) => Log::info(&format!("{:?}", m)),
                    RightValue::DoubleQuotedString(m) => Log::info(&format!("{:?}", m)),
                    RightValue::InlineTable(m) => Log::info(&format!("{:?}", m)),
                    RightValue::LiteralString(m) => Log::info(&format!("{:?}", m)),
                    RightValue::SingleQuotedString(m) => Log::info(&format!("{:?}", m)),
                }
            }
            BroadLine::Table(m) => {
                Log::info_t(
                    "Scan a Broad-line.",
                    Table::default().str("Table", &format!("{:?}", m)),
                );
            }
        }
    }
    Log::flush();
    println!("Finished.");
}
