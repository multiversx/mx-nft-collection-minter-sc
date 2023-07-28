multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait FactoryModule {
    fn create_nft_minter(
        &self,
        collections_category: ManagedBuffer,
        royalties_claim_address: ManagedAddress,
        mint_payments_claim_address: ManagedAddress,
        max_nfts_per_transaction: usize,
        admin: ManagedAddress,
    ) -> ManagedAddress {
        require!(
            !self.nft_minter_template_address().is_empty(),
            "Nft minter contract template is empty"
        );
        let (new_address, ()) = self
            .nft_minter_contract_proxy()
            .init(
                collections_category,
                royalties_claim_address,
                mint_payments_claim_address,
                max_nfts_per_transaction,
                OptionalValue::Some(admin.clone()),
            )
            .deploy_from_source(
                &self.nft_minter_template_address().get(),
                CodeMetadata::UPGRADEABLE | CodeMetadata::READABLE | CodeMetadata::PAYABLE_BY_SC,
            );

        self.user_nft_minter_contracts(&admin)
            .insert(new_address.clone());
        self.all_nft_minter_contracts().insert(new_address.clone());

        new_address
    }

    fn upgrade_nft_minter(
        &self,
        nft_minter_address: ManagedAddress,
        collections_category: ManagedBuffer,
        royalties_claim_address: ManagedAddress,
        mint_payments_claim_address: ManagedAddress,
        max_nfts_per_transaction: usize,
    ) {
        self.nft_minter_contract_proxy()
            .contract(nft_minter_address)
            .init(
                collections_category,
                royalties_claim_address,
                mint_payments_claim_address,
                max_nfts_per_transaction,
                OptionalValue::None::<ManagedAddress>,
            )
            .upgrade_from_source(
                &self.nft_minter_template_address().get(),
                CodeMetadata::UPGRADEABLE | CodeMetadata::READABLE | CodeMetadata::PAYABLE_BY_SC,
            );
    }

    #[proxy]
    fn nft_minter_contract_proxy(&self) -> nft_minter::Proxy<Self::Api>;

    #[proxy]
    fn user_nft_minter_proxy(&self, to: ManagedAddress) -> nft_minter::Proxy<Self::Api>;

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
