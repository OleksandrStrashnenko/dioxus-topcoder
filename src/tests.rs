//! Main test module that includes all test modules

#[cfg(test)]
mod tests {
    // Include component tests
    #[path = "components/tests/mod.rs"]
    mod components_tests;
    
    // Include translate tests
    #[path = "translate/tests/mod.rs"]
    mod translate_tests;
    
    #[path = "translate/tests/language_codes_tests.rs"]
    mod language_codes_tests;
    
    #[path = "translate/tests/translate_from_db_tests.rs"]
    mod translate_from_db_tests;
}