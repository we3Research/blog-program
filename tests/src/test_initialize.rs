use std::str::FromStr;

use anchor_client::solana_sdk::signer::Signer;
use anchor_client::{
    solana_sdk::{
        commitment_config::CommitmentConfig, pubkey::Pubkey, signature::read_keypair_file,
    },
    Client, Cluster,
};
use web3_blog_program::*;

const PROGRAM_ID: &str = "AGRJdaV5t2rKK7nTYFQiRztu8tKkKgoXMprTECZrLc7A";
const ANCHOR_WALLET: &str = "/home/llf/.config/solana/id.json";

#[test]
fn test_init() {
    let program_id = PROGRAM_ID;
    let anchor_wallet = ANCHOR_WALLET;
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();
    let (author_list_pubkey, _b) = Pubkey::find_program_address(&[b"author_list"], &program_id);
    println!("author_list_pubkey: {}", author_list_pubkey);

    let sig = program
        .request()
        .accounts(accounts::Init {
            author_list: author_list_pubkey,
            signer: payer.pubkey(),
            system_program: solana_system_interface::program::ID,
        })
        .args(instruction::Init {})
        .signer(&payer)
        //.payer(&payer)
        .send()
        .expect("");

    println!("Your transaction signature {}", sig);

    let author_list = program.account::<AuthorList>(author_list_pubkey).unwrap();

    println!("author_list: {:?}", author_list)
}

#[test]
fn test_add_author() {
    let program_id = PROGRAM_ID;
    let anchor_wallet = ANCHOR_WALLET;
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let (author_list_pubkey, _b) = Pubkey::find_program_address(&[b"author_list"], &program_id);
    println!("author_list_pubkey: {}", author_list_pubkey);

    let uid: i64 = 1;
    let (author_pubkey, _b) =
        Pubkey::find_program_address(&[b"author", uid.to_be_bytes().as_ref()], &program_id);
    println!("author_pubkey: {}", author_list_pubkey);

    let sig = program
        .request()
        .accounts(accounts::AddAuthor {
            author: author_pubkey,
            author_list: author_list_pubkey,
            signer: payer.pubkey(),
            system_program: solana_system_interface::program::ID,
        })
        .args(instruction::AddAuthor {
            uid,
            pseudonym: "Solitary Beluga".to_string(),
            introduction: "一只平平无奇的鲸鱼".to_string(),
            updater: payer.pubkey(),
        })
        .signer(&payer)
        //.payer(&payer)
        .send()
        .expect("");

    println!("Your transaction signature {}", sig);

    let author_list = program.account::<AuthorList>(author_list_pubkey).unwrap();

    println!("author_list: {:?}", author_list);

    let author = program.account::<Author>(author_pubkey).unwrap();

    println!("author_list: {:?}", author)
}

#[test]
fn test_add_blog() {
    let program_id = PROGRAM_ID;
    let anchor_wallet = ANCHOR_WALLET;
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    /*let (author_list_pubkey, _b) = Pubkey::find_program_address(&[b"author_list"], &program_id);
    println!("author_list_pubkey: {}", author_list_pubkey);*/

    let uid: i64 = 1;
    let (author_pubkey, _b) =
        Pubkey::find_program_address(&[b"author", uid.to_be_bytes().as_ref()], &program_id);
    println!("author_pubkey: {}", author_pubkey);

    let index: i64 = 1;
    let (blog_metadata_pubkey, _b1) = Pubkey::find_program_address(
        &[
            b"blog",
            author_pubkey.as_ref(),
            index.to_be_bytes().as_ref(),
        ],
        &program_id,
    );

    let sig = program
        .request()
        .accounts(accounts::CreateBlog {
            blog_metadata: blog_metadata_pubkey,
            author: author_pubkey,
            signer: payer.pubkey(),
            system_program: solana_system_interface::program::ID,
        })
        .args(instruction::CreateBlog {
            index,
            title: "Web3 博客".to_string(),
            cid: "QmPgBseimNtHiFufHh213jkVNMew9wMsEiGZWXzs6KLkJb".to_string(),
        })
        .signer(&payer)
        //.payer(&payer)
        .send()
        .expect("");

    println!("Your transaction signature {}", sig);

    /*let author_list = program.account::<AuthorList>(author_list_pubkey).unwrap();

    println!("author_list: {:?}", author_list);*/

    let author = program.account::<Author>(author_pubkey).unwrap();

    println!("author_list: {:?}", author);


    let blog_metadata = program.account::<BlogMetadata>(blog_metadata_pubkey).unwrap();

    println!("author_list: {:?}", blog_metadata);
}

#[test]
fn test_fetch() {
    let program_id = PROGRAM_ID;
    let anchor_wallet = ANCHOR_WALLET;
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();
    let (author_list_pubkey, _b) = Pubkey::find_program_address(&[b"author_list"], &program_id);

    let author_list = program.account::<AuthorList>(author_list_pubkey).unwrap();
    println!("author_list: {:?}", author_list);

    let uid: i64 = 1;
    let (author_pubkey, _b) =
        Pubkey::find_program_address(&[b"author", uid.to_be_bytes().as_ref()], &program_id);
    println!("author_pubkey: {}", author_list_pubkey);

    let author = program.account::<Author>(author_pubkey).unwrap();

    println!("author_list: {:?}", author);

    let index: i64 = 1;
    let (blog_metadata_pubkey, _b1) = Pubkey::find_program_address(
        &[
            b"blog",
            author_pubkey.as_ref(),
            index.to_be_bytes().as_ref(),
        ],
        &program_id,
    );
    let blog_metadata = program.account::<BlogMetadata>(blog_metadata_pubkey).unwrap();
    println!("author_list: {:?}", blog_metadata);
}
/*#[test]
fn test_create_blog() {
    let program_id = PROGRAM_ID;
    let anchor_wallet = ANCHOR_WALLET;
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let cid = "Qmbejuo7kNThaeRMjLQc3wh".to_string();

    let (blog_metadata_addr, _b1) = Pubkey::find_program_address(
        &[b"blog_metadata", cid.as_bytes(), payer.pubkey().as_ref()],
        &program_id,
    );
    let (blog_list_addr, _b2) =
        Pubkey::find_program_address(&[b"blog_list", payer.pubkey().as_ref()], &program_id);
    println!(
        "blog_metadata_addr: {}, blog_list_addr: {}",
        blog_metadata_addr, blog_list_addr
    );
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
        .payer(&payer)
        .signer(&payer)
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}

#[test]
fn test_update_blog() {
    let program_id = "D1qcfn5Eevy8LCoAb3vrCTrDzNzDYqi7spG3ja3BgFJg";
    let anchor_wallet = "/home/llf/.config/solana/id.json";
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let cid = "Qmbejuo7kNThaeRMjLQc3wh".to_string();

    let (blog_metadata_addr, _b1) = Pubkey::find_program_address(
        &[b"blog_metadata", cid.as_bytes(), payer.pubkey().as_ref()],
        &program_id,
    );
    let (blog_list_addr, _b2) =
        Pubkey::find_program_address(&[b"blog_list", payer.pubkey().as_ref()], &program_id);

    println!(
        "blog_metadata_addr: {}, blog_list_addr: {}",
        blog_metadata_addr, blog_list_addr
    );
    let tx = program
        .request()
        .accounts(web3_blog_program::accounts::UpdateBlog {
            blog_metadata: blog_metadata_addr,
            //blog_list: blog_list_addr,
            signer: payer.pubkey(),
            //system_program: solana_system_interface::program::ID,
        })
        .args(web3_blog_program::instruction::UpdateBlog {
            new_content: "Qmbejuo7kNThaeRMjLQc3whFKnp6c4N43m1XtPH4fKkRMt".to_string(),
        })
        //.payer(&payer)
        .signer(&payer)
        .send()
        .expect("");

    println!("Your transaction signature {}", tx);
}

#[test]
fn test_update_fentch() {
    let program_id = "D1qcfn5Eevy8LCoAb3vrCTrDzNzDYqi7spG3ja3BgFJg";
    let anchor_wallet = "/home/llf/.config/solana/id.json";
    let payer = read_keypair_file(&anchor_wallet).unwrap();

    let client = Client::new_with_options(Cluster::Devnet, &payer, CommitmentConfig::confirmed());
    let program_id = Pubkey::from_str(program_id).unwrap();
    let program = client.program(program_id).unwrap();

    let cid = "test".to_string();

    /*let (blog_metadata_addr, _b1) = Pubkey::find_program_address(
        &[b"blog_metadata", cid.as_bytes(), payer.pubkey().as_ref()],
        &program_id,
    );*/
    let (blog_list_addr, _b2) =
        Pubkey::find_program_address(&[b"blog_list", payer.pubkey().as_ref()], &program_id);
    println!("result: {:?}", blog_list_addr);
    let result = program.account::<BlogList>(blog_list_addr).unwrap();

    for blog_metadata_addr in result.list {
        println!("result: {:?}", blog_metadata_addr);
        let result = program.account::<BlogMetadata>(blog_metadata_addr).unwrap();
        println!("result: {:?}", result.history);
    }
}*/

/*type Solution;

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut m = std::collections::HashMap::with_capacity(min(nums.len(), value as usize));
        for x in nums {
            m.entry(((x % value) + value) % value)
                .and_modify(|y| *y += 1)
                .or_insert(1);
        }
        let mut res = 0;
        while m.entry(res % value). {
            res += 1;
            m.entry(res % value).and_modify(|y| *y -= 1);
        }

        res
    }
}*/
