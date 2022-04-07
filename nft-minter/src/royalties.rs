elrond_wasm::imports!();

use crate::common_storage::EgldValuePaymentsVecPair;

#[elrond_wasm::module]
pub trait RoyaltiesModule: crate::admin_whitelist::AdminWhitelistModule {
    #[endpoint(setRoyaltiesClaimAddress)]
    fn set_royalties_claim_address(&self, new_address: ManagedAddress) {
        self.require_caller_is_admin();
        self.royalties_claim_address().set(&new_address);
    }

    #[endpoint(setMintPaymentsClaimAddress)]
    fn set_mint_payments_claim_address(&self, new_address: ManagedAddress) {
        self.require_caller_is_admin();
        self.mint_payments_claim_address().set(&new_address);
    }

    #[endpoint(claimRoyalties)]
    fn claim_royalties(&self) -> EgldValuePaymentsVecPair<Self::Api> {
        let royalties_claim_address = self.royalties_claim_address().get();
        let mut mapper = self.accumulated_royalties();

        self.claim_common(royalties_claim_address, &mut mapper)
    }

    #[endpoint(claimMintPayments)]
    fn claim_mint_payments(&self) -> EgldValuePaymentsVecPair<Self::Api> {
        let mint_payments_claim_address = self.mint_payments_claim_address().get();
        let mut mapper = self.accumulated_mint_payments();

        self.claim_common(mint_payments_claim_address, &mut mapper)
    }

    fn claim_common(
        &self,
        claim_allowed_address: ManagedAddress,
        mapper: &mut MapMapper<TokenIdentifier, BigUint>,
    ) -> EgldValuePaymentsVecPair<Self::Api> {
        let caller = self.blockchain().get_caller();
        require!(caller == claim_allowed_address, "Claim not allowed");

        let mut egld_value = BigUint::zero();
        let mut other_payments = ManagedVec::new();
        for (token, amount) in mapper.iter() {
            if token.is_egld() {
                egld_value = amount;
            } else {
                other_payments.push(EsdtTokenPayment::new(token, 0, amount));
            }
        }

        mapper.clear();

        if egld_value > 0 {
            self.send().direct_egld(&caller, &egld_value, &[]);
        }
        if !other_payments.is_empty() {
            self.send().direct_multi(&caller, &other_payments, &[]);
        }

        (egld_value, other_payments).into()
    }

    fn add_mint_payment(&self, token: TokenIdentifier, amount: BigUint) {
        let mut mapper = self.accumulated_mint_payments();
        self.add_common(&mut mapper, token, amount);
    }

    fn add_royalties(&self, token: TokenIdentifier, amount: BigUint) {
        let mut mapper = self.accumulated_royalties();
        self.add_common(&mut mapper, token, amount);
    }

    fn add_royalties_multiple(&self, payments: &ManagedVec<EsdtTokenPayment<Self::Api>>) {
        let mut mapper = self.accumulated_royalties();
        for p in payments {
            self.add_common(&mut mapper, p.token_identifier, p.amount);
        }
    }

    fn add_common(
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

    #[view(getRoyaltiesClaimAddress)]
    #[storage_mapper("royaltiesClaimAddress")]
    fn royalties_claim_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getMintPaymentsClaimAddress)]
    #[storage_mapper("mintPaymentsClaimAddress")]
    fn mint_payments_claim_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getAccumulatedRoyalties)]
    #[storage_mapper("accumulatedRoyalties")]
    fn accumulated_royalties(&self) -> MapMapper<TokenIdentifier, BigUint>;

    #[view(getAccumulatedMintPayments)]
    #[storage_mapper("accumulatedMintPayments")]
    fn accumulated_mint_payments(&self) -> MapMapper<TokenIdentifier, BigUint>;
}
