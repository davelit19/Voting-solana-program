use anchor_lang::prelude::*;



declare_id!("CyvsiZ44UTGsBHaZyPH8UPPXQbLssD66mpPvY44KcNyC");

#[program]

mod voting_dapp {

    use super::*;

    pub fn initialize_admin(ctx: Context<InitializeAdmin>) -> Result<()> {
        let admin = &ctx.accounts.admin;

        if admin.admin_key != Pubkey::default() {
            return Err(error!(VotingErrors::AdminAlreadyInitialized));
        }
        let admin_data = &mut ctx.accounts.admin;
        admin_data.admin_key = ctx.accounts.signer.key();
        Ok(())
    }

    pub fn initialize_candidates(
        ctx: Context<InitializeCandidates>,
        candidates: Vec<String>,
    ) -> Result<()> {
        let signer = &ctx.accounts.signer;

        require!(
            ctx.accounts.admin.admin_key == signer.key(),
            VotingErrors::UnauthorizedSigner
        );

        let vote_candidates = &mut ctx.accounts.vote_candidates;

        vote_candidates.candidates_name = candidates;
        vote_candidates.candidates_votes = vec![0; vote_candidates.candidates_name.len()];
     //   vote_candidates.voting_result = vec![];

        msg!("Admin key {:?}", signer.key());

        Ok(())
    }

    pub fn add_new_candidate(
        ctx: Context<AddNewCandidateOrVoteCandidate>,
        new_candidate: String,
    ) -> Result<()> {
        let vote_candidates = &mut ctx.accounts.vote_candidates;
        let signer = &ctx.accounts.signer;

        require!(
            ctx.accounts.admin.admin_key == signer.key(),
            VotingErrors::UnauthorizedSigner
        );

        for candidate in vote_candidates.candidates_name.iter() {
            if candidate == &new_candidate {
                return Err(error!(VotingErrors::CandidateAlreadyExists));
            }
        }

        vote_candidates.candidates_name.push(new_candidate);
        vote_candidates.candidates_votes.push(0);

        Ok(())
    }

    pub fn vote_candidate(
        ctx: Context<AddNewCandidateOrVoteCandidate>,
        candidate_to_vote: String,
    ) -> Result<()> {
        let vote_candidates = &mut ctx.accounts.vote_candidates;
        let signer_key = &ctx.accounts.signer.key();

        if vote_candidates.voters.contains(&signer_key) {
            return Err(error!(VotingErrors::DuplicateVoteNotAllowed));
        }

        vote_candidates.voters.push(*signer_key);
        // Find the index of the candidate to vote for
        if let Some(index) = vote_candidates
            .candidates_name
            .iter()
            .position(|candidate| candidate == &candidate_to_vote)
        {
            // Increment the corresponding vote count
            vote_candidates.candidates_votes[index] += 1;
        } else {
            // Handle the case where the candidate is not found (optional)
            return Err(error!(VotingErrors::CandidateNotFound));
        }

        Ok(())
    }

    pub fn voting_result(ctx: Context<VotingResult>) -> Result<()> {
        let candidates_account = &mut ctx.accounts.vote_account;

        msg!("candidates array: {:?}", candidates_account.candidates_name);
        msg!(
            "candidates array: {:?}",
            candidates_account.candidates_votes
        );
        let mut result = vec![];

        for i in 0..candidates_account.candidates_name.len() {
            result.push(CandidateResult {
                candidate_name: candidates_account.candidates_name[i].clone(),
                candidate_vote_count: candidates_account.candidates_votes[i],
            });
        }
        candidates_account.voting_result = result;

        Ok(())

        //  let result = candidates_account.
    }
}

#[derive(Accounts)]
pub struct InitializeAdmin<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + 32,
        seeds = [b"admin".as_ref()],
        bump
       )]
    pub admin: Account<'info, AdminKey>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeCandidates<'info> {
    #[account(
    init,
    payer = signer,
    space = 1024,
    seeds = [b"candidate".as_ref(), admin.admin_key.as_ref()],
    bump
   )]
    pub vote_candidates: Account<'info, VotingData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    ///Check this is account is used to verify admin authorization
    pub admin: Account<'info, AdminKey>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddNewCandidateOrVoteCandidate<'info> {
    #[account(mut)]
    pub vote_candidates: Account<'info, VotingData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    // #[account()]
    pub admin: Account<'info, AdminKey>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VotingResult<'info> {
     #[account(mut)]
    pub vote_account: Account<'info, VotingData>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CandidateResult {
    candidate_name: String,
    candidate_vote_count: u64,
}

#[account]
pub struct VotingData {
    pub candidates_name: Vec<String>,
    pub candidates_votes: Vec<u64>,
    pub voters: Vec<Pubkey>,
    pub voting_result: Vec<CandidateResult>
}

#[account]
pub struct AdminKey {
    pub admin_key: Pubkey,
}

#[error_code]
pub enum VotingErrors {
    #[msg("Signer is not authorized to perform this action")]
    UnauthorizedSigner,
    #[msg("Candidate already exists.")]
    CandidateAlreadyExists,
    #[msg("Candidate does not exist.")]
    CandidateNotFound,
    #[msg("Insufficient balance to proceed.")]
    DuplicateVoteNotAllowed,
    #[msg("Voting is closed.")]
    VotingClosed,
    #[msg("Voting is closed.")]
    AdminAlreadyInitialized,
}