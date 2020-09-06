//! Case-preserving, ASCII case-insensitive `no_std` string types.
//!
//! An _uncased_ string is case-preserving. That is, the string itself contains
//! cased characters, but comparison (including ordering, equality, and hashing)
//! is ASCII case-insensitive.
//!
//! ```rust
//! use uncased::UncasedStr;
//!
//! let x: &UncasedStr = "hello!".into();
//! let y: &UncasedStr = "HelLo!".into();
//!
//! assert_eq!(x, y);
//! assert_eq!(x.as_str(), "hello!");
//! assert_eq!(y.as_str(), "HelLo!");
//! ```
//!
//! ## Unicode
//!
//! This crate _does not_ perform Unicode case-folding. For Unicode
//! case-folding, see [`unicase`](https://crates.io/crates/unicase).
//!
//! ## Features and `no_std`
//!
//! This crate is `#![no_std]` compatible. By default, the `alloc` feature is
//! enabled, which enables the [`Uncased`] type but requires `alloc` support. To
//! disable the feature, disable this crate's default features:
//!
//! ```toml
//! [dependencies]
//! uncased = { version = "0.9", default-features = false }
//! ```

#![no_std]

#![cfg_attr(nightly, feature(doc_cfg))]

#[cfg(feature = "alloc")] extern crate alloc;

mod borrowed;
pub use borrowed::UncasedStr;

#[cfg(feature = "alloc")] mod owned;
#[cfg(feature = "alloc")] pub use owned::Uncased;

#[cfg(test)] mod tests;

/// Returns true if `s1` and `s2` are equal without considering case.
///
/// That is, this function returns `s1.to_ascii_lowercase() ==
/// s2.to_ascii_lowercase()`, but does it in a much faster way. This is also
/// equivalent to `UncasedStr::new(s1) == UncasedStr::new(s2)`.
///
/// # Example
///
/// ```rust
/// assert!(uncased::eq("ENV", "env"));
/// assert!(uncased::eq("bRoWN", "BROWN"));
/// assert!(uncased::eq("hi", "HI"));
/// assert!(uncased::eq("dogs are COOL!", "DOGS are cool!"));
/// ```
#[inline(always)]
pub fn eq<S1: AsRef<str>, S2: AsRef<str>>(s1: S1, s2: S2) -> bool {
    UncasedStr::new(s1.as_ref()) == UncasedStr::new(s2.as_ref())
}
