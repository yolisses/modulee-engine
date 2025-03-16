use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt;
use std::marker::PhantomData;
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

// Implement Deserialize for FixedSizeString
impl<'de, const N: usize> Deserialize<'de> for FixedSizeString<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Define a visitor to handle the deserialization
        struct FixedSizeStringVisitor<const N: usize>;

        impl<'de, const N: usize> Visitor<'de> for FixedSizeStringVisitor<N> {
            type Value = FixedSizeString<N>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a string of length {}", N)
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                FixedSizeString::new(value)
                    .map_err(|_| E::custom(format!("String length must be exactly {}", N)))
            }
        }

        // Use the visitor to deserialize the string
        deserializer.deserialize_str(FixedSizeStringVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct<const N: usize> {
        id: FixedSizeString<N>,
    }

    #[test]
    fn test_deserialize_valid() {
        let json_data = r#"{"id": "12345"}"#;
        let expected = TestStruct {
            id: FixedSizeString::new("12345").unwrap(),
        };

        let result: TestStruct<5> = serde_json::from_str(json_data).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_deserialize_invalid_length() {
        let json_data = r#"{"id": "1234"}"#;
        let result: Result<TestStruct<5>, _> = serde_json::from_str(json_data);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("String length must be exactly 5"));
    }

    #[test]
    fn test_deserialize_non_string() {
        let json_data = r#"{"id": 12345}"#;
        let result: Result<TestStruct<5>, _> = serde_json::from_str(json_data);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("invalid type: integer"));
    }
}
