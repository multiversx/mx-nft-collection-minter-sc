multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[derive(TypeAbi, TopEncode)]
pub struct CreateNftMinterEvent<M: ManagedTypeApi> {
    royalties_claim_address: ManagedAddress<M>,
    mint_payments_claim_address: ManagedAddress<M>,
    max_nfts_per_transaction: usize,
    nft_minter_address: ManagedAddress<M>,
}

#[multiversx_sc::module]
pub trait EventsModule {
    fn emit_create_nft_minter_event(
        self,
        caller: ManagedAddress,
        royalties_claim_address: ManagedAddress,
        mint_payments_claim_address: ManagedAddress,
        max_nfts_per_transaction: usize,
        nft_minter_address: ManagedAddress,
    ) {
        self.create_nft_minter_event(
            caller,
            self.blockchain().get_block_epoch(),
            self.blockchain().get_block_nonce(),
            self.blockchain().get_block_timestamp(),
            CreateNftMinterEvent {
                royalties_claim_address,
                mint_payments_claim_address,
                max_nfts_per_transaction,
                nft_minter_address,
            },
        )
    }

    fn emit_upgrade_nft_minter_event(
        self,
        caller: ManagedAddress,
        royalties_claim_address: ManagedAddress,
        mint_payments_claim_address: ManagedAddress,
        max_nfts_per_transaction: usize,
        nft_minter_address: ManagedAddress,
    ) {
        self.upgrade_nft_minter_event(
            caller,
            self.blockchain().get_block_epoch(),
            self.blockchain().get_block_nonce(),
            self.blockchain().get_block_timestamp(),
            CreateNftMinterEvent {
                royalties_claim_address,
                mint_payments_claim_address,
                max_nfts_per_transaction,
                nft_minter_address,
            },
        )
    }

    #[event("createNftMinter")]
    fn create_nft_minter_event(
        self,
        #[indexed] caller: ManagedAddress,
        #[indexed] epoch: u64,
        #[indexed] block: u64,
        #[indexed] timestamp: u64,
        nft_minter_event: CreateNftMinterEvent<Self::Api>,
    );

    #[event("upgradeNftMinter")]
    fn upgrade_nft_minter_event(
        self,
        #[indexed] caller: ManagedAddress,
        #[indexed] epoch: u64,
        #[indexed] block: u64,
        #[indexed] timestamp: u64,
        nft_minter_event: CreateNftMinterEvent<Self::Api>,
    );
}
