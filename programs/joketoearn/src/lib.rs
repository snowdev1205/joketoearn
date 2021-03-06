use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod joketoearn {
    use super::*;

    pub fn create_joke(
        ctx: Context<CreateJokeCtx>, 
        joke_content: String,
    ) -> Result<()> {
        
        let joke: &mut Account<Joke> = &mut ctx.accounts.joke_account;
        !msg("hello joke");
        joke.author = *ctx.accounts.authority.key;
        joke.content = joke_content;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateJokeCtx<'info> {
    #[account(init, payer = authority, space = 2000)]
    pub joke_account: Account<'info, Joke>,

    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Safe
    #[account(address = anchor_lang::solana_program::system_program::ID)]
    pub system_program: AccountInfo<'info>
}


#[account]
pub struct Joke {
    pub author: Pubkey,
    pub content: String,
}