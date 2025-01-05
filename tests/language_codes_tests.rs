use learn_dioxus::translate::language_codes::LanguageCode;
use strum::IntoEnumIterator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_code_values() {
        // Test that values() returns a non-empty vector
        let values = LanguageCode::values();
        assert!(!values.is_empty());

        // Test that the vector contains expected languages
        assert!(values.contains(&LanguageCode::English));
        assert!(values.contains(&LanguageCode::Spanish));
        assert!(values.contains(&LanguageCode::Russian));
        assert!(values.contains(&LanguageCode::Ukrainian));
        assert!(values.contains(&LanguageCode::Japanese));
    }

    #[test]
    fn test_language_code_get_code() {
        // Test a sample of language codes to ensure they return the correct ISO code
        assert_eq!(LanguageCode::English.get_code(), "en");
        assert_eq!(LanguageCode::Spanish.get_code(), "es");
        assert_eq!(LanguageCode::Russian.get_code(), "ru");
        assert_eq!(LanguageCode::Ukrainian.get_code(), "uk");
        assert_eq!(LanguageCode::Japanese.get_code(), "ja");
        assert_eq!(LanguageCode::ChineseSimplified.get_code(), "zh-CN");
        assert_eq!(LanguageCode::ChineseTraditional.get_code(), "zh-TW");
        assert_eq!(LanguageCode::German.get_code(), "de");
        assert_eq!(LanguageCode::French.get_code(), "fr");
        assert_eq!(LanguageCode::Arabic.get_code(), "ar");
    }

    #[test]
    fn test_enum_iter_implementation() {
        // Test that the EnumIter implementation works correctly
        let iter_values: Vec<LanguageCode> = LanguageCode::iter().collect();
        let values = LanguageCode::values();

        // Both methods should return the same values
        assert_eq!(iter_values, values);

        // Check that the iterator contains all expected values
        assert!(iter_values.contains(&LanguageCode::English));
        assert!(iter_values.contains(&LanguageCode::Spanish));
        assert!(iter_values.contains(&LanguageCode::Russian));
        // Add more assertions as needed
    }

    #[test]
    fn test_language_count() {
        // Test that the number of languages is as expected
        let values = LanguageCode::values();

        // This should be updated if languages are added or removed
        assert_eq!(values.len(), 193);
    }

    #[test]
    fn test_regional_language_codes() {
        // Test languages with region codes
        assert_eq!(LanguageCode::ChineseSimplified.get_code(), "zh-CN");
        assert_eq!(LanguageCode::ChineseTraditional.get_code(), "zh-TW");
        assert_eq!(LanguageCode::PortuguesePortugal.get_code(), "pt-PT");
        assert_eq!(LanguageCode::PortugueseBrazil.get_code(), "pt-BR");
        assert_eq!(LanguageCode::FrenchCanadian.get_code(), "fr-CA");
        assert_eq!(LanguageCode::MalayJawi.get_code(), "ms-Arab");
        assert_eq!(LanguageCode::PunjabiShahmukhi.get_code(), "pa-Arab");
        assert_eq!(LanguageCode::MeiteilonManipuri.get_code(), "mni-Mtei");
    }

    #[test]
    fn test_all_codes_are_valid() {
        // Test that all language codes are non-empty strings
        for language in LanguageCode::values() {
            let code = language.get_code();
            assert!(!code.is_empty(), "Language code for {:?} is empty", language);

            // Check that codes follow expected format (letters, hyphens, or BCP-47 format)
            assert!(code.chars().all(|c| c.is_ascii_alphabetic() || c == '-'), 
                "Language code '{}' for {:?} contains invalid characters", code, language);
        }
    }

    #[test]
    fn test_code_uniqueness() {
        // Test that all language codes are unique
        let values = LanguageCode::values();
        let mut codes: Vec<&str> = values.iter().map(|lang| lang.get_code()).collect();

        let original_len = codes.len();
        codes.sort();
        codes.dedup();

        assert_eq!(codes.len(), original_len, "Duplicate language codes found");
    }
}
