//-
// Banjo's notes:
// For the FULL list of options please refer to:
// ./aardwolf-localization/example_lib.rs

//
// Start of rust-i18n configuration
// Load I18n macro, for allow you use `t!` macro in anywhere.

#[macro_use]
extern crate rust_i18n;

i18n!("locales", fallback = "en"); // This must be at the crate root
