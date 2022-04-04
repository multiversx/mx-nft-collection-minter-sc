#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait CollectionManager {
    #[init]
    fn init(&self) {}
}
