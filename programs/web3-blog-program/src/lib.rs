mod error;
mod instructions;
pub mod state;

use anchor_lang::prelude::*;


use instructions::*;

pub use state::*;

// This is your program's public key and it will update automatically when you build the project.
declare_id!("D1qcfn5Eevy8LCoAb3vrCTrDzNzDYqi7spG3ja3BgFJg");

#[program]
mod blog {
    use super::*;
    
    pub fn create_blog(
        ctx: Context<CreateBlog>,
        title: String,
        author: String,
        cid: String,
    ) -> Result<()> {
        instructions::create_blog(ctx, title, author, cid)
    }

    pub fn update_blog(ctx: Context<UpdateBlog>, new_content: String) -> Result<()> {
        instructions::update_blog(ctx, new_content)
    }
}
