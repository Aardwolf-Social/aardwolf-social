//
// Start of rust-i18n configuration
// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
// This will load Configuration using the `[package.metadata.i18n]` section in `Cargo.toml` if exists.
// Or you can pass arguments by `i18n!` to override it.
i18n!("locales");

// Config fallback missing translations to "en" locale.
// Use `fallback` option to set fallback locale.
//
i18n!("locales", fallback = "en");

// Or more than one fallback with priority.
//
i18n!("locales", fallback = ["en", "es"]);

// Use a short hashed key as an identifier for long string literals
// to optimize memory usage and lookup speed.
// The key generation algorithm is `${Prefix}${Base62(SipHash13("msg"))}`.
i18n!("locales", minify_key = true);
//
// Alternatively, you can customize the key length, prefix,
// and threshold for the short hashed key.
i18n!("locales",
      minify_key = true,
      minify_key_len = 12,
      minify_key_prefix = "t_",
      minify_key_thresh = 64
);
// Now, if the message length exceeds 64, the `t!` macro will automatically generate
// a 12-byte short hashed key with a "t_" prefix for it, if not, it will use the original.

// If no any argument, use config from Cargo.toml or default.
i18n!();

// End rust-i18n configuration
//
