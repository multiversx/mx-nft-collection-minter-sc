pub mod constants;
pub mod nft_minter_interactor;

use constants::*;
use nft_minter_interactor::*;

#[test]
fn init_test() {
    let _ = NftMinterSetup::new(nft_minter::contract_obj);
}

#[test]
fn create_brands_test() {
    let mut nm_setup = NftMinterSetup::new(nft_minter::contract_obj);
    nm_setup.create_default_brands();
}
