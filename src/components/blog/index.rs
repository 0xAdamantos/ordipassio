use dioxus::prelude::*;
use tracing::{info, Level};

#[component]
pub fn BlogIndex() -> Element {
    rsx! {
        div {
            class: "",
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        tracing::info!("Client received: {}", data);
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            div {
                class:"flex flex-col gap-6",
                div {"Test spam"}
            }
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
