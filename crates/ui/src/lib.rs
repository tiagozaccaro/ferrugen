pub mod hero;
pub mod navbar;
pub mod echo;

pub use hero::Hero;
pub use navbar::Navbar;
pub use echo::Echo;

use dioxus::prelude::*;

#[component]
pub fn AppShell(title: String, children: Element) -> Element {
    rsx! {
        main { class: "app-shell",
            header { h1 { "{title}" } }
            section { class: "app-content", {children} }
        }
    }
}

#[component]
pub fn EmptyState(title: String, description: String) -> Element {
    rsx! {
        section { class: "empty-state",
            h2 { "{title}" }
            p { "{description}" }
        }
    }
}
