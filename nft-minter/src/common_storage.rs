elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub const COLLECTION_HASH_LEN: usize = 46;

pub type PaymentsVec<M> = ManagedVec<M, EsdtTokenPayment<M>>;
pub type EgldValuePaymentsVecPair<M> = MultiValue2<BigUint<M>, PaymentsVec<M>>;
pub type BrandId<M> = ManagedBuffer<M>;
pub type CollectionHash<M> = ManagedByteArray<M, COLLECTION_HASH_LEN>;
pub type Tag<M> = ManagedBuffer<M>;
pub type Uri<M> = ManagedBuffer<M>;
pub type MediaType<M> = ManagedBuffer<M>;
pub type GenericAttributes<M> = ManagedBuffer<M>;

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct BrandInfo<M: ManagedTypeApi> {
    pub collection_hash: CollectionHash<M>,
    pub token_display_name: ManagedBuffer<M>,
    pub media_type: MediaType<M>,
    pub royalties: BigUint<M>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Debug)]
pub struct MintPrice<M: ManagedTypeApi> {
    pub start_timestamp: u64,
    pub opt_end_timestamp: Option<u64>,
    pub token_id: TokenIdentifier<M>,
    pub amount: BigUint<M>,
}

#[elrond_wasm::module]
pub trait CommonStorageModule {
    #[view(getCollectionsCategory)]
    #[storage_mapper("collectionsCategory")]
    fn collections_category(&self) -> SingleValueMapper<ManagedBuffer>;

    #[view(getRegisterdCollectionHashes)]
    #[storage_mapper("registeredCollectionHashes")]
    fn registered_collection_hashes(&self) -> UnorderedSetMapper<CollectionHash<Self::Api>>;

    #[view(getRegisteredBrands)]
    #[storage_mapper("registeredBrands")]
    fn registered_brands(&self) -> UnorderedSetMapper<BrandId<Self::Api>>;

    #[storage_mapper("brandInfo")]
    fn brand_info(&self, brand_id: &BrandId<Self::Api>) -> SingleValueMapper<BrandInfo<Self::Api>>;

    #[view(getPriceForBrand)]
    #[storage_mapper("priceForBrand")]
    fn price_for_brand(
        &self,
        brand_id: &BrandId<Self::Api>,
    ) -> SingleValueMapper<MintPrice<Self::Api>>;

    #[view(getTagsForBrand)]
    #[storage_mapper("tagsForBrand")]
    fn tags_for_brand(
        &self,
        brand_id: &BrandId<Self::Api>,
    ) -> SingleValueMapper<ManagedVec<Tag<Self::Api>>>;
}
