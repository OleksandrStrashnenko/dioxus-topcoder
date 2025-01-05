use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::{Element, Event};
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
    let mut translate_from = use_signal(|| LanguageCode::English);
    // let mut modal_style = use_signal(|| "".to_string());
    // let handle_on_language_select_clicked = move|evt: Event<MouseData>| async move {
    //     async {
    //
    //     }.await;
    //     modal_style.set("display: flex".to_string());
    // };
    let handle_on_language_selected = move |_evt: Event<MouseData>, language_code: LanguageCode| async move {
        async {
        }.await;

        translate_from.set(language_code.clone());
    };
    // let mouse_down = move |evt: Event<MouseData>| async move {
    //     async {
    //
    //     }.await;
    //     let modal = gloo_utils::document().get_element_by_id("modal").unwrap();
    //     if !modal.contains(Some(evt.data.)) {
    //         modal_style.set("display: none".to_string());
    //     }
    // };
    rsx! {
        div {
            class: "modal",
            "aria-hidden": "true",
            id: "modal",
            tabindex: "-1",
            // style: modal_style,
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
                                onclick: move |e| { handle_on_language_selected(e.clone(), language) },
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
                    // onclick: handle_on_language_select_clicked,
                    "{translate_from:?}"
                }
                textarea {
                    id: "source-to-translate",
                    class: "working-div",
                    oninput: handle_key_down,
                }
            }
            div { class: "full-high", id: "target",
                button { r#type: "button", class: "btn dropdown-toggle", "Languages" }
                div { class: "working-div", id: "translated", "{trans}" }
            }
        }
    }
}
