use axum::response::Response;
use dioxus::core_macro::{component, rsx};
use dioxus::desktop::use_asset_handler;
use dioxus::dioxus_core::Element;
use dioxus::document;
use dioxus::hooks::use_context_provider;
use dioxus::prelude::*;
use crate::components::cards_panel::cards_panel::CardsPanel;
use crate::components::history::{HistoryBar, HistoryItem};
use crate::components::working_panel::working_panel::WorkingPanel;

const MAIN_CSS: &str = "main.css";
const WORKING_PANEL_CSS: &str = "working-panel.css";
const HISTORY_BAR_CSS: &str = "history-bar.css";
const FAVICON: &str = "favicon.png";
const BOOTSTRAP_CSS: &str = "bootstrap.css";
const BOOTSTRAP_RTL_CSS: &str = "bootstrap.rtl.css";
const BOOTSTRAP_GRID_CSS: &str = "bootstrap-grid.css";
const BOOTSTRAP_GRID_RTL_CSS: &str = "bootstrap-grid.rtl.css";
const BOOTSTRAP_REBOOT_CSS: &str = "bootstrap-reboot.css";
const BOOTSTRAP_REBOOT_RTL_CSS: &str = "bootstrap-reboot.rtl.css";
const BOOTSTRAP_UTILITIES_CSS: &str = "bootstrap-utilities.css";
const BOOTSTRAP_UTILITIES_RTL_CSS: &str = "bootstrap-utilities.rtl.css";
const HEADER_SVG: &str = "header.svg";
const BOOSTRAP_JS: &str = "bootstrap.js";
const BOOSTRAP_ESM_JS: &str = "bootstrap.esm.js";
const BOOSTRAP_BUNDLE_JS: &str = "bootstrap.bundle.js";
const BOOTSTRAP_ICONS_SVG: &str = "bootstrap-icons.svg";

const BOOTSTRAP_ICONS_CHEVRON_LEFT: &str = "chevron-left.svg";
const BOOTSTRAP_ICONS_CHEVRON_RIGHT: &str = "chevron-right.svg";
const BOOTSTRAP_ICONS_CSS: &str = "bootstrap-icons.css";
const BOOTSTRAP_ICONS_WOFF: &str = "bootstrap-icons.woff";
const BOOTSTRAP_ICONS_WOFF2: &str = "assets/icons/fonts/bootstrap-icons.woff2";
// const SORTABLE_JS: &str = "sortable.js";


// #[component]
#[allow(non_snake_case)]
#[component]
pub fn App() -> Element {
    use_asset_handler(MAIN_CSS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/main.css").to_vec()));
    });
    use_asset_handler(WORKING_PANEL_CSS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/working-panel.css").to_vec()));
    });
    use_asset_handler(HISTORY_BAR_CSS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/history-bar.css").to_vec()));
    });
    use_asset_handler(FAVICON, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/favicon.png").to_vec()));
    });
    use_asset_handler(BOOTSTRAP_CSS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/bootstrap-5.3.5/dist/css/bootstrap.css").to_vec()));
    });
    use_asset_handler(BOOTSTRAP_RTL_CSS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/bootstrap-5.3.5/dist/css/bootstrap.rtl.css").to_vec()));
    });
    use_asset_handler(BOOTSTRAP_GRID_CSS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/bootstrap-5.3.5/dist/css/bootstrap-grid.css").to_vec()));
    });
    use_asset_handler(BOOTSTRAP_GRID_RTL_CSS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/bootstrap-5.3.5/dist/css/bootstrap-grid.rtl.css").to_vec()));
    });
    use_asset_handler(BOOTSTRAP_REBOOT_CSS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/bootstrap-5.3.5/dist/css/bootstrap-reboot.css").to_vec()));
    });
    use_asset_handler(BOOTSTRAP_REBOOT_RTL_CSS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/bootstrap-5.3.5/dist/css/bootstrap-reboot.rtl.css").to_vec()));
    });
    use_asset_handler(BOOTSTRAP_UTILITIES_CSS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/bootstrap-5.3.5/dist/css/bootstrap-utilities.css").to_vec()));
    });
    use_asset_handler(BOOTSTRAP_UTILITIES_RTL_CSS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/bootstrap-5.3.5/dist/css/bootstrap-utilities.rtl.css").to_vec()));
    });
    use_asset_handler(HEADER_SVG, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/header.svg").to_vec()));
    });
    use_asset_handler(BOOSTRAP_JS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/bootstrap-5.3.5/dist/js/bootstrap.js").to_vec()));
    });
    use_asset_handler(BOOSTRAP_ESM_JS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/bootstrap-5.3.5/dist/js/bootstrap.esm.js").to_vec()));
    });
    use_asset_handler(BOOSTRAP_BUNDLE_JS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/bootstrap-5.3.5/dist/js/bootstrap.bundle.js").to_vec()));
    });
    use_asset_handler(BOOTSTRAP_ICONS_SVG, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/icons/bootstrap-icons.svg").to_vec()));
    });
    use_asset_handler(BOOTSTRAP_ICONS_CHEVRON_LEFT, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/icons/icons/chevron-left.svg").to_vec()));
    });
    use_asset_handler(BOOTSTRAP_ICONS_CHEVRON_RIGHT, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/icons/icons/chevron-right.svg").to_vec()));
    });
    use_asset_handler(BOOTSTRAP_ICONS_CSS, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/icons/bootstrap-icons.css").to_vec()));
    });
    use_asset_handler(BOOTSTRAP_ICONS_WOFF, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/icons/fonts/bootstrap-icons.woff").to_vec()));
    });
    use_asset_handler(BOOTSTRAP_ICONS_WOFF2, |_, responder| {
        responder.respond(Response::new(include_bytes!("../../assets/icons/fonts/bootstrap-icons.woff2").to_vec()));
    });
    // use_asset_handler(SORTABLE_JS, |_, responder| {
    //     responder.respond(Response::new(include_bytes!("../../assets/Sortable.js").to_vec()));
    // });
    use_context_provider(|| Signal::<Vec<HistoryItem>>::new(vec![]));
    rsx! {
        head {
            link { rel: "icon", href: FAVICON }
        }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_RTL_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_GRID_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_GRID_RTL_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_REBOOT_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_REBOOT_RTL_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_UTILITIES_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_UTILITIES_RTL_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_ICONS_CHEVRON_LEFT }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_ICONS_CHEVRON_RIGHT }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_ICONS_CSS }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_ICONS_WOFF }
        document::Link { rel: "stylesheet", href: BOOTSTRAP_ICONS_WOFF2 }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Script { src: BOOSTRAP_JS }
        document::Script { src: BOOSTRAP_ESM_JS }
        document::Script { src: BOOSTRAP_BUNDLE_JS }
        // document::Script { src: SORTABLE_JS }
        head {
            link { rel: "stylesheet", href: WORKING_PANEL_CSS }
        }
        head {
            link { rel: "stylesheet", href: HISTORY_BAR_CSS }
        }
        document::Meta { charset: "utf-8" }
        div { id: "body",
            div { class: "width-100 flex-column justify-content-center",
                WorkingPanel {}
                CardsPanel {}
            }
            HistoryBar {}
        }
    }
}
