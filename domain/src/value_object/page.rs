/// Generic container for paginated results.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Page<T> {
    pub total_count: i32,
    pub items: Vec<T>,
}

impl<T> Page<T> {
    pub fn new(total_count: i32, items: Vec<T>) -> Self {
        Self { total_count, items }
    }

    pub fn empty() -> Self {
        Self {
            total_count: 0,
            items: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }
}

impl<T> Default for Page<T> {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_creation() {
        let page = Page::new(100, vec![1, 2, 3, 4, 5]);
        assert_eq!(page.total_count, 100);
        assert_eq!(page.items.len(), 5);
    }

    #[test]
    fn test_page_empty() {
        let page: Page<i32> = Page::empty();
        assert_eq!(page.total_count, 0);
        assert!(page.is_empty());
    }

    #[test]
    fn test_page_default() {
        let page: Page<String> = Page::default();
        assert_eq!(page.total_count, 0);
        assert!(page.is_empty());
    }
}
