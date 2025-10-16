use crate::*;

#[derive(Debug, Clone, Copy)]
pub struct WalletContext {
    pub wallet: Signal<Option<wallet_adapter::Wallet>>,
}

impl WalletContext {
    pub fn connect_wallet(&mut self, wallet_name: String) -> dioxus::Result<(), dioxus::Error> {
        let adapter = wallet_adapter::WalletAdapter::init().unwrap();
        let wallet = adapter.get_wallet(&wallet_name);
        match wallet {
            Ok(wallet) => {
                info!("Success Connected wallet: {}", &wallet.name().to_string());
                info!(" {}", &wallet.icon().as_ref().unwrap().to_string());
                self.wallet.set(Some(wallet));
                Ok(())
            }
            Err(err) => Err(dioxus::Error::msg(format!(
                "Wallet not found: {} , reason, {}",
                wallet_name,
                err.to_string()
            ))),
        }
    }
}

#[component]
pub fn WalletProvider(children: Element) -> Element {
    use_context_provider::<WalletContext>(|| WalletContext {
        wallet: Signal::new(None),
    });
    children
}
