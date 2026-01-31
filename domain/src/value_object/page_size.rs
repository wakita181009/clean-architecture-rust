use crate::error::PageSizeError;

/// Represents a page size for pagination.
/// Must be between 1 and 100 (inclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageSize(i32);

impl PageSize {
    pub const MIN_VALUE: i32 = 1;
    pub const MAX_VALUE: i32 = 100;

    /// Creates a new PageSize with validation.
    pub fn of(value: i32) -> Result<Self, PageSizeError> {
        if value < Self::MIN_VALUE {
            return Err(PageSizeError::below_minimum(value));
        }
        if value > Self::MAX_VALUE {
            return Err(PageSizeError::above_maximum(value));
        }
        Ok(Self(value))
    }

    /// Returns the inner value.
    pub fn value(&self) -> i32 {
        self.0
    }
}

impl TryFrom<i32> for PageSize {
    type Error = PageSizeError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Self::of(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_page_size_minimum() {
        let result = PageSize::of(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), 1);
    }

    #[test]
    fn test_valid_page_size_maximum() {
        let result = PageSize::of(100);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), 100);
    }

    #[test]
    fn test_valid_page_size_middle() {
        let result = PageSize::of(50);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().value(), 50);
    }

    #[test]
    fn test_page_size_below_minimum() {
        let result = PageSize::of(0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PageSizeError::BelowMinimum { value: 0 });
    }

    #[test]
    fn test_page_size_negative() {
        let result = PageSize::of(-1);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            PageSizeError::BelowMinimum { value: -1 }
        );
    }

    #[test]
    fn test_page_size_above_maximum() {
        let result = PageSize::of(101);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            PageSizeError::AboveMaximum { value: 101 }
        );
    }

    #[test]
    fn test_page_size_equality() {
        let size1 = PageSize::of(25).unwrap();
        let size2 = PageSize::of(25).unwrap();
        assert_eq!(size1, size2);
    }
}
