pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("C9UEcAxZ1oP4F6o24Nb1Z6q3VR87mKd9i83hfwLKuRWQ");

pub const OAPP_SEED: &[u8] = b"OApp";

#[program]
pub mod endpoint {
    use super::*;

    pub fn register_oapp(mut ctx: Context<RegisterOApp>, params: RegisterOAppParams) -> Result<()> {
        RegisterOApp::apply(&mut ctx, &params)
    }
}
