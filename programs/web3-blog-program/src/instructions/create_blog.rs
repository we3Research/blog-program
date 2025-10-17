use crate::error::BlogError;
use crate::state::author::Author;
use crate::state::blog_metadata::BlogMetadata;
use anchor_lang::prelude::*;

const SEED:&[u8] = b"blog";

#[derive(Accounts)]
#[instruction(index: i64)]
pub struct CreateBlog<'info> {
    #[account(
        init,
        payer = signer,
        space = 8+BlogMetadata::INIT_SPACE,
        seeds = [
            SEED,
            author.key().as_ref(),
            index.to_be_bytes().as_ref(),
         ],
        bump,
    )]
    pub blog_metadata: Account<'info, BlogMetadata>,
    #[account(mut)]
    pub author: Account<'info, Author>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create_blog(
    ctx: Context<CreateBlog>,
    index: i64,
    //author: Pubkey,
    title: String,
    cid: String,
) -> Result<()> {
    let author_account = &mut ctx.accounts.author;
    let signer = &ctx.accounts.signer;

    require!(
        author_account.updater == signer.key(),
        BlogError::Unauthorized
    );
    author_account.total += 1;

    require!(author_account.total == index, BlogError::IndexWrong);

    let blog_metadata = &mut ctx.accounts.blog_metadata;

    let ns = Clock::get()?.unix_timestamp;
    // 存储博客元数据
    blog_metadata.set_inner(BlogMetadata {
        index,
        title,
        cid: cid.clone(),
        create_at: ns,
        update_at: ns,
        history: vec![cid.clone()],
        author: author_account.key(),
    });
    Ok(())
}
