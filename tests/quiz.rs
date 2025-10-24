#[cfg(test)]
mod tests {
    use learn_dioxus::quiz_service::quiz_service::QuizServiceImpl;
    use std::sync::Arc;
    use dioxus::prelude::Signal;

    fn create_test_quiz() -> QuizServiceImpl {
        let cards = vec![
            ("What is 2+2?".to_string(), "4".to_string()),
            ("Capital of France?".to_string(), "Paris".to_string()),
            ("Color of sky?".to_string(), "blue".to_string()),
        ];
        QuizServiceImpl::from_cards(Signal::new(Arc::new(cards)))
    }

    #[test]
    fn test_passing_quiz_with_correct_answers() {
        let mut quiz = create_test_quiz();

        // First question
        assert_eq!(quiz.current_question(), "What is 2+2?");
        assert!(!quiz.quiz_is_over());
        quiz.check_for_answer("4");

        // Second question
        assert_eq!(quiz.current_question(), "Capital of France?");
        assert!(!quiz.quiz_is_over());
        quiz.check_for_answer("Paris");

        // Third question
        assert_eq!(quiz.current_question(), "Color of sky?");
        assert!(!quiz.quiz_is_over());
        quiz.check_for_answer("blue");

        // Quiz should be over
        assert!(quiz.quiz_is_over());
        assert_eq!(quiz.current_question(), "No questions left");

        let results = quiz.get_results();
        assert_eq!(
            results,
            vec![
                ("What is 2+2?".to_string(), "4".to_string(), true),
                ("Capital of France?".to_string(), "Paris".to_string(), true),
                ("Color of sky?".to_string(), "blue".to_string(), true)
            ]
        )
    }

    #[test]
    fn test_wrong_answers_dont_advance() {
        let mut quiz = create_test_quiz();

        assert_eq!(quiz.current_question(), "What is 2+2?");
        quiz.check_for_answer("5"); // wrong answer
        assert_eq!(quiz.current_question(), "What is 2+2?"); // should still be on first question
    }

    #[test]
    fn test_skip_functionality() {
        let mut quiz = create_test_quiz();

        assert_eq!(quiz.current_question(), "What is 2+2?");
        quiz.skip();
        assert_eq!(quiz.current_question(), "Capital of France?");
        quiz.skip();
        assert_eq!(quiz.current_question(), "Color of sky?");
        quiz.skip();
        assert!(quiz.quiz_is_over());
    }

    #[test]
    fn test_quiz_equality() {
        let quiz1 = create_test_quiz();
        let quiz2 = create_test_quiz();
        assert_eq!(quiz1, quiz2);

        let mut quiz3 = create_test_quiz();
        quiz3.skip();
        assert_ne!(quiz1, quiz3);
    }
}
