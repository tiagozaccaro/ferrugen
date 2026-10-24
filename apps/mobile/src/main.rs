use dioxus::prelude::*;
use ui::AppShell;

#[component]
fn Home() -> Element {
    rsx! {
        AppShell { title: "Ferrugen Platform".to_owned(),
            p { "Reusable platform workspace is running." }
            p { "This UI is a scaffold; connect it to the API before production use." }
        }
    }
}

fn main() {
    dioxus::launch(Home);
}
