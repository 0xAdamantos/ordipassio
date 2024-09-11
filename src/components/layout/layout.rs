use dioxus::prelude::*;

use crate::components::layout::Body;
use crate::components::layout::Footer;
use crate::components::layout::Header;

#[component]
pub fn Wrapper() -> Element {
    rsx! {
      div {
        class: "min-h-[100vh] flex flex-col",
        Header {}
        Body {}
        Footer {}
      }
    }
}
