use anchor_lang::prelude::*;

declare_id!("8PJFAdH2RJ2v1zdME3HU477yvHf7LRheLWd3xxeSbrsZ");

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

#[program]
pub mod tita_program {
    use super::*;

}
