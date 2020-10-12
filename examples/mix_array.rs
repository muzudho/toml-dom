//! Test.
//! テスト。
//!
//! `cargo run --example mix_array`

extern crate tomboy_toml_dom;

mod modules;

use crate::modules::log_ext::LogExt;
use casual_logger::{Level, Log, Table};
use tomboy_toml_dom::Toml;

fn main() {
    // Configuration a log.
    Log::set_file_name("exa-test-mix-array");
    Log::set_level(Level::Debug);
    Log::set_retention_days(-1);
    Log::remove_old_logs();

    // Read a Toml file.
    let toml_file = "./resource/mix-array.toml";
    let doc = Toml::from_file(toml_file);

    let mut has_error = false;

    // Test.
    let key = "mix_array_3";
    if let Some(elem) = doc.get_right_value_by_key(key) {
        Log::info_t("Test.1.", Table::default().str(key, &format!("{:?}", elem)));
    } else {
        has_error = true;
        Log::error_t("Test.1.", Table::default().str(key, ""));
    }

    if has_error {
        Log::info_toml_document(toml_file, &doc);
    }

    Log::flush();
}
