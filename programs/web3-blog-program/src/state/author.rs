use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
// 博客列表
pub struct Author {
    pub uid: i64,
    #[max_len(20)]
    pub pseudonym: String,
    #[max_len(1000)]
    pub introduction: String,
    pub total: i64,
    pub updater:Pubkey,
}
