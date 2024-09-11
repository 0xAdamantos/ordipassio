use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
      div {
        class: "bg-slate-950 text-white py-[70px]",
        "footer"
      }
    }
}
