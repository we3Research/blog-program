use crate::error::BlogError;
use crate::state::author::Author;
use crate::AuthorList;
use anchor_lang::prelude::*;


const SEED :&[u8] = b"author";
#[derive(Accounts)]
// 更新博客
#[instruction(uid:i64)]
pub struct AddAuthor<'info> {
    #[account(
        init,
        payer = signer,
        space = 8+Author::INIT_SPACE,
        seeds = [
            SEED,
            uid.to_be_bytes().as_ref(),
         ],
        bump,
    )]
    pub author: Account<'info, Author>,
    #[account(mut)]
    pub author_list: Account<'info, AuthorList>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn add_author(
    ctx: Context<AddAuthor>,
    uid: i64,
    pseudonym: String,
    introduction: String,
    updater: Pubkey,
) -> Result<()> {
    // 作者列表地址
    let (author_list_addr, _) = Pubkey::find_program_address(&[b"author_list"], ctx.program_id);
    require!(
        author_list_addr == ctx.accounts.author_list.key(),
        BlogError::AuthorListNotInit
    );

    // 作者列表修改权限
    let author_list = &mut ctx.accounts.author_list;
    require!(
        author_list.updater == ctx.accounts.signer.key(),
        BlogError::Unauthorized
    );
    author_list.total += 1;
    require!(author_list.total == uid, BlogError::IndexWrong);

    // 作者信息创建
    let author = &mut ctx.accounts.author;
    author.uid = uid;
    author.total = 0;
    author.updater = updater;
    author.pseudonym = pseudonym;
    author.introduction = introduction;

    Ok(())
}
