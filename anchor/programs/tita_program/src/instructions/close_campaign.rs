use crate::{state::*, TitaErrorCode};
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{close_account, transfer, Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
pub struct CloseCampaign<'info> {
    #[account(
        mut,
        close = grant_provider,
        constraint = grant_campaign.grant_provider == grant_provider.key(),
        constraint = grant_campaign.is_active,
        seeds = [
            grant_campaign.campaign_id.as_bytes(),
            &grant_provider.key().to_bytes(),
        ],
        bump = grant_campaign.bump
    )]
    pub grant_campaign: Account<'info, GrantCampaign>,

    #[account(
        mut,
        seeds = [
            grant_campaign.key().as_ref(),
            token_mint.key().as_ref()
        ],
        bump,
        token::mint = token_mint,
        token::authority = grant_campaign
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        constraint = receiver.owner == grant_provider.key(),
        constraint = receiver.mint == vault.mint
    )]
    pub receiver: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: InterfaceAccount<'info, Mint>,
    
    #[account(mut)]
    pub grant_provider: Signer<'info>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CloseCampaign<'info> {
    pub fn close_campaign(ctx: Context<CloseCampaign>) -> Result<()> {
        let remaining_amount = ctx.accounts.vault.amount;
        require!(remaining_amount > 0, TitaErrorCode::NoRemainingFunds);

        let seeds = &[
            ctx.accounts.grant_campaign.campaign_id.as_bytes(),
            &ctx.accounts.grant_provider.key().to_bytes(),
            &[ctx.accounts.grant_campaign.bump],
        ];
        let signer = &[&seeds[..]];

        // Transfer remaining funds
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.receiver.to_account_info(),
                    authority: ctx.accounts.grant_campaign.to_account_info(),
                },
                signer
            ),
            remaining_amount
        )?;

        // Close vault
        close_account(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                CloseAccount {
                    account: ctx.accounts.vault.to_account_info(),
                    destination: ctx.accounts.grant_provider.to_account_info(),
                    authority: ctx.accounts.grant_campaign.to_account_info(),
                },
                signer
            )
        )?;

        Ok(())
    }
}

