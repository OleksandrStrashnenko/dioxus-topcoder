use crate::quiz_service::quiz_service::QuizServiceImpl;
use dioxus::core_macro::{component, rsx, Props};
use dioxus::dioxus_core::Element;
use dioxus::prelude::*;
use std::time::Duration;
use dioxus_sdk_time::use_interval;

#[derive(Props, PartialEq, Clone)]
pub struct Activated {
    quiz_activated: Signal<bool>,
    quiz_service: Signal<QuizServiceImpl>,
}

#[component]
pub(crate) fn QuizArea(mut props: Activated) -> Element {
    let mut hided = use_signal_sync(|| false);
    let mut count = use_signal(|| 0);
    use_interval(Duration::from_secs(1), move |()| count += 1);
    let mut input_text = use_signal(|| String::new());
    rsx! {
        div { width: "100%", max_width: "1080px", id: "quiz-main",
            div { width: "50%",
                button {
                    class: "btn btn-primary",
                    onclick: move |_| {
                        props.quiz_activated.set(false);
                        props.quiz_service.write().stop_quiz();
                    },
                    if !props.quiz_service.read().quiz_is_over() {
                        "Stop quiz"
                    } else {
                        "Quit"
                    }
                }
                if props.quiz_service.read().quiz_is_over() {
                    if !hided() {
                        div {
                            button { onclick: move |_| hided.set(!hided()), "Show answers" }
                        }
                    }
                    table {
                        thead {
                            tr {
                                td {
                                    div {
                                        class: "card",
                                        style: "background: rgba(215, 200, 200, 0.24)",
                                        "From"
                                    }
                                }
                                td {
                                    div {
                                        class: "card",
                                        style: "background: rgba(215, 200, 200, 0.24)",
                                        "To"
                                    }
                                }
                                td {
                                    div {
                                        class: "card",
                                        style: "background: rgba(215, 200, 200, 0.24)",
                                        "Success"
                                    }
                                }
                            }
                        }
                        tbody {
                            for result in props.quiz_service.read().get_results().iter() {
                                tr {
                                    td {
                                        div { class: "card", "{result.0}" }
                                    }
                                    td {
                                        div { class: "card",
                                            if result.2 || hided() {
                                                "{result.1}"
                                            } else {
                                                ""
                                            }
                                        }
                                    }
                                    td {
                                        div { class: "card",
                                            if result.2 {
                                                {check()}
                                            } else {
                                                {x()}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    div { class: "card",
                        button {
                            class: "btn btn-secondary",
                            onclick: move |_| {
                                props.quiz_service.write().skip();
                                hided.set(hided());
                            },
                            "Skip"
                        }
                    }
                    div {
                        div { "{hided.read()}" }
                        "{props.quiz_service.read().current_question()}"
                        input {
                            oninput: move |e| {
                                input_text.set(e.data.value());
                                let correct = props.quiz_service.write().check_for_answer(input_text().as_str());
                                if correct {
                                    input_text.set("".into())
                                }
                            },

                            value: "{input_text()}",
                        }
                    }
                }
                div { "{count}" }
            }
        }
    }
}

fn check() -> Element {
    rsx! {
        svg {
            width: "2em",
            height: "2em",
            xmlns: "http://www.w3.org/2000/svg",
            class: "bi bi-check",
            "viewBox": "0 0 16 16",
            "aria_hidden": "true",
            path { d: "M10.97 4.97a.75.75 0 0 1 1.07 1.05l-3.99 4.99a.75.75 0 0 1-1.08.02L4.324 8.384a.75.75 0 1 1 1.06-1.06l2.094 2.093 3.473-4.425z" }
        }
    }
}

fn x() -> Element {
    rsx! {
        svg {
            width: "2em",
            height: "2em",
            xmlns: "http://www.w3.org/2000/svg",
            class: "bi bi-x",
            "viewBox": "0 0 16 16",
            "aria_hidden": "true",
            path { d: "M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708" }
        }
    }
}
