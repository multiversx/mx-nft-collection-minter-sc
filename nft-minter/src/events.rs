multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{common_storage::BrandId, nft_tier::TierName};

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("brandCreated")]
    fn brand_created_event(
        &self,
        #[indexed] brand_id: &BrandId<Self::Api>,
        #[indexed] nft_token_id: &TokenIdentifier,
    );

    #[event("nftBought")]
    fn nft_bought_event(
        &self,
        #[indexed] buyer_address: &ManagedAddress,
        #[indexed] brand_id: &BrandId<Self::Api>,
        #[indexed] tier: &TierName<Self::Api>,
        nr_nfts_bought: usize,
    );

    #[event("nftGiveaway")]
    fn nft_giveaway_event(
        &self,
        #[indexed] brand_id: &BrandId<Self::Api>,
        #[indexed] tier: &TierName<Self::Api>,
        total_nfts_given: usize,
    );
}
