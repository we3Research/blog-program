use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace,Debug)]
// 博客元数据
pub struct BlogMetadata {
    pub index: i64,
    #[max_len(50)]
    pub title: String,
    #[max_len(50)]
    pub cid: String,
    pub create_at: i64,
    pub update_at: i64,
    #[max_len(50, 100)]
    pub history: Vec<String>,
    pub author: Pubkey,
}
