use crate::components::cards_panel::draggable_card::Card;
use crate::components::history::HistoryItem;
use dioxus::core_macro::{component, rsx};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use std::sync::Arc;

#[component]
pub fn CardsPanel() -> Element {
    let container_id = "sortable-list";
    let mut index = use_context::<Signal<Option<usize>>>();
    let mut cards: Signal<Arc<Vec<(String, String)>>> =
        use_context::<Signal<Arc<Vec<(String, String)>>>>();
    let ondrop = move |_e| {
        if let Some(i) = index() {
            let history_item = use_context::<Signal<Vec<HistoryItem>>>()();
            if let Some(item) = history_item.get(i) {
                Arc::<Vec<(String, String)>>::make_mut(&mut cards.write())
                    .push((item.src.clone(), item.translated.clone()));
                index.set(None);
            }
        }
    };
    rsx! {
        div {
            id: "{container_id}",
            style: "list-style-type: none; padding: 10px;",
            ondrop,
            for (cardId , (c1 , c2)) in cards.read().iter().enumerate() {
                Card { id: cardId, content: "{c1} {c2}" }
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
                ondragleave: move |e| {
                    println!("leave: {:?}", e);
                },
                ondrop: move |e| {
                    println!("drop: {:?}", e);
                },
                "test"
            }
        }
    }
}
