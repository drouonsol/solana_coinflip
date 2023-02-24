use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
declare_id!("26Kzq7jgtst8BfT6oSWCxR1Di8JNBpZs8sytTgq1tik3");

#[program]
pub mod coinflip_new {
    use anchor_lang::{solana_program::native_token::LAMPORTS_PER_SOL, accounts::signer};

    use super::*;

    pub fn initialize(ctx: Context<Flip>,amount: i32) -> Result<()> {
        require!(
            amount >= 5,
            CoinEror::BidLessThanMinumum
        );

        msg!("Initiliazed");
        
        let amount: u64 = (amount as u64) * LAMPORTS_PER_SOL / 100 ;
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.player.key(),
            &ctx.accounts.escrow_account.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.player.to_account_info(),
                ctx.accounts.escrow_account.to_account_info(),
            ],
        );



        let fees: u64 = (amount as u64)  / 666 * 10 ;
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.player.key(),
            &ctx.accounts.fee_wallet.key(),
            fees,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.player.to_account_info(),
                ctx.accounts.fee_wallet.to_account_info(),
            ],
        );

        let clock = Clock::get()?;
        let result = clock.unix_timestamp % 2;
        let mut amount_given  = (amount);
       

        ctx.accounts.player_account.games += 1;
        ctx.accounts.player_account.last_flip_claimed = false;


        if result >= 1 {
            let    amount_given = (amount_given) * 2;
            ctx.accounts.player_account.tokens_owed = amount_given;
      
            ctx.accounts.player_account.gameswon += 1;


        } else {
            amount_given = 0;
            msg!("User Has Lost,Sending Bet Worth: {}", amount_given);
            ctx.accounts.player_account.gameswon -= 1;
            ctx.accounts.player_account.tokens_owed = 0;
        }
        let percantage_won =   ctx.accounts.player_account.gameswon / ctx.accounts.player_account.games * 100;
        msg!("Win Percantage : {} %", percantage_won);

        Ok(())
    }

    

    pub fn claim(ctx: Context<Claim>,bump: u8) -> Result<()> {
        
        msg!("Claiming Rewards");
            let pda = &mut ctx.accounts.player_account;
            let amount =  pda.tokens_owed;
            let binding = pda.to_account_info().key();
            let seeds = 
                &[  
                binding.as_ref(),
                 b"_",
                 b"elysian_flip",
                 
                ];
    
        //  Claiming of tokens

        **ctx.accounts.escrow_account.to_account_info().try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.player.try_borrow_mut_lamports()? += amount;
        
        // Settings Amount Owed To 0 
        ctx.accounts.player_account.tokens_owed = 0;

        


        Ok(())
    }

    pub fn withdraw(ctx: Context<WithdrawVault>, withdrawl_amount: i32) -> Result<()> {
        msg!("Withdrawing Vault ");
        let security_account = &mut ctx.accounts.auth_wallet;
        security_account.times_counted += 1;
        if security_account.times_counted <= 1 {
            msg!("Setting Authority");
            security_account.authority = ctx.accounts.player.to_account_info().key()
        } else {
            msg!("Authority Wallet Already Initiliazted")
        }

        if security_account.authority == ctx.accounts.player.to_account_info().key() {
            msg!("Signer Matches Authority. Withdrawing Set Amount");
            let amount = (withdrawl_amount as u64) * LAMPORTS_PER_SOL;
            **ctx.accounts.escrow_account.to_account_info().try_borrow_mut_lamports()? -= amount;
             **ctx.accounts.player.try_borrow_mut_lamports()? += amount;
        } else {
            return(err!(CoinEror::BidLessThanMinumum));
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Flip<'info> {
    #[account(
        init_if_needed,
        payer = player, 
        space = std::mem::size_of::<FlipStats>() + 8,
        seeds = [
            player.key().as_ref(),
            b"_",
            b"elysian_flip",
        ],
        bump
    )] 
    pub player_account: Account<'info, FlipStats>, 
    #[account(
        init_if_needed,
        payer = player, 
        space = 10,
        seeds = [
            b"escrowwallet",
        ],
        bump
    )]
     /// CHECK: The account is safe
    pub escrow_account: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = player, 
        space = 10,
        seeds = [
            b"feewallet",
        ],
        bump
    )]
    /// CHECK: The account is safe
    pub fee_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info,  System>,
    
}

//Let's User Withdraw Winnings and Adds to Stats
#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(

        seeds = [
            player.key().as_ref(),
            b"_",
            b"elysian_flip",  
        ],
        bump
    )]

    pub player_account: Account<'info, FlipStats>, 
    #[account(
        init_if_needed,
        payer = player, 
        space = 10,
        seeds = [
            b"escrowwallet",
        ],
        bump
    )]
     /// CHECK: The account is safe
    pub escrow_account: AccountInfo<'info>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info,  System>,

}


#[derive(Accounts)]
pub struct WithdrawVault<'info> {
    #[account(
        init_if_needed,
        payer = player, 
        space = 10,
        seeds = [
            b"escrowwallet",
        ],
        bump
    )]
     /// CHECK: The account is safe
    pub escrow_account: AccountInfo<'info>,
    #[account(
        init_if_needed,
        payer = player, 
        space = 100,
        seeds = [
            b"auth_wallet",
        ],
        bump
    )]
     /// CHECK: The account is safe
    pub auth_wallet: Account<'info, Authority>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info,  System>,

}


// Let's User Withdraw Rewards
#[account]
pub struct FlipStats{
    games: i16,
    gameswon: i16,
    gameslost: i16,
    tokens_owed: u64,
    last_flip_value: i64,
    last_flip_claimed: bool,
}

#[account]
pub struct Authority {
    times_counted: u8,
    authority: Pubkey,
}


#[derive(Debug, PartialEq, AnchorDeserialize, AnchorSerialize, Clone)]
pub enum CoinResult {
    Won,
    Lost,
}


pub enum ClaimStatus {
    Claimed,
    Unclaimed
}


#[error_code]
pub enum CoinEror {
    #[msg("Not Enough Wallet Balance")]
    EscrowBalanceLow, 
    #[msg("Bid Out Of Range")]
    OutOfRange,
    #[msg("Bid Too Low")]
    BidLessThanMinumum,
    #[msg("Not Enough Too Claim")]
    NotEnoughToClaim
}