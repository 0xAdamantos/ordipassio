use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Body(children: Element) -> Element {
    rsx! {
      div {
        class: "pt-[80px] flex grow bg-zinc-950 text-white",
        Outlet::<Route> {}
      }
    }
}
