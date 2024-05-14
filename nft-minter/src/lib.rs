#![no_std]

multiversx_sc::imports!();

pub mod admin_whitelist;
pub mod brand_creation;
pub mod common_storage;
pub mod events;
pub mod nft_attributes_builder;
pub mod nft_marketplace_interactor;
pub mod nft_minter_proxy;
pub mod nft_minting;
pub mod nft_tier;
pub mod royalties;
pub mod views;

use multiversx_sc_modules::pause;

#[multiversx_sc::contract]
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
    + events::EventsModule
    + pause::PauseModule
{
    #[init]
    fn init(
        &self,
        royalties_claim_address: ManagedAddress,
        mint_payments_claim_address: ManagedAddress,
        max_nfts_per_transaction: usize,
        opt_admin: OptionalValue<ManagedAddress>,
    ) {
        self.royalties_claim_address().set(&royalties_claim_address);
        self.mint_payments_claim_address()
            .set(&mint_payments_claim_address);
        self.set_max_nfts_per_transaction(max_nfts_per_transaction);

        if let OptionalValue::Some(admin) = opt_admin {
            self.add_user_to_admin_list(admin);
        }
    }

    #[upgrade]
    fn upgrade(
        &self,
        royalties_claim_address: ManagedAddress,
        mint_payments_claim_address: ManagedAddress,
        max_nfts_per_transaction: usize,
        opt_admin: OptionalValue<ManagedAddress>,
    ) {
        self.royalties_claim_address().set(&royalties_claim_address);
        self.mint_payments_claim_address()
            .set(&mint_payments_claim_address);
        self.set_max_nfts_per_transaction(max_nfts_per_transaction);

        if let OptionalValue::Some(admin) = opt_admin {
            self.add_user_to_admin_list(admin);
        }
    }

    #[only_owner]
    #[endpoint(setMaxNftsPerTransaction)]
    fn set_max_nfts_per_transaction(&self, max: usize) {
        require!(max > 0, "Invalid max NFTs per transaction");
        self.max_nfts_per_transaction().set(max);
    }
}
