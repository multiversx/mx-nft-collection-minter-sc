multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait CommonStorageModule {
    #[view(getLastClaimEpoch)]
    #[storage_mapper("lastClaimEpoch")]
    fn last_claim_epoch(&self) -> SingleValueMapper<u64>;

    #[view(getShareholders)]
    #[storage_mapper("shareholders")]
    fn shareholders(&self) -> UnorderedSetMapper<ManagedAddress>;
}
