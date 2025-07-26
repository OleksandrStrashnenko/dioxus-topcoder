//! Tests for the language_codes module

#[cfg(test)]
mod language_codes_tests {
    use crate::translate::language_codes::LanguageCode;
    use rstest::rstest;

    #[rstest]
    #[case(LanguageCode::English, "en")]
    #[case(LanguageCode::Ukrainian, "uk")]
    #[case(LanguageCode::German, "de")]
    #[case(LanguageCode::French, "fr")]
    #[case(LanguageCode::Spanish, "es")]
    #[case(LanguageCode::Japanese, "ja")]
    #[case(LanguageCode::ChineseSimplified, "zh-CN")]
    #[case(LanguageCode::ChineseTraditional, "zh-TW")]
    #[case(LanguageCode::Russian, "ru")]
    #[case(LanguageCode::Arabic, "ar")]
    fn test_get_code(#[case] language: LanguageCode, #[case] expected_code: &str) {
        assert_eq!(language.get_code(), expected_code);
    }

    #[test]
    fn test_values_returns_all_languages() {
        // This test ensures that the values() function returns all enum variants
        let values = LanguageCode::values();

        // Check that we have a reasonable number of languages
        // The exact count might change if languages are added/removed
        assert!(values.len() > 100, "Expected more than 100 languages, got {}", values.len());

        // Check that some key languages are included
        assert!(values.contains(&LanguageCode::English));
        assert!(values.contains(&LanguageCode::Spanish));
        assert!(values.contains(&LanguageCode::ChineseSimplified));
        assert!(values.contains(&LanguageCode::Russian));
    }
}
