use anchor_lang::prelude::*;

declare_id!("7YvYAht69qDBoSNqiVGURcwbYB9KrNioHuiLKzLGMDAc");

#[program]
pub mod frankenstain_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
