use std::thread::Scope;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::{Element, Event};
use dioxus::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;


#[component]
pub fn CardsPanel() -> Element {
    let container_id = "sortable-list";
    let cardFakes = vec!["First", "Fake", "Last"];
    rsx! {
        div {
            id: "{container_id}",
            style: "list-style-type: none; padding: 10px;",
            for (cardId, card) in cardFakes.iter().enumerate() {
                div {
                    id: "card-{cardId}",
                    draggable: true,
                    width: "100px",
                    ondragstart: move |e| {println!("dragstart: {:?}", e)},
                    ondragend: move |e| { println!("dragend: {:?}", e) },

                    class: "card",
                    style: "margin: 10px",
                    "{card}"
                }
            }
            div {
                id: "drop-t",
                height: "200px",
                ondrag: move |e| { e.prevent_default(); println!("ondrag: {:?}", e) },
                ondragenter: move |e| { e.prevent_default(); println!("enter: {:?}", e);  },
                ondragover: move |e| { e.prevent_default(); println!("over: {:?}", e); },
                ondragleave: move |e| { println!("leave: {:?}", e) },
                ondrop: move |e| { e.prevent_default(); println!("drop: {:?}", e) },
                "vodka"
            }
        }
    }
}
