use anchor_lang::prelude::*;

declare_id!("YourProgramIDGoesHere");

#[program]
pub mod real_estate {
    use super::*;

    pub fn create_asset(ctx: Context<CreateAsset>, name: String, total_shares: u64) -> Result<()> {
        let asset = &mut ctx.accounts.asset;
        asset.name = name;
        asset.total_shares = total_shares;
        asset.creator = ctx.accounts.creator.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateAsset<'info> {
    #[account(init, payer = creator, space = 8 + 64 + 8 + 32)]
    pub asset: Account<'info, Asset>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Asset {
    pub name: String,
    pub total_shares: u64,
    pub creator: Pubkey,
}
