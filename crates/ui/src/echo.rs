use dioxus::prelude::*;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");

/// Echo component that demonstrates fullstack server functions.
/// Note: requires a fullstack server endpoint (e.g., `#[post("/api/echo")]` in the API layer).
#[component]
pub fn Echo() -> Element {
    let mut response = use_signal(String::new);

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput: move |event| async move {
                    // This references a server function provided by the fullstack app layer.
                    // When connecting to an Axum-based API, replace with an HTTP client call.
                    // For now, we keep the original behavior assuming `api::echo` is wired.
                    // Server echo requires api server function; kept as placeholder.
                    let data = event.value().to_string();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}
