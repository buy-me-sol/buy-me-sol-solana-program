use anchor_lang::prelude::*;

declare_id!("6XxwwEZhFbEfg5d8iiEXhArX15rSwRbtLanQ3EcETKXy");

#[program]
pub mod buy_me_sol_solana_program {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        // Get reference of the base account
        let base_account = &mut ctx.accounts.base_account;
        // Initialize creators
        base_account.total_creators = 0;
        base_account.total_supporters = 0;
        Ok(())
    }

    // Function to create creator account
    pub fn create_creator(
        ctx: Context<CreateAccount>,
        username: String,
        name: String,
    ) -> ProgramResult {
        // Get reference of the base account
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // BUild the struct
        let creator = CreatorStruct {
            user_address: *user.to_account_info().key,
            username: username.to_string(),
            name: name.to_string(),
        };

        // Add it to the creator_list vector
        base_account.creator_list.push(creator);
        // Update Creator Count
        base_account.total_creators += 1;

        Ok(())
    }

    // Function to create supporter account
    pub fn create_supporter(ctx: Context<CreateAccount>, name: String) -> ProgramResult {
        // Get reference of the base account
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // BUild the struct
        let supporter = SupporterStruct {
            user_address: *user.to_account_info().key,
            name: name.to_string(),
        };

        // Add it to the creator_list vector
        base_account.supporter_list.push(supporter);
        // Update Creator Count
        base_account.total_supporters += 1;

        Ok(())
    }
}

// Specify data needed in Initialize Context
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 10000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Specify data needed in CreateAccount Context
#[derive(Accounts)]
pub struct CreateAccount<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

// Custom struct for Creator Account
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreatorStruct {
    pub user_address: Pubkey,
    pub username: String,
    pub name: String,
}

// Custom struct for Supporter Account
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SupporterStruct {
    pub user_address: Pubkey,
    pub name: String,
}

#[account]
pub struct BaseAccount {
    pub total_creators: u64,
    pub total_supporters: u64,
    // Vector of type CreatorStruct to store creators list
    pub creator_list: Vec<CreatorStruct>,
    pub supporter_list: Vec<SupporterStruct>,
}
