use crate::{state::*, TitaErrorCode};

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

#[derive(Accounts)]
#[instruction(campaign_id: String)]
pub struct CreateCampaign<'info> {
    #[account(mut)]
    pub grant_provider: Signer<'info>,

    #[account(
        init,
        payer = grant_provider,
        space = 8 + GrantCampaign::INIT_SPACE,
        seeds = [
            campaign_id.as_bytes(),
            &grant_provider.key().to_bytes(),
        ],
        bump
    )]
    pub grant_campaign: Account<'info, GrantCampaign>,
    
    #[account(
        init,
        payer = grant_provider,
        seeds = [
            grant_campaign.key().as_ref(),
            token_mint.key().as_ref()
        ],
        bump,
        token::mint = token_mint,
        token::authority = grant_campaign,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateCampaign<'info> {
    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        campaign_id: String,
        total_funding: u64,
        deadline: Option<i64>,
        bumps: u8
    ) -> Result<()> {
        let campaign = &mut ctx.accounts.grant_campaign;
        let clock = Clock::get()?;

        campaign.total_funding = total_funding;
        campaign.remaining_funding = total_funding;
        campaign.is_active = true;
        campaign.created_at = clock.unix_timestamp;
        campaign.updated_at = clock.unix_timestamp;
        campaign.deadline = deadline;
        campaign.bump = bumps;
        campaign.grant_provider = ctx.accounts.grant_provider.key();
        campaign.campaign_id = campaign_id;

        Ok(())
    }
}
