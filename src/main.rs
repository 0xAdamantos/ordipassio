#![allow(non_snake_case)]

mod components;

use crate::manganis;
use components::admin::{AdminIndex, AdminLogin};
use components::blog::BlogIndex;
use components::contact::ContactIndex;
use components::layout::Wrapper;
use components::nosservices::NosServicesIndex;
use dioxus::prelude::*;
use tracing::{info, Level};

const _STYLE: &str = asset!("assets/tailwind.css");

/**
 * TODO:
 * - imagine design structure
 * - implement admin panel
 * - implement mysql database connection & interaction
 * - implement writing article from admin panel
 * - implement portfolio insertion from admin panel
 */

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum Route {
    #[layout(Wrapper)]
    #[route("/")]
    BlogIndex {},

    #[route("/contact")]
    ContactIndex {},

    #[route("/nos-services")]
    NosServicesIndex {},

    #[route("/admin")]
    AdminIndex {},

    #[route("/admin/login")]
    AdminLogin {},

    #[route("/blog/:id")]
    Blog { id: i32 },
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::BlogIndex {}, "Go to counter" }
        "Blog post {id}"
    }
}
