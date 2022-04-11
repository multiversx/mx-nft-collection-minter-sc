#![no_std]

elrond_wasm::imports!();

pub mod common_storage;
pub mod nft_minter_interactor;
pub mod shareholders;

#[elrond_wasm::contract]
pub trait RoyaltiesHandler:
    common_storage::CommonStorageModule
    + nft_minter_interactor::NftMinterInteractorModule
    + shareholders::ShareholdersModule
{
    #[init]
    fn init(
        &self,
        nft_minter_sc_address: ManagedAddress,
        #[var_args] shareholders: MultiValueEncoded<ManagedAddress>,
    ) {
        require!(
            self.blockchain().is_smart_contract(&nft_minter_sc_address),
            "Invalid NFT Minter SC address"
        );

        self.nft_minter_sc_address().set(&nft_minter_sc_address);
        self.add_shareholders(shareholders);

        // init reward entry list
        self.first_entry_number().set_if_empty(&1);
        self.last_entry_number().set_if_empty(&1);
    }
}
