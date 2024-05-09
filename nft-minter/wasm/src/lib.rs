// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           34
// Async Callback:                       1
// Total number of exported functions:  36

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    nft_minter
    (
        upgrade
        setMaxNftsPerTransaction
        getMaxNftsPerTransaction
        getRegisterdCollectionHashes
        getRegisteredBrands
        getNftTokenIdForBrand
        getPriceForTier
        getTagsForBrand
        getMintWhitelist
        addUserToAdminList
        removeUserFromAdminList
        issueTokenForBrand
        addToWhitelist
        removeFromWhitelist
        setMintWhitelistExpireTimestamp
        buyRandomNft
        giveawayNfts
        getNftTiersForBrand
        nftIdOffsetForTier
        setRoyaltiesClaimAddress
        changeRoyaltiesForBrand
        setMintPaymentsClaimAddress
        claimRoyalties
        claimMintPayments
        getRoyaltiesClaimAddress
        getMintPaymentsClaimAddress
        getAccumulatedRoyalties
        getAccumulatedMintPayments
        claimRoyaltiesFromMarketplace
        getBrandInfo
        getAllBrandsInfo
        pause
        unpause
        isPaused
        callBack
    )
}
