use std::future::Future;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::{Element, Event};
use dioxus::document::{Style, StyleProps};
use dioxus::events::FormData;
use dioxus::hooks::{use_context, use_signal};
use dioxus::prelude::*;
use dioxus::signals::WritableVecExt;
use crate::components::history::HistoryItem;
use crate::translate::translate::translate_from_db_or_google;
use crate::translate::language_codes::LanguageCode;

const TRANSLATED: &str = "Translated";

#[component]
pub fn WorkingPanel() -> Element {
    let mut trans = use_signal(|| TRANSLATED.to_string());
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
    let mut modal_style = use_signal(|| "".to_string());
    let handle_on_language_select_clicked = move|evt: Event<MouseData>| async move {
        async {

        }.await;
        modal_style.set("display: flex".to_string());
    };
    let handle_on_language_selected = move |evt: Event<MouseData>| async move {
        async {

        }.await;
        modal_style.set("display: none".to_string());
    };
    let mouse_down = move |evt: Event<MouseData>| async move {
        async {

        }.await;
        let modal = gloo_utils::document().get_element_by_id("modal").unwrap();
        if !modal.contains(Some(evt.data.)) {
            modal_style.set("display: none".to_string());
        }
    };
    rsx! {
        div { onclick: mouse_down,
            class: "modal", id: "modal", style: modal_style,
            div { class: "modal-content",
                h3 { "Select Languages" },
                for language in LanguageCode::values() {
                    div { class: "lang-select", onclick: handle_on_language_selected,  "{language:?}" }
                }
            }
        }
        div { id: "working-panel", class: "white rounded",
            div { class: "full-high",
                button { class: "btn dropdown-toggle", onclick: handle_on_language_select_clicked, "languages" }
                textarea {
                    id: "source-to-translate",
                    class: "working-div",
                    oninput: handle_key_down,
                }
            }
            div { class: "full-high",
                id: "target",
                button { class: "btn dropdown-toggle", "Languages"}
                div { class: "working-div", id: "translated", "{trans}" }
            }
        }
    }
}
