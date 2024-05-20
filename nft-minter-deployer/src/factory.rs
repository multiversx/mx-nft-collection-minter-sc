multiversx_sc::imports!();
multiversx_sc::derive_imports!();
pub use nft_minter;

#[multiversx_sc::module]
pub trait FactoryModule {
    fn create_nft_minter(
        &self,
        royalties_claim_address: ManagedAddress,
        mint_payments_claim_address: ManagedAddress,
        max_nfts_per_transaction: usize,
        admin: ManagedAddress,
    ) -> ManagedAddress {
        require!(
            !self.nft_minter_template_address().is_empty(),
            "Nft minter contract template is empty"
        );
        let new_address = self
            .tx()
            .typed(nft_minter::nft_minter_proxy::NftMinterProxy)
            .init(
                royalties_claim_address,
                mint_payments_claim_address,
                max_nfts_per_transaction,
                OptionalValue::Some(admin.clone()),
            )
            .code_metadata(
                CodeMetadata::UPGRADEABLE | CodeMetadata::READABLE | CodeMetadata::PAYABLE_BY_SC,
            )
            .from_source(self.nft_minter_template_address().get())
            .returns(ReturnsNewManagedAddress)
            .sync_call();

        self.user_nft_minter_contracts(&admin)
            .insert(new_address.clone());
        self.all_nft_minter_contracts().insert(new_address.clone());

        new_address
    }

    fn upgrade_nft_minter(
        &self,
        nft_minter_address: ManagedAddress,
        royalties_claim_address: ManagedAddress,
        mint_payments_claim_address: ManagedAddress,
        max_nfts_per_transaction: usize,
    ) {
        self.tx()
            .to(nft_minter_address)
            .typed(nft_minter::nft_minter_proxy::NftMinterProxy)
            .upgrade(
                &royalties_claim_address,
                &mint_payments_claim_address,
                &max_nfts_per_transaction,
                OptionalValue::None::<ManagedAddress>,
            )
            .code_metadata(
                CodeMetadata::UPGRADEABLE | CodeMetadata::READABLE | CodeMetadata::PAYABLE_BY_SC,
            )
            .from_source(self.nft_minter_template_address().get())
            .upgrade_async_call_and_exit();
    }

    #[view(getUserNftMinterContracts)]
    #[storage_mapper("userNftMinterContracts")]
    fn user_nft_minter_contracts(
        &self,
        user: &ManagedAddress,
    ) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getAllNftMinterContracts)]
    #[storage_mapper("allNftMinterContracts")]
    fn all_nft_minter_contracts(&self) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getNftMinterTemplateAddress)]
    #[storage_mapper("nftMinterTemplateAddress")]
    fn nft_minter_template_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getNftMinterCreationEnabled)]
    #[storage_mapper("nftMinterCreationEnabled")]
    fn nft_minter_creation_enabled(&self) -> SingleValueMapper<bool>;
}
