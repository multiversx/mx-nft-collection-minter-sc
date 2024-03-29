// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           14
// Async Callback (empty):               1
// Total number of exported functions:  16

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    royalties_handler
    (
        getLastClaimEpoch
        getShareholders
        claimNftMinterPaymentsAndRoyalties
        getNftMinterScAddress
        addShareholders
        removeShareholders
        claimRewards
        getClaimableEntryIdsForAddress
        claimableTokensForRewardEntry
        createNewRewardEntry
        getLastRewardEntryEpoch
        getLastEntryId
        getClaimWhitelistForEntry
        getTokenBalances
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
