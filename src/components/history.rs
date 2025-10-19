use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

pub struct HistoryItem {
    src: String,
    translated: String,
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
    let history_list = use_context::<Signal<Vec<HistoryItem>>>();
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
        div { id: "history-bar", class: "{collapse_active} width-100 overflow-y-auto",
            h1 { "History" }
            div {
                table {
                    for item in history_list.iter() {
                        tr {
                            td { class: "font-normal", "{item.src()}" }
                            td { class: "font-normal", "{item.translated()}" }
                        }
                    }
                }
            }
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
