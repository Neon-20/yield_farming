use anchor_lang::prelude::*;

declare_id!("ARA5SFBa5iGJKUiQEfBUNaEHCSUiFthgBLYJj1kpFsrS");

#[program]
pub mod backend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
