use crate::error::BlogError;
use crate::state::author::Author;
use anchor_lang::prelude::*;

#[derive(Accounts)]
// 更新博客
#[instruction(uid:i64)]
pub struct UpdateAuthor<'info> {
    #[account(mut)]
    pub author: Account<'info, Author>,
    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn update_author(
    ctx: Context<UpdateAuthor>,
    //uid: i64,
    //pseudonym: String,
    introduction: String,
    updater: Pubkey,
) -> Result<()> {
    // 作者信息修改权限
    let author =&mut ctx.accounts.author;
    require!(author.updater == ctx.accounts.signer.key(), BlogError::Unauthorized);


    // 作者信息创建
    author.updater = updater;
    //author.pseudonym = pseudonym;
    author.introduction = introduction;

    Ok(())
}
