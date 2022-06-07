elrond_wasm::imports!();

use crate::common_storage::BrandId;

pub type TierName<M> = ManagedBuffer<M>;

const VEC_MAPPER_FIRST_ITEM_INDEX: usize = 1;
pub const MAX_TIERS_PER_BRAND: usize = 5;

#[elrond_wasm::module]
pub trait NftTierModule {
    fn get_next_random_id(
        &self,
        brand_id: &BrandId<Self::Api>,
        tier: &TierName<Self::Api>,
    ) -> UniqueId {
        let mut id_mapper = self.available_ids(brand_id, tier);
        let last_id_index = id_mapper.len();
        require!(last_id_index > 0, "No more NFTs available for brand");

        let rand_index = self.get_random_usize(VEC_MAPPER_FIRST_ITEM_INDEX, last_id_index + 1);
        let rand_id = id_mapper.swap_remove(rand_index);
        let id_offset = self.nft_id_offset_for_tier(brand_id, tier).get();

        rand_id + id_offset
    }

    /// range is [min, max)
    fn get_random_usize(&self, min: usize, max: usize) -> usize {
        let mut rand_source = RandomnessSource::<Self::Api>::new();
        rand_source.next_usize_in_range(min, max)
    }

    #[view(getNftTiersForBrand)]
    #[storage_mapper("nftTiersForBrand")]
    fn nft_tiers_for_brand(
        &self,
        brand_id: &BrandId<Self::Api>,
    ) -> UnorderedSetMapper<TierName<Self::Api>>;

    #[view(nftIdOffsetForTier)]
    #[storage_mapper("nftIdOffsetForTier")]
    fn nft_id_offset_for_tier(
        &self,
        brand_id: &BrandId<Self::Api>,
        tier: &TierName<Self::Api>,
    ) -> SingleValueMapper<usize>;

    #[storage_mapper("availableIds")]
    fn available_ids(
        &self,
        brand_id: &BrandId<Self::Api>,
        tier: &TierName<Self::Api>,
    ) -> UniqueIdMapper<Self::Api>;

    #[storage_mapper("totalNfts")]
    fn total_nfts(
        &self,
        brand_id: &BrandId<Self::Api>,
        tier: &TierName<Self::Api>,
    ) -> SingleValueMapper<usize>;
}
