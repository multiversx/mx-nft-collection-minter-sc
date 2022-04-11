elrond_wasm::imports!();

use nft_minter::{common_storage::EgldValuePaymentsVecPair, royalties::ProxyTrait as _};

#[elrond_wasm::module]
pub trait NftMinterInteractorModule: crate::common_storage::CommonStorageModule {
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

        let mut mapper = self.accumulated_payments();
        let sc_address = self.nft_minter_sc_address().get();

        let royalties_result = self.call_claim_royalties(sc_address.clone());
        self.update_balance_from_results(&mut mapper, royalties_result);

        let mint_payments_result = self.call_claim_mint_payments(sc_address);
        self.update_balance_from_results(&mut mapper, mint_payments_result);
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

    fn update_balance_from_results(
        &self,
        mapper: &mut MapMapper<TokenIdentifier, BigUint>,
        result: EgldValuePaymentsVecPair<Self::Api>,
    ) {
        let (egld_value, other_payments) = result.into_tuple();

        if egld_value > 0 {
            self.add_single_payment(mapper, TokenIdentifier::egld(), egld_value);
        }
        for p in &other_payments {
            self.add_single_payment(mapper, p.token_identifier, p.amount);
        }
    }

    fn add_single_payment(
        &self,
        mapper: &mut MapMapper<TokenIdentifier, BigUint>,
        token: TokenIdentifier,
        amount: BigUint,
    ) {
        match mapper.get(&token) {
            Some(mut prev_amount) => {
                prev_amount += amount;
                let _ = mapper.insert(token, prev_amount);
            }
            None => {
                let _ = mapper.insert(token, amount);
            }
        }
    }

    #[proxy]
    fn nft_minter_proxy(&self, sc_address: ManagedAddress) -> nft_minter::Proxy<Self::Api>;

    #[view(getNftMinterScAddress)]
    #[storage_mapper("nftMinterScAddress")]
    fn nft_minter_sc_address(&self) -> SingleValueMapper<ManagedAddress>;
}
