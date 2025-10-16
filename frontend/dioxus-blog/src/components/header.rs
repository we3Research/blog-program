use crate::*;

#[component]
pub fn Header() -> Element {
    let mut wallet_context = use_context::<WalletContext>();
    let wallet_connected = wallet_context.wallet.read().is_some();

    rsx! {
        div { class: "navbar bg-base-100 shadow-lg px-6 py-2 sticky top-0 z-50",
            div { class: "flex-1",
                h1 { class: "text-2xl font-bold text-primary", "Web3 Blog" }
            }
            div { class: "flex flex-row gap-2 items-center",
                WalletInfo {}
                button {
                    class: "btn btn-primary btn-sm",
                    onclick: move |_| {
                        let res = wallet_context.connect_wallet("Phantom".to_string());
                        if let Err(err) = res {
                            warn!("Error: {:?}", err)
                        }
                    },
                    if wallet_connected {
                        "Disconnect"
                    } else {
                        "Connect"
                    }
                }
            }
        }
    }
}

#[component]
fn WalletInfo() -> Element {
    let wallet_context = use_context::<WalletContext>();
    rsx!(
        div { class: "flex items-center",
            if let Some(wallet) = &*wallet_context.wallet.read() {
                div { class: "flex flex-col items-center gap-1",
                    if let Some(icon) = wallet.icon() {
                        img { class: "w-8 h-8", src: icon.as_ref().to_string() }
                    }
                    p { class: "text-xs", {wallet.name()} }
                }
            } else {
                p { class: "text-sm italic text-gray-500", "Wallet not connected" }
            }
        }
    )
}
