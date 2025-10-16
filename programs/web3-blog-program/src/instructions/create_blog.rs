use crate::state::blog_list::BlogList;
use crate::state::blog_metadata::BlogMetadata;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(title: String,author: String,cid: String)]
pub struct CreateBlog<'info> {
    #[account(
        init,
        payer = signer,
        space = BlogMetadata::INIT_SPACE,
        seeds = [
            b"blog_metadata",
            cid.as_bytes(),
            signer.key().as_ref(),
         ],
        bump,
    )]
    pub blog_metadata: Account<'info, BlogMetadata>,
    #[account(
        init_if_needed,
        payer = signer,
        space = BlogList::INIT_SPACE,
        seeds = [
            b"blog_list",
            signer.key().as_ref(),
         ],
        bump,
    )]
    pub blog_list: Account<'info, BlogList>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_blog(
    ctx: Context<CreateBlog>,
    title: String,
    author: String,
    cid: String,
) -> Result<()> {
    let blog_list = &mut ctx.accounts.blog_list;
    let blog_metadata = &mut ctx.accounts.blog_metadata;
    let signer = &ctx.accounts.signer;

    if !blog_list.is_initialized {
        blog_list.list = Vec::new();
        blog_list.is_initialized = true;
    }

    blog_list.list.push(blog_metadata.key()); // Add the new blog's pubkey to the list

    // 存储博客元数据
    blog_metadata.title = title;
    blog_metadata.author = author.clone();
    blog_metadata.history = vec![cid.clone()]; // Store initial content in history
    blog_metadata.cid = cid;
    blog_metadata.create_at = Clock::get()?.unix_timestamp;
    blog_metadata.update_at = blog_metadata.create_at;
    blog_metadata.owner = signer.key(); // Save the creator's public key

    msg!("Created blog: {}", blog_metadata.title);
    Ok(())
}
