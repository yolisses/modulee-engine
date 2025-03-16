use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FixedSizeString<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> FixedSizeString<N> {
    pub fn new(s: &str) -> Result<Self, &'static str> {
        if s.len() != N {
            return Err("String length does not match fixed size");
        }

        let mut data = [0u8; N];
        data.copy_from_slice(s.as_bytes());
        Ok(Self { data })
    }

    pub fn as_str(&self) -> &str {
        // SAFETY: We ensure during construction that `data` is valid UTF-8.
        unsafe { std::str::from_utf8_unchecked(&self.data) }
    }
}

impl<const N: usize> fmt::Display for FixedSizeString<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const N: usize> FromStr for FixedSizeString<N> {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_size_string_creation() {
        // Test valid creation
        let id: FixedSizeString<5> = "12345".parse().unwrap();
        assert_eq!(id.as_str(), "12345");

        // Test Display implementation
        assert_eq!(format!("{}", id), "12345");

        // Test invalid creation (wrong length)
        let result: Result<FixedSizeString<5>, _> = "1234".parse();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "String length does not match fixed size"
        );

        // Test invalid creation (too long)
        let result: Result<FixedSizeString<5>, _> = "123456".parse();
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "String length does not match fixed size"
        );
    }

    #[test]
    fn test_fixed_size_string_equality() {
        let id1: FixedSizeString<3> = "abc".parse().unwrap();
        let id2: FixedSizeString<3> = "abc".parse().unwrap();
        let id3: FixedSizeString<3> = "def".parse().unwrap();

        assert_eq!(id1, id2); // Same content
        assert_ne!(id1, id3); // Different content
    }

    #[test]
    fn test_fixed_size_string_clone_and_copy() {
        let id1: FixedSizeString<4> = "test".parse().unwrap();
        let id2 = id1; // Copy happens here
        let id3 = id1.clone(); // Clone happens here

        assert_eq!(id1, id2);
        assert_eq!(id1, id3);
    }
}
