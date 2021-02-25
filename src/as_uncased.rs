use crate::UncasedStr;

/// Helper trait to convert string-like references to `&UncasedStr`.
pub trait AsUncased {
    fn as_uncased(&self) -> &UncasedStr;
}

impl<T: AsRef<str> + ?Sized> AsUncased for T {
    #[inline(always)]
    fn as_uncased(&self) -> &UncasedStr {
        UncasedStr::new(self.as_ref())
    }
}
