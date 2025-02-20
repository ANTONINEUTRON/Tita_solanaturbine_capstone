use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct SubmitProof<'info> {
    #[account(
        mut,
        constraint = milestone.status == MilestoneStatus::InProgress,
        constraint = milestone.proposal == proposal.key()
    )]
    pub milestone: Account<'info, Milestone>,

    #[account(
        constraint = proposal.status == ProposalStatus::Approved,
        constraint = proposal.applicant == applicant.key()
    )]
    pub proposal: Account<'info, Proposal>,

    #[account(mut)]
    pub applicant: Signer<'info>,
}

impl<'info> SubmitProof<'info> {
    pub fn submit_proof(
        ctx: Context<SubmitProof>,
        proof_uri: String
    ) -> Result<()> {
        let milestone = &mut ctx.accounts.milestone;
        let clock = Clock::get()?;

        milestone.proof_uri = proof_uri;
        milestone.status = MilestoneStatus::Pending;
        milestone.updated_at = clock.unix_timestamp;

        Ok(())
    }
    
}