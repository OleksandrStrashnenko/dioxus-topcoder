//! Tests for the components module

#[cfg(test)]
mod history_tests {
    use crate::components::history::HistoryItem;

    #[test]
    fn test_history_item_new() {
        let src = "Hello".to_string();
        let translated = "Привіт".to_string();
        
        let item = HistoryItem::new(src.clone(), translated.clone());
        
        assert_eq!(item.src(), &src);
        assert_eq!(item.translated(), &translated);
    }
    
    #[test]
    fn test_history_item_getters() {
        let src = "Good morning".to_string();
        let translated = "Доброго ранку".to_string();
        
        let item = HistoryItem::new(src.clone(), translated.clone());
        
        // Test src() method
        let retrieved_src = item.src();
        assert_eq!(retrieved_src, &src);
        
        // Test translated() method
        let retrieved_translated = item.translated();
        assert_eq!(retrieved_translated, &translated);
    }
    
    #[test]
    fn test_history_item_with_empty_strings() {
        let src = "".to_string();
        let translated = "".to_string();
        
        let item = HistoryItem::new(src, translated);
        
        assert_eq!(item.src(), "");
        assert_eq!(item.translated(), "");
    }
    
    #[test]
    fn test_history_item_with_special_characters() {
        let src = "Hello, world! 123".to_string();
        let translated = "Привіт, світ! 123".to_string();
        
        let item = HistoryItem::new(src.clone(), translated.clone());
        
        assert_eq!(item.src(), &src);
        assert_eq!(item.translated(), &translated);
    }
}