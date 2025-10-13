use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
// 更新博客
pub struct UpdateBlog<'info> {
    #[account(mut)]
    pub blog_metadata: Account<'info, BlogMetadata>,
    #[account(mut)]
    pub signer: Signer<'info>,
}