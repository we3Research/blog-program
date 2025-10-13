mod error;
mod instructions;
mod state;

use anchor_lang::prelude::*;
use error::BlogError;

use instructions::*;

// This is your program's public key and it will update automatically when you build the project.
declare_id!("D1qcfn5Eevy8LCoAb3vrCTrDzNzDYqi7spG3ja3BgFJg");

#[program]
mod blog {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Initialize the BlogList account
        let blog_list = &mut ctx.accounts.blog_list;
        blog_list.list = Vec::new(); // Initialize the list as empty
        msg!("Initialized BlogList!");
        Ok(())
    }

    pub fn create_blog(
        ctx: Context<CreateBlog>,
        title: String,
        author: String,
        content: String,
    ) -> Result<()> {
        // 存储博客列表
        let blog_list = &mut ctx.accounts.blog_list;
        blog_list.list.push(ctx.accounts.blog_metadata.key()); // Add the new blog's pubkey to the list

        // 存储博客元数据
        let blog_metadata = &mut ctx.accounts.blog_metadata;
        blog_metadata.title = title;
        blog_metadata.author = author.clone();
        blog_metadata.history = vec![content.clone()]; // Store initial content in history
        blog_metadata.content = content;
        blog_metadata.create_at = Clock::get()?.unix_timestamp;
        blog_metadata.update_at = blog_metadata.create_at;
        blog_metadata.owner = ctx.accounts.signer.key(); // Save the creator's public key

        msg!("Created blog: {}", blog_metadata.title);
        Ok(())
    }

    pub fn update_blog(ctx: Context<UpdateBlog>, new_content: String) -> Result<()> {
        let blog_metadata = &mut ctx.accounts.blog_metadata;

        // Ensure the signer is the creator of the blog
        require!(
            blog_metadata.owner == ctx.accounts.signer.key(),
            BlogError::Unauthorized
        );

        // Update content and timestamps
        blog_metadata.content = new_content.clone();
        blog_metadata.update_at = Clock::get()?.unix_timestamp;
        blog_metadata.history.push(new_content);

        msg!("Updated blog: {}", blog_metadata.title);
        Ok(())
    }
}










