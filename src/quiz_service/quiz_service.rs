use dioxus::prelude::Signal;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;

#[derive(Clone, Debug)]
pub struct QuizServiceImpl {
    index: Arc<AtomicUsize>,
    cards: Signal<Arc<Vec<(String, String)>>>,
    question_answers: Arc<Vec<bool>>,
    thread: Arc<RwLock<Option<JoinHandle<()>>>>,
}

impl PartialEq for QuizServiceImpl {
    fn eq(&self, other: &QuizServiceImpl) -> bool {
        self.index.load(Ordering::SeqCst) == other.index.load(Ordering::SeqCst)
    }
}

impl QuizServiceImpl {
    pub fn current_question(&self) -> String {
        if self.index.load(Ordering::Acquire) >= (self.cards)().len() {
            return "No questions left".to_string();
        }
        format!("{}", (self.cards)()[self.index.load(Ordering::Acquire)].0)
    }

    pub fn from_cards(cards: Signal<Arc<Vec<(String, String)>>>) -> Self {
        QuizServiceImpl {
            index: Arc::new(AtomicUsize::new(0)),
            cards,
            question_answers: Arc::new(vec![]),
            thread: Arc::new(None.into()),
        }
    }

    pub fn check_for_answer(&mut self, answer: &str) -> bool {
        if self.index.load(Ordering::Acquire) >= (self.cards)().len() {
            return false;
        }
        if (self.cards)()[self.index.load(Ordering::Acquire)].1.trim() == answer.trim() {
            self.next(true);
            return true;
        }
        false
    }

    pub fn get_results(&self) -> Vec<(String, String, bool)> {
        (self.cards)()
            .iter()
            .zip(self.question_answers.iter().cloned())
            .map(|((q, a), y)| ((*q).clone(), (*a).clone(), y))
            .collect()
    }

    pub fn skip(&mut self) {
        self.next(false);
    }

    pub(crate) fn stop_quiz(&mut self) {
        if let Ok(mut lock) = self.thread.write() {
            if let Some(_) = lock.as_ref() {
                *lock = None;
            }
        }
        Arc::make_mut(&mut self.question_answers).clear();
        self.index.store(0, Ordering::Release);
    }

    fn next(&mut self, success: bool) {
        Arc::make_mut(&mut self.question_answers).push(success);
        self.index.fetch_add(1, Ordering::AcqRel);
    }

    pub fn quiz_is_over(&self) -> bool {
        self.index.load(Ordering::Acquire) >= (self.cards)().len()
    }
}
