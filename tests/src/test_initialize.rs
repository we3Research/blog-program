use std::str::FromStr;

use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
    },
    Client, Cluster,
};
use anchor_client::solana_sdk::signature::Keypair;
use anchor_client::solana_sdk::signer::Signer;

#[test]
fn test_initialize() {
    let program_id = "D1qcfn5Eevy8LCoAb3vrCTrDzNzDYqi7spG3ja3BgFJg";
    //let program_id = "BdmuS51J646B3C5ECQrjfRNmrmp7So3vQFiTHwVHya2h";
    let anchor_wallet = "/home/llf/.config/solana/id.json";
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let new_acc = Keypair::new();

    let tx = program
        .request()
        .accounts(web3_blog_program::accounts::Initialize {
            blog_list: new_acc.pubkey(),
            signer: payer.pubkey(),
            system_program: solana_system_interface::program::ID,
        })
        .args(web3_blog_program::instruction::Initialize {})
        //.payer(&payer)
        .signer(&payer)
        .signer(new_acc)
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}
