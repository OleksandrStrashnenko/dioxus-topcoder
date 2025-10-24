use crate::components::cards_panel::draggable_card_with_component::CardWithElement;
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[derive(Clone)]
pub struct HistoryItem {
    pub(crate) src: String,
    pub(crate) translated: String,
}

impl HistoryItem {
    pub fn new(src: String, translated: String) -> Self {
        HistoryItem { src, translated }
    }

    pub fn src(&self) -> &String {
        &self.src
    }

    pub fn translated(&self) -> &String {
        &self.translated
    }
}

#[component]
pub(crate) fn HistoryBar() -> Element {
    let mut history_list = use_context::<Signal<Vec<HistoryItem>>>();
    let mut collapse_active = use_signal(|| "active");
    let mut chevron = use_signal(|| chevron_left());
    let onclick = move |_| async move {
        if collapse_active() == "" {
            collapse_active.set("active");
            chevron.set(chevron_left());
        } else {
            collapse_active.set("");
            chevron.set(chevron_right());
        }
    };
    rsx! {
        button {
            r#type: "button",
            id: "collapse-button",
            role: "button",
            aria_controls: "history-bar",
            aria_expanded: "false",
            onclick,
            {chevron}
        }
        div {
            id: "history-bar",
            class: "{collapse_active} width-100 overflow-y-auto",
            h1 { "History" }
            div {
                table {
                    for (i , item) in history_list.iter().enumerate() {
                        CardWithElement {
                            index: i,
                            content: rsx! {
                                tr {
                                    td { class: "font-normal", "{item.src()}" }
                                    td { class: "font-normal", "{item.translated()}" }
                                }
                            },
                            button: rsx! {
                                button {
                                    r#type: "button",
                                    role: "button",
                                    class: "trash-button",
                                    onclick: move |_e| {
                                        history_list.remove(i);
                                    },
                                    {trash()}
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

fn trash() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "bi bi-trash",
            "viewBox": "0 0 16 16",
            "aria_hidden": "true",
            path { d: "M5.5 5.5A.5.5 0 0 1 6 6v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5m2.5 0a.5.5 0 0 1 .5.5v6a.5.5 0 0 1-1 0V6a.5.5 0 0 1 .5-.5m3 .5a.5.5 0 0 0-1 0v6a.5.5 0 0 0 1 0z" }
            path { d: "M14.5 3a1 1 0 0 1-1 1H13v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V4h-.5a1 1 0 0 1-1-1V2a1 1 0 0 1 1-1H6a1 1 0 0 1 1-1h2a1 1 0 0 1 1 1h3.5a1 1 0 0 1 1 1zM4.118 4 4 4.059V13a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1V4.059L11.882 4zM2.5 3h11V2h-11z" }
        }
    }
}

fn chevron_left() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "bi bi-chevron-left",
            "viewBox": "0 0 16 16",
            "aria_hidden": "true",
            path { d: "M11.354 1.646a.5.5 0 0 1 0 .708L5.707 8l5.647 5.646a.5.5 0 0 1-.708.708l-6-6a.5.5 0 0 1 0-.708l6-6a.5.5 0 0 1 .708 0" }
        }
    }
}

fn chevron_right() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            class: "bi bi-chevron-right",
            "viewBox": "0 0 16 16",
            "aria_hidden": "true",
            path { d: "M4.646 1.646a.5.5 0 0 1 .708 0l6 6a.5.5 0 0 1 0 .708l-6 6a.5.5 0 0 1-.708-.708L10.293 8 4.646 2.354a.5.5 0 0 1 0-.708" }
        }
    }
}
