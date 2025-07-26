//! Tests for the translate module

#[cfg(test)]
mod translate_tests {
    use crate::translate::translate::build_rpc_request;
    use serde_json::json;

    #[test]
    fn test_build_rpc_request() {
        // Test with simple text
        let text = "Hello";
        let dest = "uk";
        let src = "auto";
        let result = build_rpc_request(text, dest, src);
        
        // Expected format based on the implementation
        let expected = json!([[
            [
                "MkEWBc",
                format!("[[{text}, {src}, {dest}, true], [null]]").as_str(),
                "null",
                "generic"
            ]
        ]]).to_string();
        
        assert_eq!(result, expected);
        
        // Test with text containing special characters
        let text = "Hello, world! How are you?";
        let result = build_rpc_request(text, dest, src);
        
        let expected = json!([[
            [
                "MkEWBc",
                format!("[[{text}, {src}, {dest}, true], [null]]").as_str(),
                "null",
                "generic"
            ]
        ]]).to_string();
        
        assert_eq!(result, expected);
    }
}