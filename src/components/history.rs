use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

pub struct HistoryItem {
    src: String,
    translated: String
}

impl HistoryItem {
    pub fn new(src: String, translated: String) -> Self {
        HistoryItem {src, translated}
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
    rsx! {
        div { id: "history-bar",
            h1 { "History" }
            div {
                table {
                    for item in history_list.iter() {
                        tr {
                            td { "{item.src()}" }
                            td { "{item.translated()}" }
                        }
                    }
                }
            }
        }
    }
}