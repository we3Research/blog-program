mod error;
mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use state::*;

use instructions::*;

// This is your program's public key and it will update automatically when you build the project.
declare_id!("AGRJdaV5t2rKK7nTYFQiRztu8tKkKgoXMprTECZrLc7A");

#[program]
mod blog {
    use super::*;

    pub fn init(ctx: Context<Init>) -> Result<()> {
        init::init(ctx)
    }

    pub fn add_author(
        ctx: Context<AddAuthor>,
        uid: i64,
        pseudonym: String,
        introduction: String,
        updater: Pubkey,
    ) -> Result<()> {
        instructions::add_author(ctx, uid, pseudonym, introduction, updater)
    }

    pub fn update_author(
        ctx: Context<UpdateAuthor>,
        //uid: i64,
        //pseudonym: String,
        introduction: String,
        updater: Pubkey,
    ) -> Result<()> {
        instructions::update_author(ctx, introduction, updater)
    }

    pub fn create_blog(
        ctx: Context<CreateBlog>,
        index: i64,
        //author: Pubkey,
        title: String,
        cid: String,
    ) -> Result<()> {
        instructions::create_blog(ctx, index, title, cid)
    }

    pub fn update_blog(
        ctx: Context<UpdateBlog>,
        new_title: String,
        new_content: String,
    ) -> Result<()> {
        instructions::update_blog(ctx, new_title, new_content)
    }
}
