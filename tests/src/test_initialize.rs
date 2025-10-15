use std::str::FromStr;

use anchor_client::solana_sdk::signer::Signer;
use anchor_client::{solana_sdk::{
    commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
}, Client, Cluster};
use web3_blog_program::BlogMetadata;
pub use web3_blog_program::state::blog_list::BlogList;

#[test]
fn test_create_blog() {
    let program_id = "D1qcfn5Eevy8LCoAb3vrCTrDzNzDYqi7spG3ja3BgFJg";
    let anchor_wallet = "/home/llf/.config/solana/id.json";
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let cid = "test".to_string();

    let (blog_metadata_addr, _b1) = Pubkey::find_program_address(
        &[b"blog_metadata", cid.as_bytes(), payer.pubkey().as_ref()],
        &program_id,
    );
    let (blog_list_addr, _b2) =
        Pubkey::find_program_address(&[b"blog_list", payer.pubkey().as_ref()], &program_id);
    println!("blog_metadata_addr: {}, blog_list_addr: {}", blog_metadata_addr, blog_list_addr);
    let tx = program
        .request()
        .accounts(web3_blog_program::accounts::CreateBlog {
            blog_metadata: blog_metadata_addr,
            blog_list: blog_list_addr,
            signer: payer.pubkey(),
            system_program: solana_system_interface::program::ID,
        })
        .args(web3_blog_program::instruction::CreateBlog {
            title: "test".to_string(),
            author: cid.to_string(),
            cid,
        })
        //.payer(&payer)
        .signer(&payer)
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}


#[test]
fn test_update_blog(){
    let program_id = "D1qcfn5Eevy8LCoAb3vrCTrDzNzDYqi7spG3ja3BgFJg";
    let anchor_wallet = "/home/llf/.config/solana/id.json";
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let cid = "test".to_string();

    let (blog_metadata_addr, _b1) = Pubkey::find_program_address(
        &[b"blog_metadata", cid.as_bytes(), payer.pubkey().as_ref()],
        &program_id,
    );
    let (blog_list_addr, _b2) =
        Pubkey::find_program_address(&[b"blog_list", payer.pubkey().as_ref()], &program_id);


    println!("blog_metadata_addr: {}, blog_list_addr: {}", blog_metadata_addr, blog_list_addr);
    let tx = program
        .request()
        .accounts(web3_blog_program::accounts::UpdateBlog {
            blog_metadata: blog_metadata_addr,
            //blog_list: blog_list_addr,
            signer: payer.pubkey(),
            //system_program: solana_system_interface::program::ID,
        })
        .args(web3_blog_program::instruction::UpdateBlog {
            new_content: "test121".to_string(),
        })
        //.payer(&payer)
        .signer(&payer)
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}



#[test]
fn test_update_fentch(){
    let program_id = "D1qcfn5Eevy8LCoAb3vrCTrDzNzDYqi7spG3ja3BgFJg";
    let anchor_wallet = "/home/llf/.config/solana/id.json";
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let cid = "test".to_string();

    let (blog_metadata_addr, _b1) = Pubkey::find_program_address(
        &[b"blog_metadata", cid.as_bytes(), payer.pubkey().as_ref()],
        &program_id,
    );
    let (blog_list_addr, _b2) =
        Pubkey::find_program_address(&[b"blog_list", payer.pubkey().as_ref()], &program_id);


    println!("blog_metadata_addr: {}, blog_list_addr: {}", blog_metadata_addr, blog_list_addr);
    let result = program.account::<BlogList>(blog_list_addr).unwrap();
    println!("result: {:?}", result.list);
    let result = program.account::<BlogMetadata>(blog_metadata_addr).unwrap();
    println!("result: {:?}", result.history);
}
