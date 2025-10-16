use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace,Debug)]
// 博客列表
pub struct BlogList {
    #[max_len(100)] // 目前列表长度为100 todo 优化为无限长的列表
    pub list: Vec<Pubkey>,
    pub is_initialized:bool,
}