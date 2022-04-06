elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::{
    common_storage::{BrandId, BrandInfo, Tag},
    nonce_mapper::NonceMapper,
};

pub const NFT_ISSUE_COST: u64 = 50_000_000_000_000_000; // 0.05 EGLD
pub const ROYALTIES_MAX: u32 = 10_000; // 100%

#[elrond_wasm::module]
pub trait NftModule:
    crate::common_storage::CommonStorageModule + crate::admin_whitelist::AdminWhitelistModule
{
    #[payable("EGLD")]
    #[endpoint(issueTokenForBrand)]
    fn issue_token_for_brand(
        &self,
        brand_id: BrandId<Self::Api>,
        royalties: BigUint,
        max_nfts: usize,
        mint_start_epoch: u64,
        mint_price_token_id: TokenIdentifier,
        mint_price_amount: BigUint,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        #[var_args] tags: MultiValueEncoded<Tag<Self::Api>>,
    ) {
        let payment_amount = self.call_value().egld_value();
        require!(
            payment_amount == NFT_ISSUE_COST,
            "Invalid payment amount. Issue costs exactly 0.05 EGLD"
        );

        require!(royalties <= ROYALTIES_MAX, "Royalties cannot be over 100%");
        require!(max_nfts > 0, "Cannot create brand with max 0 items");
        require!(
            mint_price_token_id.is_egld() || mint_price_token_id.is_valid_esdt_identifier(),
            "Invalid price token"
        );

        let is_new_brand = self.registered_brands().insert(brand_id.clone());
        require!(is_new_brand, "Brand already exists");

        let brand_info = BrandInfo {
            royalties,
            mint_start_epoch,
            mint_price_token_id,
            mint_price_amount,
        };
        self.brand_info(&brand_id).set(&brand_info);
        self.available_nonces(&brand_id).set_initial_len(max_nfts);

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
                self.available_nonces(&brand_id).clear_len();
                let _ = self.registered_brands().swap_remove(&brand_id);
            }
        }
    }

    #[endpoint(setLocalRoles)]
    fn set_local_roles(&self, brand_id: BrandId<Self::Api>) {
        self.nft_token(&brand_id)
            .set_local_roles(&[EsdtLocalRole::NftCreate], None);
    }

    #[view(getNftTokenIdForBrand)]
    #[storage_mapper("nftTokenId")]
    fn nft_token(&self, brand_id: &BrandId<Self::Api>) -> NonFungibleTokenMapper<Self::Api>;

    #[storage_mapper("availableNonces")]
    fn available_nonces(&self, brand_id: &BrandId<Self::Api>) -> NonceMapper<Self::Api>;
}
