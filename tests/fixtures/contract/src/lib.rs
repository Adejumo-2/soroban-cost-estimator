#![no_std]

//! Minimal Soroban contract used as a structural test fixture for
//! `soroban-cost-estimator`: one exported function taking one typed
//! argument. The storage write gives `simulateTransaction` a non-trivial
//! footprint (rent / refundable fee portion) so fee breakdowns are realistic.
//!
//! Rebuild with `./build.sh` (requires the `wasm32v1-none` rustup target).

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Reads a stored counter, adds `step`, writes the new value back and
    /// returns it.
    pub fn increment(env: Env, step: i64) -> i64 {
        let key = symbol_short!("count");
        let current = env.storage().instance().get::<Symbol, i64>(&key).unwrap_or(0);
        let next = current + step;
        env.storage().instance().set(&key, &next);
        next
    }
}
