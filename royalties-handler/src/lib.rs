#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait RoyaltiesHandler {
    #[init]
    fn init(&self) {}
}
