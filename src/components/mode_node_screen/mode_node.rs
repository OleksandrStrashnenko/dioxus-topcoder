use crate::components::cards_panel::cards_panel::CardsPanel;
use crate::components::history::HistoryBar;
use crate::components::quiz::quiz::QuizArea;
use crate::components::working_panel::working_panel::WorkingPanel;
use crate::quiz_service::quiz_service::QuizServiceImpl;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use std::sync::Arc;

#[component]
pub(crate) fn ModeNode() -> Element {
    let mut quiz_activated = use_signal(|| false);
    use_context_provider::<Signal<Option<usize>>>(|| Signal::new(None));
    let cards = Signal::new(Arc::new(vec![
        (String::from("First"), String::from("q1")),
        (String::from("FakeFakeFake"), String::from("w2")),
    ]));
    use_context_provider::<Signal<Arc<Vec<(String, String)>>>>(|| cards);
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
