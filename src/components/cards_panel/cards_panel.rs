use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;

#[component]
pub fn CardsPanel() -> Element {
    let container_id = "sortable-list";
    let cardFakes = vec!["First", "Fake", "Last"];
    rsx! {
        div {
            id: "{container_id}",
            style: "list-style-type: none; padding: 10px;",
            for (cardId , card) in cardFakes.iter().enumerate() {
                div {
                    id: "card-{cardId}",
                    draggable: true,
                    width: "100px",
                    ondrag: move |e| {},
                    ondragstart: move |e| { println!("dragstart: {:?}", e) },
                    ondragend: move |e| {
                        e.prevent_default();
                        println!("dragend: {:?}", e)
                    },
                    class: "card",
                    style: "margin: 10px",
                    "{card}"
                }
            }
            div {
                id: "drop-t",
                height: "200px",
                draggable: true,
                ondrag: move |e| { println!("ondrag: {:?}", e) },
                ondragenter: move |e| {
                    println!("enter: {:?}", e);
                },
                ondragover: move |e| {
                    println!("over: {:?}", e);
                },
                ondragleave: move |e| {},
                ondrop: move |e| { println!("drop: {:?}", e) },
                "test"
            }
        }
    }
}
