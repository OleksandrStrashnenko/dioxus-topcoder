use dioxus::core_macro::{component, rsx, Props};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct IdAndContent {
    index: usize,
    content: Element,
    button: Element,
}

#[component]
pub fn CardWithElement(props: IdAndContent) -> Element {
    let mut current_draggable = use_context::<Signal<Option<usize>>>();
    rsx! {
        div { class: "flex-row",
            div {
                draggable: true,
                width: "100px",
                ondrag: move |_e| {
                    current_draggable.set(Some(props.index));
                },
                ondragstart: move |e| { println!("dragstart: {:?}", e) },
                ondragend: move |e| {
                    e.prevent_default();
                    println!("dragend: {:?}", e)
                },
                class: "card",
                style: "margin: 10px",
                {props.content}
            }
            {props.button}
        }
    }
}
