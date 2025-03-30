use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::{Element, Event};
use dioxus::events::FormData;
use dioxus::hooks::{use_context, use_signal};
use dioxus::prelude::*;
use dioxus::signals::WritableVecExt;
use crate::components::history::HistoryItem;
use crate::translate::translate_from_db_or_google;

#[component]
pub fn WorkingPanel() -> Element {
    let mut trans = use_signal(|| "Translated".to_string());
    let mut history_list: Signal<Vec<HistoryItem>> = use_context::<Signal<Vec<HistoryItem>>>();
    let handle_key_down = move |evt: Event<FormData>| async move {
        let src = evt.data().value();
        match translate_from_db_or_google(&src).await {
            Some(translation) => {
                trans.set(translation.clone());
                history_list.push(HistoryItem::new(src.clone(), translation))
            },
            None => {
                trans.set("Translation".to_string());
            }
        }
    };
    rsx! {
        div {
            id: "working-panel",
            input {
                id: "source-text-to-translate",
                class: "working-div",
                background_color: "white",
                oninput: handle_key_down
            }
            div { class: "working-div",
                id: "translated",
                "{trans}"
            }
        }
    }
}