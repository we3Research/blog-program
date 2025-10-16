use wasm_client_solana::{SolanaRpcClient, DEVNET};

use crate::*;

#[component]
pub fn RpcProvider(children: Element) -> Element {
    use_context_provider(|| SolanaRpcClient::new(DEVNET));
    children
}
