use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
// 初始化博客列表
pub struct Initialize<'info> {
    #[account(
        init,
        payer = signer,
        space = BlogList::INIT_SPACE,
    )]
    pub blog_list: Account<'info, BlogList>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
