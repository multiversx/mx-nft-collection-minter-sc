// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           33
// Async Callback:                       1
// Total number of exported functions:  36

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    nft_minter
    (
        init => init
        upgrade => upgrade
        setMaxNftsPerTransaction => set_max_nfts_per_transaction
        getMaxNftsPerTransaction => max_nfts_per_transaction
        getRegisterdCollectionHashes => registered_collection_hashes
        getRegisteredBrands => registered_brands
        getNftTokenIdForBrand => nft_token
        getPriceForTier => price_for_tier
        getTagsForBrand => tags_for_brand
        getMintWhitelist => mint_whitelist
        addUserToAdminList => add_user_to_admin_list
        removeUserFromAdminList => remove_user_from_admin_list
        issueTokenForBrand => issue_token_for_brand
        addToWhitelist => add_to_whitelist
        removeFromWhitelist => remove_from_whitelist
        setMintWhitelistExpireTimestamp => set_mint_whitelist_expire_timestamp
        buyRandomNft => buy_random_nft
        giveawayNfts => giveaway_nfts
        getNftTiersForBrand => nft_tiers_for_brand
        nftIdOffsetForTier => nft_id_offset_for_tier
        setRoyaltiesClaimAddress => set_royalties_claim_address
        changeRoyaltiesForBrand => change_royalties_for_brand
        setMintPaymentsClaimAddress => set_mint_payments_claim_address
        claimRoyalties => claim_royalties
        claimMintPayments => claim_mint_payments
        getRoyaltiesClaimAddress => royalties_claim_address
        getMintPaymentsClaimAddress => mint_payments_claim_address
        getAccumulatedRoyalties => accumulated_royalties
        getAccumulatedMintPayments => accumulated_mint_payments
        claimRoyaltiesFromMarketplace => claim_royalties_from_marketplace
        getBrandInfo => get_brand_info_view
        getAllBrandsInfo => get_all_brands_info
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
    )
}

multiversx_sc_wasm_adapter::async_callback! { nft_minter }
