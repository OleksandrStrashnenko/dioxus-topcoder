c# Testing Documentation

This directory contains tests for the Dioxus Topcoder application. The tests are organized by module and focus on testing the core functionality of the application.

## Test Structure

- `src/tests.rs`: Main test module that includes all test modules
- `src/components/tests/`: Tests for UI components
- `src/translate/tests/`: Tests for translation functionality

## Running Tests

To run all tests:

```bash
cargo test
```

To run a specific test:

```bash
cargo test test_name
```

For example:

```bash
cargo test test_build_rpc_request
```

To run tests with output:

```bash
cargo test -- --nocapture
```

## Test Coverage

The tests cover the following functionality:

1. **Translation Module**
   - `build_rpc_request`: Tests the function that builds the request payload for Google Translate
   - `LanguageCode`: Tests the language code enum and its methods
   - `translate_from_db_or_google`: Tests the main translation function (with limitations due to external dependencies)

2. **Components**
   - `HistoryItem`: Tests the history item struct and its methods

## Adding New Tests

When adding new tests:

1. Create a new test file in the appropriate directory
2. Add the test module to `src/tests.rs`
3. Follow the existing test patterns
4. Ensure tests are isolated and don't depend on external resources when possible
5. Use mocking for external dependencies

## Mocking Strategy

For tests that involve external dependencies:

- **Database**: Currently using the existing DB thread_local, but could be improved with a test database or more sophisticated mocking
- **HTTP Requests**: Using wiremock to mock the Google Translate API

## Future Improvements

- Improve testability of the `translate` function by making the API URL configurable
- Add more component tests
- Add integration tests
- Set up a CI pipeline to run tests automatically