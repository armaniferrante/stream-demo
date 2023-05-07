use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, FreezeAccount, Mint, MintTo, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod stream {
    use super::*;

    ///
    /// create_mint initializes a fungeible token and assigns the mint and
    /// freeze authority to the program (some PDA).
    ///
    ///
    pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
        let mut stream_authority = &mut ctx.accounts.stream_authority;
        stream_authority.bump = *ctx.bumps.get("stream_authority").unwrap();
        Ok(())
    }

    ///
    /// mint_to will mint a new fungible token to  the given user wallet.
    /// If the user already has a token amount > 0, then abort.
    ///
    pub fn mint_to_self(ctx: Context<MintToSelf>) -> Result<()> {
        if ctx.accounts.token.amount != 0 {
            return Err(ErrorCode::AlreadyMinted.into());
        }

        //
        // Mint the token.
        //
        token::mint_to(
            ctx.accounts
                .mint_to_ctx()
                .with_signer(&[&["idk".as_bytes(), &[ctx.accounts.stream_authority.bump]]]),
            1,
        )?;

        //
        // Freeze the token.
        //
        token::freeze_account(
            ctx.accounts
                .freeze_account_ctx()
                .with_signer(&[&["idk".as_bytes(), &[ctx.accounts.stream_authority.bump]]]),
        )?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(
				init,
				payer = payer,
				space = 8+ StreamAuthority::LEN,
				seeds = ["idk".as_bytes()],
				bump,
		)]
    pub stream_authority: Account<'info, StreamAuthority>,
    #[account(
        init,
        payer = payer,
        mint::authority = stream_authority,
        mint::freeze_authority = stream_authority,
        mint::decimals = 0,
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintToSelf<'info> {
    #[account(
        init_if_needed,
        payer = payer,
				associated_token::authority = payer,
        associated_token::mint = mint,
    )]
    pub token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub stream_authority: Account<'info, StreamAuthority>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> MintToSelf<'info> {
    pub fn mint_to_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.token.to_account_info(),
            authority: self.stream_authority.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }

    pub fn freeze_account_ctx(&self) -> CpiContext<'_, '_, '_, 'info, FreezeAccount<'info>> {
        let program = self.token_program.to_account_info();
        let accounts = FreezeAccount {
            account: self.token.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.stream_authority.to_account_info(),
        };
        CpiContext::new(program, accounts)
    }
}

#[account]
pub struct StreamAuthority {
    //
    pub bump: u8,
}

impl StreamAuthority {
    pub const LEN: usize = 8;
}

#[error_code]
pub enum ErrorCode {
    AlreadyMinted,
}
