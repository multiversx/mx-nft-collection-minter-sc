elrond_wasm::imports!();

use nft_minter::{common_storage::EgldValuePaymentsVecPair, royalties::ProxyTrait as _};

#[elrond_wasm::module]
pub trait NftMinterInteractorModule:
    crate::common_storage::CommonStorageModule + crate::token_balance::TokenBalanceModule
{
    #[only_owner]
    #[endpoint(claimNftMinterPaymentsAndRoyalties)]
    fn claim_nft_minter_payments_and_royalties(&self) {
        let current_epoch = self.blockchain().get_block_epoch();
        let last_claim_epoch = self.last_claim_epoch().get();
        require!(
            current_epoch > last_claim_epoch,
            "Already claimed this epoch"
        );

        self.last_claim_epoch().set(&current_epoch);

        let sc_address = self.nft_minter_sc_address().get();

        let royalties_result = self.call_claim_royalties(sc_address.clone());
        self.update_balance_from_results(royalties_result);

        let mint_payments_result = self.call_claim_mint_payments(sc_address);
        self.update_balance_from_results(mint_payments_result);
    }

    fn call_claim_royalties(
        &self,
        sc_address: ManagedAddress,
    ) -> EgldValuePaymentsVecPair<Self::Api> {
        self.nft_minter_proxy(sc_address)
            .claim_royalties()
            .execute_on_dest_context()
    }

    fn call_claim_mint_payments(
        &self,
        sc_address: ManagedAddress,
    ) -> EgldValuePaymentsVecPair<Self::Api> {
        self.nft_minter_proxy(sc_address)
            .claim_mint_payments()
            .execute_on_dest_context()
    }

    #[proxy]
    fn nft_minter_proxy(&self, sc_address: ManagedAddress) -> nft_minter::Proxy<Self::Api>;

    #[view(getNftMinterScAddress)]
    #[storage_mapper("nftMinterScAddress")]
    fn nft_minter_sc_address(&self) -> SingleValueMapper<ManagedAddress>;
}
