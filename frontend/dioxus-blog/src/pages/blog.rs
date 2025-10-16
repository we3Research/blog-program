use crate::*;
use components::BlogContent;
use wasm_client_solana::SolanaRpcClient;
use web3_blog_program::BlogMetadata;

#[component]
pub fn Blog(id: String) -> Element {
    let id = use_signal(|| id);
    let blog_metadata =
        use_resource(move || async move { get_blog_metadata(id.read().to_string()).await });

    let res = &*blog_metadata.read();

    rsx! {
        div { class: "max-w-4xl mx-auto",
            match res {
                Some(Ok(blog_metadata)) => {
                    rsx! {
                        div { class: "card bg-base-200 shadow-xl p-6 mb-6",
                            h1 { class: "text-3xl font-bold mb-2", "{blog_metadata.title}" }
                            div { class: "flex flex-wrap items-center gap-4 mb-6 text-sm",
                                p { class: "badge badge-primary", "Author: {blog_metadata.author}" }
                                p { class: "badge badge-secondary",
                                    "Created: "
                                    {
                                        {
                                            dayjs::from_int64(blog_metadata.create_at)
                                                .unwrap()
                                                .format("%Y-%m-%d %H:%M:%S")
                                        }
                                    }
                                }
                                p { class: "badge badge-accent",
                                    "Updated: "
                                    {
                                        {
                                            dayjs::from_int64(blog_metadata.update_at)
                                                .unwrap()
                                                .format("%Y-%m-%d %H:%M:%S")
                                        }
                                    }
                                }
                            }
                            div { class: "divider" }
                            div { class: "mb-6",
                                h2 { class: "text-xl font-semibold mb-3", "Content" }
                                p { class: "whitespace-pre-wrap", "cid: {blog_metadata.cid}" }
                                BlogContent { id: blog_metadata.cid.clone() }
                            }
                            div { class: "collapse collapse-arrow bg-base-300",
                                input { r#type: "checkbox" }
                                div { class: "collapse-title font-medium", "Content History" }
                                div { class: "collapse-content",
                                    ul { class: "list-disc list-inside",
                                        for cid in blog_metadata.history.iter() {
                                            li { "{cid}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some(Err(_)) => {
                    rsx! {
                        div { class: "alert alert-error shadow-lg",
                            h2 { class: "text-xl font-bold", "Error" }
                            p { "Failed to load blog metadata" }
                        }
                    }
                }
                None => {
                    rsx! {
                        div { class: "flex justify-center items-center h-64",
                            div { class: "loading loading-spinner loading-lg" }
                        }
                    }
                }
            }
        }
    }
}

async fn get_blog_metadata(id: String) -> dioxus::Result<BlogMetadata, dioxus::Error> {
    let client = use_context::<SolanaRpcClient>();

    let blog_metadata_address = Pubkey::from_str_const(&id);
    let data = client.get_account_data(&blog_metadata_address).await?;

    let blog_metadata = BlogMetadata::try_deserialize(&mut data.as_slice())?;

    Ok(blog_metadata)
}
