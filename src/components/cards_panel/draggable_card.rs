use dioxus::core_macro::{component, rsx, Props};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct IdAndContent {
    id: usize,
    content: String,
}

#[component]
pub fn Card(props: IdAndContent) -> Element {
    rsx! {
        div {
            id: "card-{props.id}",
            draggable: true,
            width: "100px",
            ondrag: move |_e| {},
            ondragstart: move |_e| {},
            ondragover: move |_e| {
                println!("ondragover {_e:?}");
            },
            ondragend: move |e| {
                e.prevent_default();
            },
            class: "draggable-card",
            style: "margin: 10px",
            "{props.content}"
        }
    }
}
