use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
// 博客列表
pub struct AuthorList {
    pub total: i64,
    pub updater: Pubkey,
}
