#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use multiversx_sc_modules::pause::ProxyTrait as _;
use nft_minter::admin_whitelist::ProxyTrait as _;

mod events;
pub mod factory;

const MAX_DEPLOYS_PER_USER: usize = 50;

#[multiversx_sc::contract]
pub trait NftMinterDeployer: factory::FactoryModule + events::EventsModule {
    #[init]
    fn init(&self, nft_minter_template_address_opt: OptionalValue<ManagedAddress>) {
        self.nft_minter_creation_enabled().set_if_empty(false);

        if let OptionalValue::Some(address) = nft_minter_template_address_opt {
            self.nft_minter_template_address().set(address);
        }
    }

    #[endpoint(createNftMinter)]
    fn create_nft_minter_endpoint(
        &self,
        royalties_claim_address: ManagedAddress,
        mint_payments_claim_address: ManagedAddress,
        max_nfts_per_transaction: usize,
    ) -> ManagedAddress {
        let owner = self.blockchain().get_owner_address();
        let caller = self.blockchain().get_caller();

        if caller != owner {
            require!(
                self.nft_minter_creation_enabled().get(),
                "NFT minter creation is disabled"
            );
        }
        require!(
            self.user_nft_minter_contracts(&caller).len() < MAX_DEPLOYS_PER_USER,
            "Cannot exceed the maximum number of deploys allowed per user"
        );

        let nft_minter_address = self.create_nft_minter(
            royalties_claim_address.clone(),
            mint_payments_claim_address.clone(),
            max_nfts_per_transaction,
            caller.clone(),
        );

        self.emit_create_nft_minter_event(
            caller,
            royalties_claim_address,
            mint_payments_claim_address,
            max_nfts_per_transaction,
            nft_minter_address.clone(),
        );

        nft_minter_address
    }

    #[only_owner]
    #[endpoint(upgradeNftMinter)]
    fn upgrade_nft_minter_endpoint(
        &self,
        nft_minter_address: ManagedAddress,
        royalties_claim_address: ManagedAddress,
        mint_payments_claim_address: ManagedAddress,
        max_nfts_per_transaction: usize,
    ) {
        require!(
            self.all_nft_minter_contracts()
                .contains(&nft_minter_address),
            "NFT Minter contract does not exist"
        );

        self.upgrade_nft_minter(
            nft_minter_address.clone(),
            royalties_claim_address.clone(),
            mint_payments_claim_address.clone(),
            max_nfts_per_transaction,
        );

        self.emit_upgrade_nft_minter_event(
            self.blockchain().get_caller(),
            royalties_claim_address,
            mint_payments_claim_address,
            max_nfts_per_transaction,
            nft_minter_address,
        );
    }

    #[only_owner]
    #[endpoint(pauseNftMinter)]
    fn pause_nft_minter(&self, nft_minter_address: ManagedAddress) {
        require!(
            self.all_nft_minter_contracts()
                .contains(&nft_minter_address),
            "NFT Minter contract does not exist"
        );

        let _: IgnoreValue = self
            .user_nft_minter_proxy(nft_minter_address)
            .pause_endpoint()
            .execute_on_dest_context();
    }

    #[only_owner]
    #[endpoint(resumeNftMinter)]
    fn resume_nft_minter(&self, nft_minter_address: ManagedAddress) {
        require!(
            self.all_nft_minter_contracts()
                .contains(&nft_minter_address),
            "NFT Minter contract does not exist"
        );

        let _: IgnoreValue = self
            .user_nft_minter_proxy(nft_minter_address)
            .unpause_endpoint()
            .execute_on_dest_context();
    }

    #[only_owner]
    #[endpoint(addAdminToNftMinterContract)]
    fn add_admin_to_nft_minter_contract(
        &self,
        admin_address: ManagedAddress,
        nft_minter_address: ManagedAddress,
    ) {
        require!(
            self.all_nft_minter_contracts()
                .contains(&nft_minter_address),
            "NFT Minter contract does not exist"
        );

        let _: IgnoreValue = self
            .user_nft_minter_proxy(nft_minter_address)
            .add_user_to_admin_list(admin_address)
            .execute_on_dest_context();
    }

    #[only_owner]
    #[endpoint(removeAdminToNftMinterContract)]
    fn remove_admin_to_nft_minter_contract(
        &self,
        admin_address: ManagedAddress,
        nft_minter_address: ManagedAddress,
    ) {
        require!(
            self.all_nft_minter_contracts()
                .contains(&nft_minter_address),
            "NFT Minter contract does not exist"
        );

        let _: IgnoreValue = self
            .user_nft_minter_proxy(nft_minter_address)
            .remove_user_from_admin_list(admin_address)
            .execute_on_dest_context();
    }

    #[only_owner]
    #[endpoint(setNftMinterTemplateAddress)]
    fn set_nft_minter_template_address(&self, address: ManagedAddress) {
        self.nft_minter_template_address().set(&address);
    }

    #[only_owner]
    #[endpoint(setNftMinterCreationEnabled)]
    fn set_nft_minter_creation_enabled(&self, enabled: bool) {
        self.nft_minter_creation_enabled().set(enabled);
    }
}
