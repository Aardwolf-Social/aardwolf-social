//-
// Banjo's notes:
// For the FULL list of options please refer to:
// ./aardwolf-localization/example_lib.rs

//
// Start of rust-i18n configuration
// Load I18n macro, for allow you use `t!` macro in anywhere.

#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
// This will load Configuration using the `[package.metadata.i18n]` section in `Cargo.toml` if exists.
// Or you can pass arguments by `i18n!` to override it.
i18n!("locales", fallback = "en");

// End rust-i18n configuration
