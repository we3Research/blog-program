use crate::state::*;

use anchor_lang::prelude::*;

#[derive(Accounts)]
// 创建博客
pub struct CreateBlog<'info> {
    #[account(
        init,
        payer = signer,
        space = BlogMetadata::INIT_SPACE
    )]
    pub blog_metadata: Account<'info, BlogMetadata>,
    #[account(mut)]
    pub blog_list: Account<'info, BlogList>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
