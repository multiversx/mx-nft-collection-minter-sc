elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
    common_storage::{BrandId, BrandInfo, MintPrice, TimePeriod},
    nft_attributes_builder::{CollectionHash, Tag},
    nft_tier::{TierName, MAX_TIERS_PER_BRAND},
};

const NFT_ISSUE_COST: u64 = 50_000_000_000_000_000; // 0.05 EGLD
const ROYALTIES_MAX: u32 = 10_000; // 100%

const MAX_BRAND_ID_LEN: usize = 50;
pub static INVALID_BRAND_ID_ERR_MSG: &[u8] = b"Invalid Brand ID";

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode)]
pub struct TempCallbackTierInfo<M: ManagedTypeApi> {
    pub tier: TierName<M>,
    pub total_nfts: usize,
    pub id_offset: usize,
    pub mint_price: MintPrice<M>,
}

#[derive(TopEncode, TopDecode)]
pub struct TempCallbackStorageInfo<M: ManagedTypeApi> {
    pub brand_info: BrandInfo<M>,
    pub tags: ManagedVec<M, Tag<M>>,
    pub tier_info_entries: ArrayVec<TempCallbackTierInfo<M>, MAX_TIERS_PER_BRAND>,
}

/// Tier name, number of NFTs, price
pub type TierArgPair<M> = MultiValue3<TierName<M>, usize, BigUint<M>>;

#[elrond_wasm::module]
pub trait BrandCreationModule:
    crate::admin_whitelist::AdminWhitelistModule
    + crate::common_storage::CommonStorageModule
    + crate::nft_attributes_builder::NftAttributesBuilderModule
    + crate::nft_tier::NftTierModule
    + crate::events::EventsModule
{
    #[payable("EGLD")]
    #[endpoint(issueTokenForBrand)]
    fn issue_token_for_brand(
        &self,
        collection_hash: CollectionHash<Self::Api>,
        brand_id: BrandId<Self::Api>,
        media_type: ManagedBuffer,
        royalties: BigUint,
        mint_start_timestamp: u64,
        mint_end_timestamp: u64,
        mint_price_token_id: TokenIdentifier,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        tags: ManagedVec<Tag<Self::Api>>,
        tier_name_nr_nfts_pairs: MultiValueEncoded<TierArgPair<Self::Api>>,
    ) {
        self.require_caller_is_admin();

        let id_len = brand_id.len();
        require!(
            id_len > 0 && id_len <= MAX_BRAND_ID_LEN,
            INVALID_BRAND_ID_ERR_MSG
        );

        let payment_amount = self.call_value().egld_value();
        require!(
            payment_amount == NFT_ISSUE_COST,
            "Invalid payment amount. Issue costs exactly 0.05 EGLD"
        );

        require!(
            self.is_supported_media_type(&media_type),
            "Invalid media type"
        );
        require!(royalties <= ROYALTIES_MAX, "Royalties cannot be over 100%");
        require!(
            mint_price_token_id.is_egld() || mint_price_token_id.is_valid_esdt_identifier(),
            "Invalid price token"
        );

        let is_new_collection = self
            .registered_collection_hashes()
            .insert(collection_hash.clone());
        require!(is_new_collection, "Collection hash already exists");

        let is_new_brand = self.registered_brands().insert(brand_id.clone());
        require!(is_new_brand, "Brand already exists");

        require!(
            mint_start_timestamp < mint_end_timestamp,
            "Invalid timestamps"
        );
        require!(
            !tier_name_nr_nfts_pairs.is_empty(),
            "Must have at least one tier"
        );
        require!(
            tier_name_nr_nfts_pairs.len() <= MAX_TIERS_PER_BRAND,
            "Max tiers per brand limit exceeded"
        );

        let mut tier_mapper = self.nft_tiers_for_brand(&brand_id);
        let mut tiers_info = ArrayVec::new();
        let mut current_id_offset = 0;
        for pair in tier_name_nr_nfts_pairs {
            let (tier, nr_nfts, price): (TierName<Self::Api>, usize, BigUint) = pair.into_tuple();

            let is_new_tier = tier_mapper.insert(tier.clone());
            require!(is_new_tier, "Duplicate tier name");

            unsafe {
                tiers_info.push_unchecked(TempCallbackTierInfo {
                    tier,
                    total_nfts: nr_nfts,
                    id_offset: current_id_offset,
                    mint_price: MintPrice {
                        token_id: mint_price_token_id.clone(),
                        amount: price,
                    },
                });
            }
            current_id_offset += nr_nfts;
        }

        let brand_info = BrandInfo {
            collection_hash: collection_hash.clone(),
            token_display_name: token_display_name.clone(),
            media_type,
            royalties,
            mint_period: TimePeriod {
                start: mint_start_timestamp,
                end: mint_end_timestamp,
            },
        };

        self.temporary_callback_storage(&brand_id)
            .set(&TempCallbackStorageInfo {
                brand_info,
                tags,
                tier_info_entries: tiers_info,
            });

        self.nft_token(&brand_id).issue_and_set_all_roles(
            EsdtTokenType::NonFungible,
            payment_amount,
            token_display_name,
            token_ticker,
            0,
            Some(self.callbacks().issue_callback(collection_hash, brand_id)),
        );
    }

    #[callback]
    fn issue_callback(
        &self,
        collection_hash: CollectionHash<Self::Api>,
        brand_id: BrandId<Self::Api>,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                let cb_info: TempCallbackStorageInfo<Self::Api> =
                    self.temporary_callback_storage(&brand_id).get();

                self.nft_token(&brand_id).set_token_id(&token_id);
                self.brand_info(&brand_id).set(&cb_info.brand_info);

                for tier_info in cb_info.tier_info_entries {
                    self.available_ids(&brand_id, &tier_info.tier)
                        .set_initial_len(tier_info.total_nfts);
                    self.total_nfts(&brand_id, &tier_info.tier)
                        .set(tier_info.total_nfts);
                    self.nft_id_offset_for_tier(&brand_id, &tier_info.tier)
                        .set(tier_info.id_offset);

                    self.price_for_tier(&brand_id, &tier_info.tier)
                        .set(&tier_info.mint_price);
                }

                if !cb_info.tags.is_empty() {
                    self.tags_for_brand(&brand_id).set(&cb_info.tags);
                }

                self.brand_created_event(&brand_id, &token_id);
            }
            ManagedAsyncCallResult::Err(_) => {
                let _ = self.registered_brands().swap_remove(&brand_id);
                let _ = self
                    .registered_collection_hashes()
                    .swap_remove(&collection_hash);
                self.nft_tiers_for_brand(&brand_id).clear();
            }
        }

        self.temporary_callback_storage(&brand_id).clear();
    }

    #[storage_mapper("temporaryCallbackStorage")]
    fn temporary_callback_storage(
        &self,
        brand_id: &BrandId<Self::Api>,
    ) -> SingleValueMapper<TempCallbackStorageInfo<Self::Api>>;
}
