elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
    common_storage::{BrandId, BrandInfo, MintPrice, TimePeriod},
    nft_attributes_builder::{CollectionHash, Tag},
    nft_tier::{TierName, MAX_TIERS_PER_BRAND},
};

const NFT_AMOUNT: u32 = 1;
const NFT_ISSUE_COST: u64 = 50_000_000_000_000_000; // 0.05 EGLD
const ROYALTIES_MAX: u32 = 10_000; // 100%

const MAX_BRAND_ID_LEN: usize = 50;
static INVALID_BRAND_ID_ERR_MSG: &[u8] = b"Invalid Brand ID";

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct BrandInfoViewResultType<M: ManagedTypeApi> {
    pub brand_id: BrandId<M>,
    pub nft_token_id: TokenIdentifier<M>,
    pub brand_info: BrandInfo<M>,
    pub tier_info_entries: ArrayVec<TierInfoEntry<M>, MAX_TIERS_PER_BRAND>,
}

#[derive(TypeAbi, TopEncode, TopDecode, NestedEncode, NestedDecode, Debug, PartialEq)]
pub struct TierInfoEntry<M: ManagedTypeApi> {
    pub tier: TierName<M>,
    pub total_nfts: usize,
    pub available_nfts: usize,
    pub mint_price: MintPrice<M>,
}

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
pub trait NftModule:
    crate::common_storage::CommonStorageModule
    + crate::admin_whitelist::AdminWhitelistModule
    + crate::nft_attributes_builder::NftAttributesBuilderModule
    + crate::royalties::RoyaltiesModule
    + crate::nft_tier::NftTierModule
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

            tiers_info.push(TempCallbackTierInfo {
                tier,
                total_nfts: nr_nfts,
                id_offset: current_id_offset,
                mint_price: MintPrice {
                    token_id: mint_price_token_id.clone(),
                    amount: price,
                },
            });
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

    #[payable("*")]
    #[endpoint(buyRandomNft)]
    fn buy_random_nft(
        &self,
        brand_id: BrandId<Self::Api>,
        tier: TierName<Self::Api>,
        opt_nfts_to_buy: OptionalValue<usize>,
    ) {
        require!(
            self.registered_brands().contains(&brand_id),
            INVALID_BRAND_ID_ERR_MSG
        );
        require!(
            self.nft_tiers_for_brand(&brand_id).contains(&tier),
            "Invalid tier"
        );

        let nfts_to_buy = match opt_nfts_to_buy {
            OptionalValue::Some(val) => {
                if val == 0 {
                    return;
                }

                let max_nfts_per_transaction = self.max_nfts_per_transaction().get();
                require!(
                    val <= max_nfts_per_transaction,
                    "Max NFTs per transaction limit exceeded"
                );

                val
            }
            OptionalValue::None => NFT_AMOUNT as usize,
        };

        let price_for_tier: MintPrice<Self::Api> = self.price_for_tier(&brand_id, &tier).get();
        let payment: EsdtTokenPayment<Self::Api> = self.call_value().payment();
        let total_required_amount = &price_for_tier.amount * (nfts_to_buy as u32);
        require!(
            payment.token_identifier == price_for_tier.token_id
                && payment.amount == total_required_amount,
            "Invalid payment"
        );

        let brand_info: BrandInfo<Self::Api> = self.brand_info(&brand_id).get();
        let current_timestamp = self.blockchain().get_block_timestamp();
        require!(
            current_timestamp >= brand_info.mint_period.start,
            "May not mint yet"
        );
        require!(
            current_timestamp < brand_info.mint_period.end,
            "May not mint after deadline"
        );

        self.add_mint_payment(payment.token_identifier, payment.amount);

        let caller = self.blockchain().get_caller();
        self.mint_and_send_random_nft(&caller, &brand_id, &tier, &brand_info, nfts_to_buy);
    }

    #[endpoint(giveawayNfts)]
    fn giveaway_nfts(
        &self,
        brand_id: BrandId<Self::Api>,
        tier: TierName<Self::Api>,
        dest_amount_pairs: MultiValueEncoded<MultiValue2<ManagedAddress, usize>>,
    ) {
        self.require_caller_is_admin();

        require!(
            self.registered_brands().contains(&brand_id),
            INVALID_BRAND_ID_ERR_MSG
        );

        let brand_info = self.brand_info(&brand_id).get();
        for pair in dest_amount_pairs {
            let (dest_address, nfts_to_send) = pair.into_tuple();
            if nfts_to_send > 0 {
                self.mint_and_send_random_nft(
                    &dest_address,
                    &brand_id,
                    &tier,
                    &brand_info,
                    nfts_to_send,
                );
            }
        }
    }

    fn mint_and_send_random_nft(
        &self,
        to: &ManagedAddress,
        brand_id: &BrandId<Self::Api>,
        tier: &TierName<Self::Api>,
        brand_info: &BrandInfo<Self::Api>,
        nfts_to_send: usize,
    ) {
        let total_available_nfts = self.available_ids(brand_id, tier).len();
        require!(
            nfts_to_send <= total_available_nfts,
            "Not enough NFTs available"
        );

        let nft_token_id = self.nft_token(brand_id).get_token_id();
        let mut nft_output_payments = ManagedVec::new();
        for _ in 0..nfts_to_send {
            let nft_id = self.get_next_random_id(brand_id, tier);
            let nft_uri = self.build_nft_main_file_uri(
                &brand_info.collection_hash,
                nft_id,
                &brand_info.media_type,
            );
            let nft_json = self.build_nft_json_file_uri(&brand_info.collection_hash, nft_id);
            let collection_json = self.build_collection_json_file_uri(&brand_info.collection_hash);

            let mut uris = ManagedVec::new();
            uris.push(nft_uri);
            uris.push(nft_json);
            uris.push(collection_json);

            let attributes =
                self.build_nft_attributes(&brand_info.collection_hash, brand_id, nft_id);
            let nft_amount = BigUint::from(NFT_AMOUNT);
            let nft_nonce = self.send().esdt_nft_create(
                &nft_token_id,
                &nft_amount,
                &brand_info.token_display_name,
                &brand_info.royalties,
                &ManagedBuffer::new(),
                &attributes,
                &uris,
            );

            nft_output_payments.push(EsdtTokenPayment::new(
                nft_token_id.clone(),
                nft_nonce,
                nft_amount,
            ));
        }

        self.send().direct_multi(to, &nft_output_payments, &[]);
    }

    #[view(getBrandInfo)]
    fn get_brand_info_view(
        &self,
        brand_id: BrandId<Self::Api>,
    ) -> BrandInfoViewResultType<Self::Api> {
        require!(
            self.registered_brands().contains(&brand_id),
            INVALID_BRAND_ID_ERR_MSG
        );

        let nft_token_id = self.nft_token(&brand_id).get_token_id();
        let brand_info = self.brand_info(&brand_id).get();

        let mut tier_info_entries = ArrayVec::new();
        for tier in self.nft_tiers_for_brand(&brand_id).iter() {
            let total_nfts = self.total_nfts(&brand_id, &tier).get();
            let available_nfts = self.available_ids(&brand_id, &tier).len();
            let mint_price = self.price_for_tier(&brand_id, &tier).get();

            tier_info_entries.push(TierInfoEntry {
                tier,
                total_nfts,
                available_nfts,
                mint_price,
            })
        }

        BrandInfoViewResultType {
            brand_id,
            nft_token_id,
            brand_info,
            tier_info_entries,
        }
    }

    #[view(getAllBrandsInfo)]
    fn get_all_brands_info(&self) -> MultiValueEncoded<BrandInfoViewResultType<Self::Api>> {
        let mut result = MultiValueEncoded::new();
        for brand_id in self.registered_brands().iter() {
            let brand_info_entry = self.get_brand_info_view(brand_id);
            result.push(brand_info_entry);
        }

        result
    }

    #[view(getNftTokenIdForBrand)]
    #[storage_mapper("nftTokenId")]
    fn nft_token(&self, brand_id: &BrandId<Self::Api>) -> NonFungibleTokenMapper<Self::Api>;

    #[storage_mapper("temporaryCallbackStorage")]
    fn temporary_callback_storage(
        &self,
        brand_id: &BrandId<Self::Api>,
    ) -> SingleValueMapper<TempCallbackStorageInfo<Self::Api>>;
}
