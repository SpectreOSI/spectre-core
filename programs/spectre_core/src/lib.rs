use anchor_lang::prelude::*;
use sha2::{Digest, Sha256};
use blake3::Hasher as Blake3Hasher;

declare_id!("SpEcTre11111111111111111111111111111111111");

#[program]
pub mod spectre_core {
    use super::*;

    /// Initialize a shadow account for a given owner.
    pub fn init_shadow_account(ctx: Context<InitShadowAccount>) -> Result<()> {
        let shadow = &mut ctx.accounts.shadow_account;
        shadow.owner = ctx.accounts.owner.key();
        shadow.bump = *ctx.bumps.get("shadow_account").unwrap();
        shadow.routing_tag = [0u8; 32];
        Ok(())
    }

    /// Update the routing tag used by off-chain relayers.
    pub fn set_routing_tag(
        ctx: Context<UpdateRoutingTag>,
        new_tag: [u8; 32],
    ) -> Result<()> {
        let shadow = &mut ctx.accounts.shadow_account;

        require_keys_eq!(
            shadow.owner,
            ctx.accounts.owner.key(),
            SpectreError::InvalidOwner
        );

        shadow.routing_tag = new_tag;
        Ok(())
    }

    /// Store a commitment for a private payment route.
    pub fn commit_route(
        ctx: Context<CommitRoute>,
        payload: Vec<u8>,
    ) -> Result<()> {
        let mut hasher = Sha256::new();
        hasher.update(payload);
        let digest = hasher.finalize();

        let mut blake = Blake3Hasher::new();
        blake.update(&digest);
        let blake_hash = blake.finalize();

        let route = &mut ctx.accounts.route_commitment;
        route.shadow = ctx.accounts.shadow_account.key();
        route.commitment = blake_hash.as_bytes().to_owned();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitShadowAccount<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + ShadowAccount::SIZE,
        seeds = [b"shadow", owner.key().as_ref()],
        bump
    )]
    pub shadow_account: Account<'info, ShadowAccount>,
    /// CHECK: owner can be any pubkey
    pub owner: UncheckedAccount<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateRoutingTag<'info> {
    #[account(
        mut,
        seeds = [b"shadow", owner.key().as_ref()],
        bump = shadow_account.bump
    )]
    pub shadow_account: Account<'info, ShadowAccount>,
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct CommitRoute<'info> {
    #[account(
        mut,
        seeds = [b"shadow", shadow_account.owner.as_ref()],
        bump = shadow_account.bump
    )]
    pub shadow_account: Account<'info, ShadowAccount>,

    #[account(
        init,
        payer = payer,
        space = 8 + RouteCommitment::SIZE,
        seeds = [b"route", shadow_account.key().as_ref()],
        bump
    )]
    pub route_commitment: Account<'info, RouteCommitment>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ShadowAccount {
    pub owner: Pubkey,
    pub routing_tag: [u8; 32],
    pub bump: u8,
}

impl ShadowAccount {
    pub const SIZE: usize = 32 + 32 + 1;
}

#[account]
pub struct RouteCommitment {
    pub shadow: Pubkey,
    pub commitment: Vec<u8>, // blake3(commit(sha256(payload)))
}

impl RouteCommitment {
    pub const SIZE: usize = 32 + 4 + 32; // vec overhead + hash bytes
}

#[error_code]
pub enum SpectreError {
    #[msg("owner does not match shadow account")]
    InvalidOwner,
}
