use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::sysvar},
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{Mint, TokenAccount},
    },
    clockwork_crank::{program::ClockworkCrank, state::SEED_QUEUE},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(market_id: String)]
pub struct CreateSubscription<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        associated_token::authority = owner,
        associated_token::mint = mint,
    )]
    pub subscription_bank: Box<Account<'info, TokenAccount>>,

    pub mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        address = Subscription::pubkey(owner.key(),market_id),
        payer = owner,
        space = 8 + size_of::<Subscription>(),
    )]
    pub subscription: Account<'info, Subscription>,
    #[account(
        seeds = [
            SEED_QUEUE,
            subscription.key().as_ref(),
            "subscription".as_bytes()
        ],
        seeds::program = clockwork_crank::ID,
        bump
    )]
    pub subscriptions_queue: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, anchor_spl::token::Token>,
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(address = clockwork_crank::ID)]
    pub clockwork_program: Program<'info, ClockworkCrank>,
    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> CreateSubscription<'_> {
    pub fn process(
        &mut self,
        recurrent_amount: u64,
        epochs_reset: u64,
        mint: Pubkey,
        is_active: bool,
        market_id: String,
    ) -> Result<()> {
        let Self {
            owner,
            subscription,
            subscription_bank,
            ..
        } = self;

        subscription.new(
            owner.key(),
            subscription_bank.key(),
            mint,
            recurrent_amount,
            epochs_reset,
            is_active,
            vec![],
            market_id,
        );

        Ok(())
    }
}
