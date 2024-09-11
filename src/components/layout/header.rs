use crate::Route;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
struct LinkProps {
    title: String,
    href: String,
    external: bool,
}

#[component]
pub fn Header() -> Element {
    rsx! {
      div {
        class: "w-full fixed bg-teal-800/85 py-4 flex items-center place-content-end",
        Logo {}
        Menu {}
      }
    }
}

#[component]
fn Logo() -> Element {
    rsx! {
      div {
        class: "absolute left-4 uppercase text-teal-50 font-bold font-serif text-[36px]",
        "OrdiPassio"
      }
    }
}

// TODO: mobile deployable menu, with hamburger icon to deploy
#[component]
fn Menu() -> Element {
    rsx! {
      div {
        class: "lg:mx-auto flex flex-row gap-1 lg:gap-2 font-semibold font-sans text-[20px]",
        MenuLink {title: "Accueil", href: "/", external: false}
        MenuLink {title: "Nos Services", href: "nos-services", external: false}
        MenuLink {title: "Contact", href: "contact", external: false}
      }
    }
}

#[component]
fn MenuLink(props: LinkProps) -> Element {
    let title = props.title;
    let href = props.href;

    rsx! {
      Link {
        class: "px-2 lg:px-4 py-2 transition-all text-white hover:text-sky-300",
        to: href,
        new_tab: props.external,
        "{title}"
      }
    }
}
