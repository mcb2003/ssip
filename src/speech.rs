use std::{
    borrow::{Borrow, Cow},
    fmt,
    ops::Deref,
};

#[derive(Clone)]
pub struct Speech<'a>(Cow<'a, str>);

impl<'a> From<&'a str> for Speech<'a> {
    fn from(s: &'a str) -> Self {
        // str::lines() splits by either '\n' or "\r\n", we only want the latter
        let needs_replace = s.split("\r\n").any(|l| l.starts_with("."));
        if needs_replace {
            Self(Cow::Owned(s.replace("\r\n.", "\r\n..")))
        } else {
            Self(Cow::Borrowed(s))
        }
    }
}

impl AsRef<str> for Speech<'_> {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Speech<'_> {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl<'a> Deref for Speech<'a> {
    type Target = Cow<'a, str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Debug for Speech<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.0.as_ref(), f)
    }
}

impl fmt::Display for Speech<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.0.as_ref(), f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_dot() {
        // No replacements
        let speech = Speech::from("test");
        assert!(matches!(speech.0, Cow::Borrowed(_)));
        assert_eq!(speech.as_ref(), "test");
        // Replacement
        let speech = Speech::from("Hello\r\n.line\n.not a line");
        assert!(matches!(speech.0, Cow::Owned(_)));
        assert_eq!(speech.as_ref(), "Hello\r\n..line\n.not a line")
    }
}
