use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::fmt;

/// A cost-free reference to an uncased (case-insensitive, case-preserving)
/// ASCII string.
///
/// This is typically created from an `&str` as follows:
///
/// ```rust
/// use uncased::UncasedStr;
///
/// let ascii_ref: &UncasedStr = "Hello, world!".into();
/// ```
#[derive(Debug)]
#[repr(transparent)]
pub struct UncasedStr(str);

impl UncasedStr {
    /// Cost-free conversion from an `&str` reference to an `UncasedStr`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncased::UncasedStr;
    ///
    /// let uncased_str = UncasedStr::new("Hello!");
    /// assert_eq!(uncased_str, "hello!");
    /// assert_eq!(uncased_str, "Hello!");
    /// assert_eq!(uncased_str, "HeLLo!");
    /// ```
    #[inline(always)]
    pub fn new(string: &str) -> &UncasedStr {
        // This is a `newtype`-like transformation. `repr(transparent)` ensures
        // that this is safe and correct.
        unsafe { &*(string as *const str as *const UncasedStr) }
    }

    /// Returns `self` as an `&str`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncased::UncasedStr;
    ///
    /// let uncased_str = UncasedStr::new("Hello!");
    /// assert_eq!(uncased_str.as_str(), "Hello!");
    /// assert_ne!(uncased_str.as_str(), "hELLo!");
    /// ```
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the length, in bytes, of `self`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncased::UncasedStr;
    ///
    /// let uncased_str = UncasedStr::new("Hello!");
    /// assert_eq!(uncased_str.len(), 6);
    /// ```
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.as_str().len()
    }

    /// Returns `true` if `self` starts with any casing of the string `string`;
    /// otherwise, returns `false`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncased::UncasedStr;
    ///
    /// let uncased_str = UncasedStr::new("MoOO");
    /// assert!(uncased_str.starts_with("moo"));
    /// assert!(uncased_str.starts_with("MOO"));
    /// assert!(uncased_str.starts_with("MOOO"));
    /// assert!(!uncased_str.starts_with("boo"));
    /// ```
    #[inline(always)]
    pub fn starts_with(&self, string: &str) -> bool {
        self.len() >= string.len() && self[..string.len()] == string
    }

    /// Converts a `Box<UncasedStr>` into an `Uncased` without copying or
    /// allocating.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncased::Uncased;
    ///
    /// let uncased = Uncased::new("Hello!");
    /// let boxed = uncased.clone().into_boxed_uncased();
    /// assert_eq!(boxed.into_uncased(), uncased);
    /// ```
    #[inline(always)]
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly, doc(cfg(feature = "alloc")))]
    pub fn into_uncased(self: alloc::boxed::Box<UncasedStr>) -> crate::Uncased<'static> {
        // This is the inverse of a `newtype`-like transformation. The
        // `repr(transparent)` ensures that this is safe and correct.
        unsafe {
            let raw_str = alloc::boxed::Box::into_raw(self) as *mut str;
            crate::Uncased::from(alloc::boxed::Box::from_raw(raw_str).into_string())
        }
    }
}

impl<I: core::slice::SliceIndex<str, Output=str>> core::ops::Index<I> for UncasedStr {
    type Output = UncasedStr;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.as_str()[index].into()
    }
}

impl PartialEq for UncasedStr {
    #[inline(always)]
    fn eq(&self, other: &UncasedStr) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl PartialEq<str> for UncasedStr {
    #[inline(always)]
    fn eq(&self, other: &str) -> bool {
        self.0.eq_ignore_ascii_case(other)
    }
}

impl PartialEq<UncasedStr> for str {
    #[inline(always)]
    fn eq(&self, other: &UncasedStr) -> bool {
        other.0.eq_ignore_ascii_case(self)
    }
}

impl PartialEq<&str> for UncasedStr {
    #[inline(always)]
    fn eq(&self, other: &&str) -> bool {
        self.0.eq_ignore_ascii_case(other)
    }
}

impl PartialEq<UncasedStr> for &str {
    #[inline(always)]
    fn eq(&self, other: &UncasedStr) -> bool {
        other.0.eq_ignore_ascii_case(self)
    }
}

impl<'a> From<&'a str> for &'a UncasedStr {
    #[inline(always)]
    fn from(string: &'a str) -> &'a UncasedStr {
        UncasedStr::new(string)
    }
}

impl Eq for UncasedStr {  }

impl Hash for UncasedStr {
    #[inline(always)]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.0.bytes().for_each(|b| hasher.write_u8(b.to_ascii_lowercase()));
    }
}

impl PartialOrd for UncasedStr {
    #[inline(always)]
    fn partial_cmp(&self, other: &UncasedStr) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UncasedStr {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_chars = self.0.chars().map(|c| c.to_ascii_lowercase());
        let other_chars = other.0.chars().map(|c| c.to_ascii_lowercase());
        self_chars.cmp(other_chars)
    }
}

impl fmt::Display for UncasedStr {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
