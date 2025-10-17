use crate::AuthorList;
use anchor_lang::prelude::*;

pub const SEED: &[u8] = b"author_list";

#[derive(Accounts)]
// 更新博客
pub struct Init<'info> {
    #[account(
        init,
        payer = signer,
        space = 8+AuthorList::INIT_SPACE,
        seeds = [
            SEED,
         ],
        bump,
    )]
    pub author_list: Account<'info, AuthorList>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init(ctx: Context<Init>) -> Result<()> {
    let author_list = &mut ctx.accounts.author_list;
    author_list.updater = ctx.accounts.signer.key();
    author_list.total = 0;
    Ok(())
}
