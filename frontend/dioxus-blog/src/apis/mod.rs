use std::str::FromStr;

use serde::{Deserialize, Serialize};
use solana_sdk::{native_token::LAMPORTS_PER_SOL, pubkey::Pubkey};
use wallet_adapter::SendOptions;
use wasm_client_solana::SolanaRpcClient;

use crate::*;

pub const BASE_URL: &str = "http://120.26.192.103:8088";

#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateBlog {
    pub title: String,
    pub author: String,
    pub cid: String,
}

pub async fn foo() -> dioxus::Result<(), dioxus::Error> {
    let rpc_client = use_context::<SolanaRpcClient>();
    let wallet_context = use_context::<WalletContext>();
    let wallet = &*wallet_context.wallet.read();

    if let Some(wallet) = wallet {
        let wallet_account = wallet.connect().await?;

        let program_id = "D1qcfn5Eevy8LCoAb3vrCTrDzNzDYqi7spG3ja3BgFJg";
        let program_id = Pubkey::from_str(program_id).unwrap();
        info!("wallet_account: {:?}", wallet_account);
        let public_key = wallet_account.public_key();

        let payer = Pubkey::new_from_array(public_key);

        let cid = "Qmbejuo7kNThaeRMjLQc3wh".to_string();

        let (blog_metadata_addr, _b1) = Pubkey::find_program_address(
            &[b"blog_metadata", cid.as_bytes(), public_key.as_ref()],
            &program_id,
        );
        let (blog_list_addr, _b2) =
            Pubkey::find_program_address(&[b"blog_list", public_key.as_ref()], &program_id);

        let solana_system_pubkey =
            solana_sdk::pubkey::Pubkey::new_from_array([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        let accounts = vec![
            solana_sdk::instruction::AccountMeta::new(blog_metadata_addr, false),
            solana_sdk::instruction::AccountMeta::new(blog_list_addr, false),
            solana_sdk::instruction::AccountMeta::new(payer, true),
            solana_sdk::instruction::AccountMeta::new_readonly(solana_system_pubkey, false),
        ];

        let create_blog_data = CreateBlog {
            title: "test1".to_string(),
            author: cid.to_string(),
            cid,
        };

        let block_hash = rpc_client.get_latest_blockhash().await?;

        let instruction = solana_sdk::instruction::Instruction::new_with_bincode(
            program_id,
            &create_blog_data,
            accounts,
        );

        info!("instruction: {:?}", instruction);

        let mut tx =
            solana_sdk::transaction::Transaction::new_with_payer(&[instruction], Some(&payer));

        let recent_blockhash = solana_sdk::hash::Hash::from_str(&block_hash.to_string()).unwrap();
        info!("recent_blockhash: {:?}", recent_blockhash);

        tx.message.recent_blockhash = recent_blockhash;
        info!("tx: {:?}", tx);
        let tx_bytes = bincode::serialize(&tx).unwrap();

        // info!("tx_bytes: {:?}", tx_bytes);
        // let output = wallet
        //     .sign_transaction(
        //         &tx_bytes,
        //         Some(wallet_adapter::Cluster::DevNet),
        //         &wallet_account,
        //     )
        //     .await?;

        let output = wallet
            .sign_and_send_transaction(
                &tx_bytes,
                wallet_adapter::Cluster::DevNet,
                SendOptions::default(),
                &wallet_account,
            )
            .await?;

        info!("output: {:?}", output);

        // let deser_tx_output =
        //     bincode::deserialize::<solana_sdk::transaction::Transaction>(&output[0]).unwrap();
        // info!("deser_tx_output: {:?}", deser_tx_output);
    }

    Ok(())
}
