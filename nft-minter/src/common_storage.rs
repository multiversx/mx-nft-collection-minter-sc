elrond_wasm::imports!();
elrond_wasm::derive_imports!();

pub type PaymentsVec<M> = ManagedVec<M, EsdtTokenPayment<M>>;
pub type EgldValuePaymentsVecPair<M> = MultiValue2<BigUint<M>, PaymentsVec<M>>;
pub type BrandId<M> = ManagedBuffer<M>;
pub type CollectionId<M> = ManagedBuffer<M>;
pub type Tag<M> = ManagedBuffer<M>;

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct BrandInfo<M: ManagedTypeApi> {
    pub royalties: BigUint<M>,
    pub mint_start_epoch: u64,
    pub mint_price_token_id: TokenIdentifier<M>,
    pub mint_price_amount: BigUint<M>,
}

#[elrond_wasm::module]
pub trait CommonStorageModule {
    #[view(getParentCollectionId)]
    #[storage_mapper("parentCollectionId")]
    fn parent_collection_id(&self) -> SingleValueMapper<CollectionId<Self::Api>>;

    #[view(getRegisteredBrands)]
    #[storage_mapper("registeredBrands")]
    fn registered_brands(&self) -> UnorderedSetMapper<BrandId<Self::Api>>;

    #[view(getBrandOwner)]
    #[storage_mapper("brandOwner")]
    fn brand_owner(&self, brand_id: &BrandId<Self::Api>) -> SingleValueMapper<ManagedAddress>;

    #[view(getBrandInfo)]
    #[storage_mapper("brandInfo")]
    fn brand_info(&self, brand_id: &BrandId<Self::Api>) -> SingleValueMapper<BrandInfo<Self::Api>>;

    #[view(getTagsForBrand)]
    #[storage_mapper("tagsForBrand")]
    fn tags_for_brand(
        &self,
        brand_id: &BrandId<Self::Api>,
    ) -> SingleValueMapper<ManagedVec<Tag<Self::Api>>>;
}
