// #![allow(clippy::result_large_err)]
//
// use anchor_lang::prelude::*;
//
// declare_id!("DenE1esqiAydZLirKcykhFKMKVxq825x8dYc1Vix89PY");
//
// #[program]
// pub mod voting {
//     use super::*;
//
//     pub fn initialize_poll(
//         ctx: Context<InitializePoll>,
//         poll_id: u64,
//         description: String,
//         poll_start: u64,
//         poll_end: u64,
//     ) -> Result<()> {
//         let poll = &mut ctx.accounts.poll;
//         poll.poll_id = poll_id;
//         poll.description = description;
//         poll.poll_start = poll_start;
//         poll.poll_end = poll_end;
//         poll.candidate_amount = 0;
//
//         Ok(())
//     }
//
//     pub fn initialize_candidate(
//         ctx: Context<InitializeCandidate>,
//         _poll_id: u64,
//         candidate_name: String,
//     ) -> Result<()> {
//         Ok(())
//     }
// }
//
// #[derive(Accounts)]
// #[instruction(poll_id: u64, candidate: String)]
// pub struct InitializeCandidate<'info> {
//     #[account(mut)]
//     pub signer: Signer<'info>,
//
//     pub poll: Account<'info, Poll>,
//
//     #[account(
//     init,
//     payer = signer,
//     space = 8 + Candidate::INIT_SPACE,
//     seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
//     bump
//     )]
//     pub candidate: Account<'info, Candidate>,
//
//     pub system_program: Program<'info, System>,
// }
//
// #[derive(Accounts)]
// #[instruction(poll_id: u64)]
// pub struct InitializePoll<'info> {
//     #[account(mut)]
//     pub signer: Signer<'info>,
//
//     #[account(
//     init,
//     payer = signer,
//     space = 8 + Poll::INIT_SPACE,
//     seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
//     bump
//     )]
//     pub poll: Account<'info, Poll>,
//
//     pub system_program: Program<'info, System>,
// }
//
// #[account]
// #[derive(InitSpace)]
// pub struct Candidate {
//     #[max_len(32)]
//     pub candidate_name: String,
//     pub candidate_votes: u64,
// }
//
// #[account]
// #[derive(InitSpace)]
// pub struct Poll {
//     pub poll_id: u64,
//     #[max_len(280)]
//     pub description: String,
//     pub poll_start: u64,
//     pub poll_end: u64,
//     pub candidate_amount: u64,
// }
//
use anchor_lang::prelude::*;

// declare_id!("5s3PtT8kLYCv1WEp6dSh3T7EuF35Z6jSu5Cvx4hWG79H");

declare_id!("DenE1esqiAydZLirKcykhFKMKVxq825x8dYc1Vix89PY");

#[program]
pub mod voting {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        _poll_id: u64,
        start_time: u64,
        end_time: u64,
        name: String,
        description: String,
    ) -> Result<()> {
        ctx.accounts.poll_account.poll_name = name;
        ctx.accounts.poll_account.poll_description = description;
        ctx.accounts.poll_account.poll_voting_start = start_time;
        ctx.accounts.poll_account.poll_voting_end = end_time;
        Ok(())
    }

    pub fn initialize_candidate(
        ctx: Context<InitializeCandidate>,
        _poll_id: u64,
        candidate: String,
    ) -> Result<()> {
        ctx.accounts.candidate_account.candidate_name = candidate;
        ctx.accounts.poll_account.poll_option_index += 1;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, _poll_id: u64, _candidate: String) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct InitializeCandidate<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init,
        payer = signer,
        space = 8 + CandidateAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump
    )]
    pub candidate_account: Account<'info, CandidateAccount>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(poll_id: u64, candidate: String)]
pub struct Vote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll".as_ref(), poll_id.to_le_bytes().as_ref()],
        bump,
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds = [poll_id.to_le_bytes().as_ref(), candidate.as_ref()],
        bump)]
    pub candidate_account: Account<'info, CandidateAccount>,
}

#[account]
#[derive(InitSpace)]
pub struct CandidateAccount {
    #[max_len(32)]
    pub candidate_name: String,
    pub candidate_votes: u64,
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    #[max_len(32)]
    pub poll_name: String,
    #[max_len(280)]
    pub poll_description: String,
    pub poll_voting_start: u64,
    pub poll_voting_end: u64,
    pub poll_option_index: u64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Voting has not started yet")]
    VotingNotStarted,
    #[msg("Voting has ended")]
    VotingEnded,
}
