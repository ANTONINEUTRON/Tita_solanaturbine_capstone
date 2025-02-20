use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateCampaign<'info> {
    #[account(
        mut,
        constraint = grant_campaign.grant_provider == grant_provider.key()
    )]
    pub grant_campaign: Account<'info, GrantCampaign>,

    #[account(mut)]
    pub grant_provider: Signer<'info>,
}

impl<'info> UpdateCampaign<'info> {
    pub fn update_campaign(
        ctx: Context<UpdateCampaign>,
        total_funding: u64,
        is_active: bool,
    ) -> Result<()> {
        let campaign = &mut ctx.accounts.grant_campaign;
        let clock = Clock::get()?;

        campaign.total_funding = total_funding;
        campaign.remaining_funding = total_funding;
        campaign.is_active = is_active;
        campaign.updated_at = clock.unix_timestamp;

        // // Update total funding if provided
        // if let Some(new_funding) = new_total_funding {
        //     require!(
        //         new_funding >= campaign.total_funding - campaign.remaining_funding,
        //         CustomError::InvalidFundingUpdate
        //     );

        //     let funding_difference = new_funding.checked_sub(campaign.total_funding)
        //         .ok_or(CustomError::CalculationError)?;

        //     campaign.remaining_funding = campaign.remaining_funding.checked_add(funding_difference)
        //         .ok_or(CustomError::CalculationError)?;

        //     campaign.total_funding = new_funding;
        // }

        // // Update active status if provided
        // if let Some(active_status) = is_active {
        //     campaign.is_active = active_status;
        // }

        campaign.updated_at = clock.unix_timestamp;

        Ok(())
    }
}
