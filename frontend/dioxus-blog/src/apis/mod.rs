use serde::{Deserialize, Serialize};
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use wasm_client_solana::SolanaRpcClient;

use crate::*;

pub const BASE_URL: &str = "http://120.26.192.103:8088";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateBlog {
    pub blog_metadata: Pubkey,
    pub blog_list:Pubkey,
    pub signer: Pubkey,
    pub system_program: Pubkey,
}

pub async fn foo() -> dioxus::Result<(), dioxus::Error> {
    let wallet_context = use_context::<WalletContext>();
    let wallet = &*wallet_context.wallet.read();

    if let Some(wallet) = wallet {
        let wallet_account = wallet.connect().await?;
        info!("wallet_account: {:?}", wallet_account);
        let public_key = wallet_account.public_key();

        let payer = Pubkey::new_from_array(public_key);

        let recipient_pubkey = Pubkey::from_str_const(
            "2oAs4aTanvsB4y755fF5QWU2DdqSx9JhWa5JDSYnitbH",
        );
        let accounts = vec![
            solana_sdk::instruction::AccountMeta::new(payer, true),
            solana_sdk::instruction::AccountMeta::new(recipient_pubkey, false),
        ];

        let create_blog_data = CreateBlog {
            blog_metadata: Pubkey::from_str_const("D1qcfn5Eevy8LCoAb3vrCTrDzNzDYqi7spG3ja3BgFJg"),
            blog_list: Pubkey::from_str_const(BLOG_LIST),
            signer: Pubkey::new_from_array(public_key),
            system_program: Pubkey::new_from_array([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
        };


        let instruction = solana_sdk::instruction::Instruction::new_with_bincode(
            Pubkey::from_str_const(
                "D1qcfn5Eevy8LCoAb3vrCTrDzNzDYqi7spG3ja3BgFJg",
            ),
            &create_blog_data,
            accounts,
        );

        info!("instruction: {:?}", instruction);

        let tx =
            solana_sdk::transaction::Transaction::new_with_payer(&[instruction], Some(&payer));

        info!("tx: {:?}", tx);
        let tx_bytes = bincode::serialize(&tx).unwrap();
        info!("tx_bytes: {:?}", tx_bytes);
        let output = wallet
            .sign_transaction(
                &tx_bytes,
                Some(wallet_adapter::Cluster::DevNet),
                &wallet_account,
            )
            .await?;
        info!("output: {:?}", output);

        let deser_tx_output =
            bincode::deserialize::<solana_sdk::transaction::Transaction>(&output[0]).unwrap();
        info!("deser_tx_output: {:?}", deser_tx_output);
    }
    Ok(())
}
