// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           12
// Async Callback (empty):               1
// Total number of exported functions:  14

#![no_std]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    nft_minter_deployer
    (
        createNftMinter
        upgradeNftMinter
        pauseNftMinter
        resumeNftMinter
        addAdminToNftMinterContract
        removeAdminToNftMinterContract
        setNftMinterTemplateAddress
        setNftMinterCreationEnabled
        getUserNftMinterContracts
        getAllNftMinterContracts
        getNftMinterTemplateAddress
        getNftMinterCreationEnabled
    )
}

multiversx_sc_wasm_adapter::empty_callback! {}
