use crate::{state::*, TitaErrorCode};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(campaign_id: String)]
pub struct CreateCampaign<'info> {
    #[account(
        init,
        payer = grant_provider,
        space = 8 + GrantCampaign::INIT_SPACE,
        seeds = [b"campaign", campaign_id.as_bytes()],
        bump
    )]
    pub grant_campaign: Account<'info, GrantCampaign>,

    #[account(mut)]
    pub grant_provider: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateCampaign<'info> {
    pub fn create_campaign(
        ctx: Context<CreateCampaign>,
        campaign_id: String,
        total_funding: u64,
        bumps: u8
    ) -> Result<()> {
        let campaign = &mut ctx.accounts.grant_campaign;
        let clock = Clock::get()?;

        campaign.total_funding = total_funding;
        campaign.remaining_funding = total_funding;
        campaign.is_active = true;
        campaign.created_at = clock.unix_timestamp;
        campaign.updated_at = clock.unix_timestamp;
        campaign.bump = bumps;
        campaign.grant_provider = ctx.accounts.grant_provider.key();
        campaign.campaign_id = campaign_id;

        Ok(())
    }
}
