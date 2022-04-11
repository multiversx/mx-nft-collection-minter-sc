elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait CommonStorageModule {
    #[view(getLastClaimEpoch)]
    #[storage_mapper("lastClaimEpoch")]
    fn last_claim_epoch(&self) -> SingleValueMapper<u64>;

    #[storage_mapper("accumulatedPayments")]
    fn accumulated_payments(&self) -> MapMapper<TokenIdentifier, BigUint>;
}
