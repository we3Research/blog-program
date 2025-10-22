use std::str::FromStr;

// use solana_sdk::message::AccountMeta;
use wallet_adapter::SendOptions;
use wasm_client_solana::{SolanaRpcClient, VersionedTransactionExtension};

use crate::*;

pub const BASE_URL: &str = "http://120.26.192.103:8088";

pub async fn foo() -> dioxus::Result<(), dioxus::Error> {
    let rpc_client = use_context::<SolanaRpcClient>();
    let wallet_context = use_context::<WalletContext>();
    let wallet = &*wallet_context.wallet.read();

    if let Some(wallet) = wallet {
        // create_blog 的 discriminator
        const CREATE_BLOG_DISCRIMINATOR: [u8; 8] = [221, 118, 241, 5, 53, 181, 90, 253];
        // update_blog 的 discriminator
        const UPDATE_BLOG_DISCRIMINATOR: [u8; 8] = [252, 54, 5, 181, 182, 6, 112, 203];

        let wallet_account = wallet.connect().await?;

        let payer_pubkey = wallet_account.public_key();
        let payer = solana_sdk::pubkey::Pubkey::new_from_array(payer_pubkey);

        let program_id = solana_sdk::pubkey::Pubkey::from_str_const(
            "D1qcfn5Eevy8LCoAb3vrCTrDzNzDYqi7spG3ja3BgFJg",
        );

        let cid = "Qmbejuo7kNThaeRMjLQc3wh".to_string();

        let (blog_metadata_addr, _b1) = solana_sdk::pubkey::Pubkey::find_program_address(
            &[b"blog_metadata", cid.as_bytes(), payer_pubkey.as_ref()],
            &program_id,
        );
        let (blog_list_addr, _b2) = solana_sdk::pubkey::Pubkey::find_program_address(
            &[b"blog_list", payer_pubkey.as_ref()],
            &program_id,
        );

        let create_accounts = vec![
            AccountMeta::new(blog_metadata_addr, false),
            AccountMeta::new(blog_list_addr, false),
            // AccountMeta::new(program_id, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(
                solana_sdk::pubkey!("11111111111111111111111111111111"),
                false,
            ),
        ];

        let update_accounts = vec![
            AccountMeta::new(blog_metadata_addr, false),
            // AccountMeta::new(blog_list_addr, false),
            // AccountMeta::new(program_id, false),
            AccountMeta::new(payer, true),
            // AccountMeta::new_readonly(
            //     solana_sdk::pubkey!("11111111111111111111111111111111"),
            //     false,
            // ),
        ];

        let create_blog_data = web3_blog_program::instruction::CreateBlog {
            title: "test1".to_string(),
            author: cid.to_string(),
            cid,
        };

        let update_blog_data = web3_blog_program::instruction::UpdateBlog {
            new_content: "test 123123 1234".to_string(),
        };

        let mut data = CREATE_BLOG_DISCRIMINATOR.to_vec();
        let mut update_data = UPDATE_BLOG_DISCRIMINATOR.to_vec();
        // let data = create_blog_data.try_to_vec().unwrap();
        create_blog_data.serialize(&mut data).unwrap();
        update_blog_data.serialize(&mut update_data).unwrap();

        let block_hash = rpc_client.get_latest_blockhash().await?;

        let instruction = solana_sdk::instruction::Instruction {
            program_id,
            accounts:update_accounts,
            data:update_data,
        };

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
        //     bincode::deserialize::<solana_sdk::transaction::VersionedTransaction>(&output[0]).unwrap();
        // info!("deser_tx_output: {:?}", deser_tx_output);


        // let sig = rpc_client
        //     .send_transaction(&deser_tx_output)
        //     .await?;

        // info!("sig: {:?}", sig);
    }

    Ok(())
}
