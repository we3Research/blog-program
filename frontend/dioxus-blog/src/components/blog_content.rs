use reqwasm::http::Request;
use crate::{apis::BASE_URL, *};

#[component]
pub fn BlogContent(id: String) -> Element {
    let id = use_signal(|| id);
    let res = use_resource(move || async move { get_blog_content(id.read().to_string()).await });

    rsx!(
        div {
            "Content : ~"
            if let Some(Ok(data)) = &*res.read() {
                p { class: "whitespace-pre-line", "{data}" }
            }
        }
    )
}

async fn get_blog_content(id: String) -> dioxus::Result<String, dioxus::Error> {
    let res = Request::get(&format!("{}/ipfs/{id}", BASE_URL, id = id))
        .send()
        .await?;
    let body = res.text().await?;
    info!("res: {:?}", body);

    Ok(body)
}
