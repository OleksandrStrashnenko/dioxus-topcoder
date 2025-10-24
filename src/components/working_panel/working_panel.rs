use crate::components::history::HistoryItem;
use crate::translate::language_codes::LanguageCode;
use crate::translate::translate::translate_from_db_or_google;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::{Element, Event};
use dioxus::events::FormData;
use dioxus::hooks::{use_context, use_signal};
use dioxus::prelude::*;
use dioxus::signals::WritableVecExt;
const TRANSLATED: &str = "Translated";

enum Target {
    TO,
    FROM,
}

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
            }
            None => {
                trans.set("Translation".to_string());
            }
        }
    };

    let mut translate_from = use_signal(|| LanguageCode::English);
    let mut translate_to = use_signal(|| LanguageCode::Ukrainian);
    let mut target = use_signal(|| Target::TO);
    let handle_language_selected = move |_evt: Event<MouseData>, language_code: LanguageCode| async move {
        async {}.await;
        match *target.read() {
            Target::FROM => {
                translate_from.set(language_code.clone());
            }
            Target::TO => {
                translate_to.set(language_code.clone());
            }
        }
    };
    let mut handle_language_select_clicked = move |_target: Target| {
        target.set(_target);
    };
    rsx! {
        div {
            class: "modal",
            "aria-hidden": "true",
            id: "modal",
            tabindex: "-1",
            div { class: "modal-dialog modal-dialog-centered",
                div { class: "modal-content",
                    div { class: "modal-header",
                        h3 { class: "modal-title", "Select Language" }
                    }
                    div { class: "modal-body",
                        for language in LanguageCode::values() {
                            button {
                                class: "lang-select",
                                "data-bs-dismiss": "modal",
                                onclick: move |e| { handle_language_selected(e.clone(), language) },
                                "{language:?}"
                            }
                        }
                    }
                }
            }
        }
        div { id: "working-panel", class: "white rounded",
            div { class: "full-high",
                button {
                    r#type: "button",
                    class: "btn dropdown-toggle",
                    "data-bs-toggle": "modal",
                    "data-bs-target": "#modal",
                    onclick: move |_e| { handle_language_select_clicked(Target::FROM) },
                    "{translate_from:?}"
                }
                textarea {
                    id: "source-to-translate",
                    class: "working-div",
                    oninput: handle_key_down,
                }
            }
            div { class: "full-high", id: "target",
                button {
                    r#type: "button",
                    class: "btn dropdown-toggle",
                    "data-bs-toggle": "modal",
                    "data-bs-target": "#modal",
                    onclick: move |_e| { handle_language_select_clicked(Target::TO) },
                    "{translate_to:?}"
                }
                div { class: "working-div", id: "translated", "{trans}" }
            }
        }
    }
}
