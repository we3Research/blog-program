use crate::*;

#[derive(Debug, Clone)]
pub struct WalletContext {
    pub wallet: Option<wallet_adapter::Wallet>,
}

impl WalletContext {
    pub fn connect_wallet(&mut self, wallet_name: String) -> dioxus::Result<(), dioxus::Error> {
        let adapter = wallet_adapter::WalletAdapter::init().unwrap();
        let wallet = adapter.get_wallet(&wallet_name);
        match wallet {
            Ok(wallet) => {
                info!("Connected wallet: {}", &wallet.name().to_string());
                self.wallet = Some(wallet);
                Ok(())
            }
            Err(err) => Err(dioxus::Error::msg(format!(
                "Wallet not found: {} , reason",
                wallet_name,
                // format!("{}", err)
            ))),
        }
    }
}

#[component]
pub fn WalletProvider(children: Element) -> Element {
    use_context_provider::<WalletContext>(|| WalletContext { wallet: None });
    children
}
