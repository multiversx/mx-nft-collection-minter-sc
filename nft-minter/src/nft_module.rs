elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
    common_storage::{BrandId, BrandInfo, Tag},
    unique_id_mapper::{UniqueId, UniqueIdMapper},
};

const NFT_AMOUNT: u32 = 1;
const NFT_ISSUE_COST: u64 = 50_000_000_000_000_000; // 0.05 EGLD
const ROYALTIES_MAX: u32 = 10_000; // 100%
const VEC_MAPPER_FIRST_ITEM_INDEX: usize = 1;

const MAX_BRAND_ID_LEN: usize = 50;
static INVALID_BRAND_ID_ERR_MSG: &[u8] = b"Invalid Brand ID";

#[elrond_wasm::module]
pub trait NftModule:
    crate::common_storage::CommonStorageModule
    + crate::admin_whitelist::AdminWhitelistModule
    + crate::nft_attributes_builder::NftAttributesBuilderModule
    + crate::royalties::RoyaltiesModule
{
    #[payable("EGLD")]
    #[endpoint(issueTokenForBrand)]
    fn issue_token_for_brand(
        &self,
        brand_id: BrandId<Self::Api>,
        media_type: ManagedBuffer,
        royalties: BigUint,
        max_nfts: usize,
        mint_start_epoch: u64,
        mint_price_token_id: TokenIdentifier,
        mint_price_amount: BigUint,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        #[var_args] tags: MultiValueEncoded<Tag<Self::Api>>,
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
        require!(max_nfts > 0, "Cannot create brand with max 0 items");
        require!(
            mint_price_token_id.is_egld() || mint_price_token_id.is_valid_esdt_identifier(),
            "Invalid price token"
        );

        let is_new_brand = self.registered_brands().insert(brand_id.clone());
        require!(is_new_brand, "Brand already exists");

        let id_offset = self.last_item_id().update(|last_id| {
            let prev_last_id = *last_id;
            *last_id += max_nfts;
            require!(*last_id > prev_last_id, "ID overflow!");

            prev_last_id
        });
        let brand_info = BrandInfo {
            token_display_name: token_display_name.clone(),
            media_type,
            id_offset,
            royalties,
            mint_start_epoch,
            mint_price_token_id,
            mint_price_amount,
        };
        self.brand_info(&brand_id).set(&brand_info);
        self.available_ids(&brand_id).set_initial_len(max_nfts);

        if !tags.is_empty() {
            self.tags_for_brand(&brand_id).set(&tags.to_vec());
        }

        self.nft_token(&brand_id).issue(
            EsdtTokenType::NonFungible,
            payment_amount,
            token_display_name,
            token_ticker,
            0,
            Some(self.callbacks().issue_callback(brand_id)),
        );
    }

    #[callback]
    fn issue_callback(
        &self,
        brand_id: BrandId<Self::Api>,
        #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.nft_token(&brand_id).set_token_id(&token_id);
            }
            ManagedAsyncCallResult::Err(_) => {
                self.brand_info(&brand_id).clear();
                self.tags_for_brand(&brand_id).clear();
                self.available_ids(&brand_id).clear_len();
                let _ = self.registered_brands().swap_remove(&brand_id);
            }
        }
    }

    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self, brand_id: BrandId<Self::Api>) {
        self.nft_token(&brand_id)
            .set_local_roles(&[EsdtLocalRole::NftCreate], None);
    }

    #[payable("*")]
    #[endpoint(buyRandomNft)]
    fn buy_random_nft(&self, brand_id: BrandId<Self::Api>) {
        require!(
            self.registered_brands().contains(&brand_id),
            INVALID_BRAND_ID_ERR_MSG
        );

        let brand_info: BrandInfo<Self::Api> = self.brand_info(&brand_id).get();
        let payment: EsdtTokenPayment<Self::Api> = self.call_value().payment();
        require!(
            payment.token_identifier == brand_info.mint_price_token_id
                && payment.amount == brand_info.mint_price_amount,
            "Invalid payment"
        );

        self.add_mint_payment(payment.token_identifier, payment.amount);

        let caller = self.blockchain().get_caller();
        self.send_random_nft(&caller, &brand_id, &brand_info);
    }

    #[endpoint(givawayNft)]
    fn giveaway_nft(&self, to: ManagedAddress, brand_id: BrandId<Self::Api>) {
        self.require_caller_is_admin();

        let brand_info = self.brand_info(&brand_id).get();
        self.send_random_nft(&to, &brand_id, &brand_info);
    }

    fn send_random_nft(
        &self,
        to: &ManagedAddress,
        brand_id: &BrandId<Self::Api>,
        brand_info: &BrandInfo<Self::Api>,
    ) {
        let nft_id = self.get_next_random_id(brand_id, brand_info.id_offset);
        let nft_uri = self.build_nft_main_file_uri(nft_id, &brand_info.media_type);
        let nft_json = self.build_nft_json_file_uri(nft_id);
        let collection_json = self.build_collection_json_file_uri();

        let mut uris = ManagedVec::new();
        uris.push(nft_uri);
        uris.push(nft_json);
        uris.push(collection_json);

        let attributes = self.build_nft_attributes(brand_id, nft_id);
        let nft_token_id = self.nft_token(brand_id).get_token_id();
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

        self.send()
            .direct(to, &nft_token_id, nft_nonce, &nft_amount, &[]);
    }

    fn get_next_random_id(&self, brand_id: &BrandId<Self::Api>, id_offset: usize) -> UniqueId {
        let mut id_mapper = self.available_ids(brand_id);
        let last_id_index = id_mapper.len();
        require!(last_id_index > 0, "No more NFTs available for brand");

        let rand_index = self.get_random_usize(VEC_MAPPER_FIRST_ITEM_INDEX, last_id_index + 1);
        let rand_id = id_mapper.get_and_swap_remove(rand_index);

        rand_id + id_offset
    }

    /// range is [min, max)
    fn get_random_usize(&self, min: usize, max: usize) -> usize {
        let mut rand_source = RandomnessSource::<Self::Api>::new();
        rand_source.next_usize_in_range(min, max)
    }

    #[view(getNftTokenIdForBrand)]
    #[storage_mapper("nftTokenId")]
    fn nft_token(&self, brand_id: &BrandId<Self::Api>) -> NonFungibleTokenMapper<Self::Api>;

    #[storage_mapper("availableIds")]
    fn available_ids(&self, brand_id: &BrandId<Self::Api>) -> UniqueIdMapper<Self::Api>;
}
