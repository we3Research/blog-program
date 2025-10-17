use crate::error::BlogError;
use crate::state::author::Author;
use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
// 更新博客
pub struct UpdateBlog<'info> {
    #[account(mut)]
    pub blog_metadata: Account<'info, BlogMetadata>,
    #[account(mut)]
    pub author: Account<'info, Author>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn update_blog(ctx: Context<UpdateBlog>, new_title: String, new_content: String) -> Result<()> {
    let author = &mut ctx.accounts.author;
    let signer = &mut ctx.accounts.signer;

    require!(author.updater == signer.key(), BlogError::Unauthorized);

    let blog_metadata = &mut ctx.accounts.blog_metadata;

    require!(
        blog_metadata.author == author.key(),
        BlogError::Unauthorized
    );

    // Update content and timestamps
    blog_metadata.cid = new_content.clone();
    blog_metadata.title = new_title.clone();
    blog_metadata.update_at = Clock::get()?.unix_timestamp;
    blog_metadata.history.push(new_content);

    msg!("Updated blog: {}", blog_metadata.title);
    Ok(())
}
