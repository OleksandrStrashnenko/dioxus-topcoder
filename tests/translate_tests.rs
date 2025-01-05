use learn_dioxus::translate::translate::build_rpc_request;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_rpc_request() {
        // Test with simple inputs
        let text = "Hello";
        let dest = "uk";
        let src = "auto";

        let result = build_rpc_request(text, dest, src);

        // The result should be a JSON string containing the input parameters
        assert!(result.contains("Hello"));
        assert!(result.contains("uk"));
        assert!(result.contains("auto"));
        assert!(result.contains("MkEWBc"));

        // Test with text containing special characters
        let text = "Hello, world! 123 @#$%";
        let result = build_rpc_request(text, dest, src);

        assert!(result.contains("Hello, world! 123 @#$%"));
    }
}
