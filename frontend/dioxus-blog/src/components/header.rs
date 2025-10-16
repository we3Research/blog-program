use crate::*;

#[component]
pub fn Header() -> Element {
    let mut wallet_context = use_context::<WalletContext>();
    rsx! {
        div {
            button {
                class: "btn btn-primary",
                onclick: move |_| {
                    info!("Connecting to Phantom...");
                    let res = wallet_context.connect_wallet("Phantom".to_string());
                    if let Err(err) = res {
                        warn!("Error: {:?}", err)
                    }
                },
                "Connect to Phantom"
            }
        }
    }
}

#[component]
fn WalletInfo() -> Element {
    let wallet_context = use_context::<WalletContext>();
    rsx!(
        div {
            if let Some(wallet) = wallet_context.wallet {
                p {
                    "Wallet name: "
                    {wallet.name()}
                }
                if let Some(icon) = wallet.icon() {
                    img { src: icon.as_ref().to_string() }
                }
            } else {
                p { "Wallet: None" }
            }
        }
    )
}
