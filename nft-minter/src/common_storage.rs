multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::{
    nft_attributes_builder::{CollectionHash, MediaType, Tag},
    nft_tier::TierName,
};

pub type PaymentsVec<M> = ManagedVec<M, EsdtTokenPayment<M>>;
pub type EgldValuePaymentsVecPair<M> = MultiValue2<BigUint<M>, PaymentsVec<M>>;
pub type BrandId<M> = ManagedBuffer<M>;

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct BrandInfo<M: ManagedTypeApi> {
    pub collection_hash: CollectionHash<M>,
    pub token_display_name: ManagedBuffer<M>,
    pub media_type: MediaType<M>,
    pub royalties: BigUint<M>,
    pub mint_period: TimePeriod,
    pub whitelist_expire_timestamp: u64,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct TimePeriod {
    pub start: u64,
    pub end: u64,
}

#[type_abi]
#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct MintPrice<M: ManagedTypeApi> {
    pub token_id: EgldOrEsdtTokenIdentifier<M>,
    pub amount: BigUint<M>,
}

#[multiversx_sc::module]
pub trait CommonStorageModule {
    #[view(getMaxNftsPerTransaction)]
    #[storage_mapper("maxNftsPerTransaction")]
    fn max_nfts_per_transaction(&self) -> SingleValueMapper<usize>;

    #[view(getRegisterdCollectionHashes)]
    #[storage_mapper("registeredCollectionHashes")]
    fn registered_collection_hashes(&self) -> UnorderedSetMapper<CollectionHash<Self::Api>>;

    #[view(getRegisteredBrands)]
    #[storage_mapper("registeredBrands")]
    fn registered_brands(&self) -> UnorderedSetMapper<BrandId<Self::Api>>;

    #[view(getNftTokenIdForBrand)]
    #[storage_mapper("nftTokenId")]
    fn nft_token(&self, brand_id: &BrandId<Self::Api>) -> NonFungibleTokenMapper<Self::Api>;

    #[storage_mapper("brandInfo")]
    fn brand_info(&self, brand_id: &BrandId<Self::Api>) -> SingleValueMapper<BrandInfo<Self::Api>>;

    #[view(getPriceForTier)]
    #[storage_mapper("priceForTier")]
    fn price_for_tier(
        &self,
        brand_id: &BrandId<Self::Api>,
        tier: &TierName<Self::Api>,
    ) -> SingleValueMapper<MintPrice<Self::Api>>;

    #[view(getTagsForBrand)]
    #[storage_mapper("tagsForBrand")]
    fn tags_for_brand(
        &self,
        brand_id: &BrandId<Self::Api>,
    ) -> SingleValueMapper<ManagedVec<Tag<Self::Api>>>;

    #[view(getMintWhitelist)]
    #[storage_mapper("mintWhitelist")]
    fn mint_whitelist(&self, brand_id: &BrandId<Self::Api>) -> UnorderedSetMapper<ManagedAddress>;
}
