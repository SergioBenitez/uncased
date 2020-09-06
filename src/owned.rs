use alloc::borrow::{Cow, Borrow};
use alloc::{string::String, boxed::Box};

use core::ops::Deref;
use core::cmp::Ordering;
use core::hash::{Hash, Hasher};
use core::fmt;

use crate::UncasedStr;

/// An uncased (case-preserving), owned _or_ borrowed ASCII string.
#[cfg_attr(nightly, doc(cfg(feature = "alloc")))]
#[derive(Clone, Debug)]
pub struct Uncased<'s> {
    #[doc(hidden)]
    pub string: Cow<'s, str>
}

impl<'s> Uncased<'s> {
    /// Creates a new `Uncased` string from `string` without allocating.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncased::Uncased;
    ///
    /// let uncased = Uncased::new("Content-Type");
    /// assert_eq!(uncased, "content-type");
    /// assert_eq!(uncased, "CONTENT-Type");
    /// ```
    #[inline(always)]
    pub fn new<S: Into<Cow<'s, str>>>(string: S) -> Uncased<'s> {
        Uncased { string: string.into() }
    }

    /// Creates a new `Uncased` string from a borrowed `string`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncased::Uncased;
    ///
    /// const UNCASED: Uncased = Uncased::from_borrowed("Content-Type");
    /// assert_eq!(UNCASED, "content-type");
    /// assert_eq!(UNCASED, "CONTENT-Type");
    /// ```
    #[inline(always)]
    pub const fn from_borrowed(string: &'s str) -> Uncased<'s> {
        Uncased { string: Cow::Borrowed(string) }
    }

    /// Creates a new `Uncased` string from `string` without allocating.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncased::Uncased;
    ///
    /// const UNCASED: Uncased = Uncased::from_owned(String::new());
    ///
    /// let uncased = Uncased::from_owned("Content-Type".to_string());
    /// assert_eq!(uncased, "content-type");
    /// assert_eq!(uncased, "CONTENT-Type");
    /// ```
    #[inline(always)]
    pub const fn from_owned(string: String) -> Uncased<'s> {
        Uncased { string: Cow::Owned(string) }
    }

    /// Converts `self` into an owned `String`, allocating if necessary.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncased::Uncased;
    ///
    /// let uncased = Uncased::new("Content-Type");
    /// let string = uncased.into_string();
    /// assert_eq!(string, "Content-Type".to_string());
    /// ```
    #[inline(always)]
    pub fn into_string(self) -> String {
        self.string.into_owned()
    }

    /// Converts `self` into a `Box<UncasedStr>`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use uncased::Uncased;
    ///
    /// let boxed = Uncased::new("Content-Type").into_boxed_uncased();
    /// assert_eq!(&*boxed, "content-type");
    /// ```
    #[inline(always)]
    pub fn into_boxed_uncased(self) -> Box<UncasedStr> {
        // This is simply a `newtype`-like transformation. The `repr(C)` ensures
        // that this is safe and correct. Note this exact pattern appears often
        // in the standard library.
        unsafe {
            let raw_str = Box::into_raw(self.string.into_owned().into_boxed_str());
            Box::from_raw(raw_str as *mut UncasedStr)
        }
    }

    /// Returns the inner `Cow`.
    #[doc(hidden)]
    #[inline(always)]
    pub fn into_cow(self) -> Cow<'s, str> {
        self.string
    }
}

impl Deref for Uncased<'_> {
    type Target = UncasedStr;

    #[inline(always)]
    fn deref(&self) -> &UncasedStr {
        UncasedStr::new(self.string.borrow())
    }
}

impl AsRef<UncasedStr> for Uncased<'_> {
    #[inline(always)]
    fn as_ref(&self) -> &UncasedStr {
        UncasedStr::new(self.string.borrow())
    }
}

impl Borrow<UncasedStr> for Uncased<'_> {
    #[inline(always)]
    fn borrow(&self) -> &UncasedStr {
        self.as_str().into()
    }
}

impl<'s, 'c: 's> From<&'c str> for Uncased<'s> {
    #[inline(always)]
    fn from(string: &'c str) -> Self {
        Uncased::new(string)
    }
}

impl From<String> for Uncased<'static> {
    #[inline(always)]
    fn from(string: String) -> Self {
        Uncased::new(string)
    }
}

impl<'s, 'c: 's> From<Cow<'c, str>> for Uncased<'s> {
    #[inline(always)]
    fn from(string: Cow<'c, str>) -> Self {
        Uncased::new(string)
    }
}

impl<'b> PartialOrd<Uncased<'b>> for Uncased<'_> {
    #[inline(always)]
    fn partial_cmp(&self, other: &Uncased<'b>) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl Ord for Uncased<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl fmt::Display for Uncased<'_> {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.string.fmt(f)
    }
}

impl<'b> PartialEq<Uncased<'b>> for Uncased<'_> {
    #[inline(always)]
    fn eq(&self, other: &Uncased<'b>) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl PartialEq<str> for Uncased<'_> {
    #[inline(always)]
    fn eq(&self, other: &str) -> bool {
        self.as_ref().eq(other)
    }
}

impl PartialEq<Uncased<'_>> for str {
    #[inline(always)]
    fn eq(&self, other: &Uncased<'_>) -> bool {
        other.as_ref().eq(self)
    }
}

impl<'b> PartialEq<&'b str> for Uncased<'_> {
    #[inline(always)]
    fn eq(&self, other: & &'b str) -> bool {
        self.as_ref().eq(other)
    }
}

impl<'b> PartialEq<Uncased<'b>> for &str {
    #[inline(always)]
    fn eq(&self, other: &Uncased<'b>) -> bool {
        other.as_ref().eq(self)
    }
}

impl Eq for Uncased<'_> {  }

impl Hash for Uncased<'_> {
    #[inline(always)]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.as_ref().hash(hasher)
    }
}
