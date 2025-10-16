use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::BlogError;

#[derive(Accounts)]
// 更新博客
pub struct UpdateBlog<'info> {
    #[account(mut)]
    pub blog_metadata: Account<'info, BlogMetadata>,
    #[account(mut)]
    pub signer: Signer<'info>,
}


pub fn update_blog(ctx : Context<UpdateBlog>, new_content: String)->Result<()>{
    let blog_metadata = &mut ctx.accounts.blog_metadata;

    // Ensure the signer is the creator of the blog
    require!(
            blog_metadata.owner == ctx.accounts.signer.key(),
            BlogError::Unauthorized
        );

    // Update content and timestamps
    blog_metadata.cid = new_content.clone();
    blog_metadata.update_at = Clock::get()?.unix_timestamp;
    blog_metadata.history.push(new_content);

    msg!("Updated blog: {}", blog_metadata.title);
    Ok(())
}