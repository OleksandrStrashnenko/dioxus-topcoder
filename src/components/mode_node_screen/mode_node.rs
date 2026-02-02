use crate::components::cards_panel::cards_panel::CardsPanel;
use crate::components::history::HistoryBar;
use crate::components::quiz::quiz::QuizArea;
use crate::components::working_panel::working_panel::WorkingPanel;
use crate::quiz_service::quiz_service::QuizServiceImpl;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use std::sync::Arc;
use dioxus::logger::tracing::log;
use rusqlite::Error;
use rusqlite::params;
use crate::QUIZ_DB;

#[component]
pub(crate) fn ModeNode() -> Element {
    let mut quiz_activated = use_signal(|| false);
    use_context_provider::<Signal<Option<usize>>>(|| Signal::new(None));


    let mut cards: Vec<(String, String)> = vec![];
    let query_result = QUIZ_DB.with(|f| {
        let q_iter = f.prepare("SELECT * FROM quiz");

        match q_iter {
            Err(e) => println!("Error {}", e),
            Ok(mut q_res) => {
                q_res.query_map([], |row| {
                    Ok(cards.push((row.get(0)?, row.get(1)?)))
                }).and_then(|cards| Ok(cards.collect::<Vec<_>>()));
            }
        }
    });
    let cards = Signal::new(Arc::new(cards));
    use_context_provider::<Signal<Arc<Vec<(String, String)>>>>(|| cards);
    use_effect(move || {
        let cards = cards.read();
        save_questions(&cards.to_vec());
    });
    let quiz_service = use_signal(|| QuizServiceImpl::from_cards(cards));
    rsx! {
        if quiz_activated() {
            QuizArea { quiz_activated, quiz_service }
        } else {
            div { class: "width-100 flex-column justify-content-start",
                WorkingPanel {}
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        quiz_activated.set(!quiz_activated());
                    },
                    "Start quiz"
                }
                CardsPanel {}
            }

            HistoryBar {}
        }
    }
}

fn save_questions(questions: &Vec<(String, String)>) {
    // fixme: it is called at the start
    QUIZ_DB.with(|f| {
        for (q, a) in questions {
            if let Err(e) = f.execute("INSERT INTO quiz (question, answer) VALUES (?1, ?2)", (q, a)) {
                // log::error!("Failed to save questions: {}", e);
                println!("Failed to save questions: {}", e)
            }
        }
    });
}
