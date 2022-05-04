#![no_std]

elrond_wasm::imports!();

pub mod admin_whitelist;
pub mod brand_creation;
pub mod common_storage;
pub mod nft_attributes_builder;
pub mod nft_marketplace_interactor;
pub mod nft_minting;
pub mod nft_tier;
pub mod royalties;
pub mod unique_id_mapper;
pub mod views;

#[elrond_wasm::contract]
pub trait NftMinter:
    common_storage::CommonStorageModule
    + admin_whitelist::AdminWhitelistModule
    + brand_creation::BrandCreationModule
    + nft_minting::NftMintingModule
    + nft_tier::NftTierModule
    + nft_attributes_builder::NftAttributesBuilderModule
    + royalties::RoyaltiesModule
    + nft_marketplace_interactor::NftMarketplaceInteractorModule
    + views::ViewsModule
{
    #[init]
    fn init(
        &self,
        collections_category: ManagedBuffer,
        royalties_claim_address: ManagedAddress,
        mint_payments_claim_address: ManagedAddress,
        max_nfts_per_transaction: usize,
    ) {
        self.collections_category().set(&collections_category);
        self.royalties_claim_address().set(&royalties_claim_address);
        self.mint_payments_claim_address()
            .set(&mint_payments_claim_address);
        self.set_max_nfts_per_transaction(max_nfts_per_transaction);
    }

    #[only_owner]
    #[endpoint(setMaxNftsPerTransaction)]
    fn set_max_nfts_per_transaction(&self, max: usize) {
        require!(max > 0, "Invalid max NFTs per transaction");
        self.max_nfts_per_transaction().set(max);
    }
}
