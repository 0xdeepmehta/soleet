use anchor_lang::prelude::*;
use anchor_lang::solana_program::system_program;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod sol_tweet {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>, content: String) -> ProgramResult {
        let tweet_account: &mut Account<TweetAccount> = &mut ctx.accounts.tweet_account;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();
        
        // validate content lenght remain under 280 chars
        if content.chars().count() > 280 {  
            return Err(ErrorCode::ContentTooLong.into())
        }

        tweet_account.author = *author.key;
        tweet_account.timestamp = clock.unix_timestamp;
        tweet_account.content = content;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = author, space = TweetAccount::LEN)]
    pub tweet_account: Account<'info, TweetAccount>,
    #[account(mut)] // going to mutate the amont of the money.
    pub author: Signer<'info>,
    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>
}


#[account]
pub struct TweetAccount {
    pub author: Pubkey,
    pub timestamp: i64,
    pub content: String,
}

// 2. Add some useful constants for sizing propeties.
const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max.

impl TweetAccount {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH
        + TIMESTAMP_LENGTH
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH; 
}


// handle possible errors
#[error]
pub enum ErrorCode {
    #[msg("The Provide content length should be 280 characters long maximum.")]
    ContentTooLong
}