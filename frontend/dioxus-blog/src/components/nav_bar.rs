use anchor_lang::{prelude::Pubkey, AccountDeserialize};
use wasm_client_solana::SolanaRpcClient;
use web3_blog_program::BlogList;

use crate::*;
const BLOG_LIST: &'static str = "2oAs4aTanvsB4y755fF5QWU2DdqSx9JhWa5JDSYnitbH";

#[component]
pub fn Navbar() -> Element {
    let blog_list = use_resource(get_blog_list);
    let res = &*blog_list.read();

    rsx! {
        div {
            Header {}
            div { class: "flex flex-col md:flex-row w-full min-h-screen",
                // 左侧导航栏
                div { class: "w-full md:w-64 bg-base-200 p-4 flex-shrink-0",
                    div { class: "mb-8",
                        h1 { class: "text-2xl font-bold mb-4 text-primary", "Web3 Blog" }
                        Link {
                            class: "link link-hover block py-2",
                            to: Route::Home {},
                            "Home"
                        }
                    }

                    div { class: "divider", "Blogs" }

                    div { class: "space-y-2",
                        if let Some(Ok(blog_list_data)) = res {
                            {
                                blog_list_data
                                    .list
                                    .iter()
                                    .map(|blog| {
                                        rsx! {
                                            Link {
                                                class: "link link-hover block py-2 truncate",
                                                to: Route::Blog {
                                                    id: blog.to_owned().to_string(),
                                                },
                                                "{blog.to_string()}"
                                            }
                                        }
                                    })
                            }
                        } else {
                            div { class: "text-sm opacity-50", "Loading blogs..." }
                        }
                    }
                }

                // 右侧内容区域
                div { class: "flex-1 bg-base-100 p-4 md:p-8", Outlet::<Route> {} }
            }
        }
    }
}

async fn get_blog_list() -> dioxus::Result<BlogList, dioxus::Error> {
    let client = use_context::<SolanaRpcClient>();
    let blog_list_address = Pubkey::from_str_const(BLOG_LIST);

    let data = client.get_account_data(&blog_list_address).await?;
    let blog_list = BlogList::try_deserialize(&mut data.as_slice())?;

    info!("blog_list: {:?}", blog_list);
    Ok(blog_list)
}
