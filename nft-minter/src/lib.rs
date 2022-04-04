#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait NftMinter {
    #[init]
    fn init(&self) {}
}
