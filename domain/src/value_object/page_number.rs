use crate::error::PageNumberError;

/// Represents a page number for pagination.
/// Must be at least 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageNumber(i32);

impl PageNumber {
    pub const MIN_VALUE: i32 = 1;

    /// Creates a new PageNumber with validation.
    pub fn of(value: i32) -> Result<Self, PageNumberError> {
        if value < Self::MIN_VALUE {
            return Err(PageNumberError::below_minimum(value));
        }
        Ok(Self(value))
    }

    /// Returns the inner value.
    pub fn value(&self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for PageNumber {
    type Error = PageNumberError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::of(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_page_number() {
        let result = PageNumber::of(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), 1);
    }

    #[test]
    fn test_page_number_above_minimum() {
        let result = PageNumber::of(100);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), 100);
    }

    #[test]
    fn test_page_number_below_minimum() {
        let result = PageNumber::of(0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            PageNumberError::BelowMinimum { value: 0 }
        );
    }

    #[test]
    fn test_page_number_negative() {
        let result = PageNumber::of(-1);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            PageNumberError::BelowMinimum { value: -1 }
        );
    }

    #[test]
    fn test_page_number_equality() {
        let page1 = PageNumber::of(5).unwrap();
        let page2 = PageNumber::of(5).unwrap();
        assert_eq!(page1, page2);
    }
}
