multiversx_sc::imports!();

use multiversx_sc_modules::pause;

use crate::brand_creation::ROYALTIES_MAX;
use crate::common_storage::{self, BrandId, EgldValuePaymentsVecPair};

#[multiversx_sc::module]
pub trait RoyaltiesModule:
    crate::admin_whitelist::AdminWhitelistModule
    + pause::PauseModule
    + common_storage::CommonStorageModule
{
    #[endpoint(setRoyaltiesClaimAddress)]
    fn set_royalties_claim_address(&self, new_address: ManagedAddress) {
        self.require_caller_is_admin();
        self.royalties_claim_address().set(&new_address);
    }

    #[endpoint(changeRoyaltiesForBrand)]
    fn change_royalties_for_brand(&self, brand_id: &BrandId<Self::Api>, new_royalties: BigUint) {
        self.require_caller_is_admin();
        let is_new_brand = self.registered_brands().insert(brand_id.clone());
        require!(!is_new_brand, "Brand doesn't exist");
        require!(
            new_royalties <= ROYALTIES_MAX,
            "Royalties cannot be over 100%"
        );
        self.brand_info(brand_id)
            .update(|brand| brand.royalties = new_royalties)
    }

    #[endpoint(setMintPaymentsClaimAddress)]
    fn set_mint_payments_claim_address(&self, new_address: ManagedAddress) {
        self.require_caller_is_admin();
        self.mint_payments_claim_address().set(&new_address);
    }

    #[endpoint(claimRoyalties)]
    fn claim_royalties(&self) -> EgldValuePaymentsVecPair<Self::Api> {
        self.require_not_paused();
        let royalties_claim_address = self.royalties_claim_address().get();
        let mut mapper = self.accumulated_royalties();

        self.claim_common(royalties_claim_address, &mut mapper)
    }

    #[endpoint(claimMintPayments)]
    fn claim_mint_payments(&self) -> EgldValuePaymentsVecPair<Self::Api> {
        self.require_not_paused();
        let mint_payments_claim_address = self.mint_payments_claim_address().get();
        let mut mapper = self.accumulated_mint_payments();

        self.claim_common(mint_payments_claim_address, &mut mapper)
    }

    fn claim_common(
        &self,
        claim_allowed_address: ManagedAddress,
        mapper: &mut MapMapper<EgldOrEsdtTokenIdentifier, BigUint>,
    ) -> EgldValuePaymentsVecPair<Self::Api> {
        let caller = self.blockchain().get_caller();
        require!(caller == claim_allowed_address, "Claim not allowed");

        let mut egld_value = BigUint::zero();
        let mut other_payments = ManagedVec::new();
        for (token, amount) in mapper.iter() {
            if token.is_egld() {
                egld_value = amount;
            } else {
                other_payments.push(EsdtTokenPayment::new(token.unwrap_esdt(), 0, amount));
            }
        }

        mapper.clear();

        if egld_value > 0 {
            self.send().direct_egld(&caller, &egld_value);
        }
        if !other_payments.is_empty() {
            self.send().direct_multi(&caller, &other_payments);
        }

        (egld_value, other_payments).into()
    }

    fn add_mint_payment(&self, token: EgldOrEsdtTokenIdentifier, amount: BigUint) {
        let mut mapper = self.accumulated_mint_payments();
        self.add_common(&mut mapper, token, amount);
    }

    fn add_royalties(&self, token: EgldOrEsdtTokenIdentifier, amount: BigUint) {
        let mut mapper = self.accumulated_royalties();
        self.add_common(&mut mapper, token, amount);
    }

    fn add_royalties_multiple(&self, payments: &ManagedVec<EsdtTokenPayment<Self::Api>>) {
        let mut mapper = self.accumulated_royalties();
        for p in payments {
            self.add_common(
                &mut mapper,
                EgldOrEsdtTokenIdentifier::esdt(p.token_identifier),
                p.amount,
            );
        }
    }

    fn add_common(
        &self,
        mapper: &mut MapMapper<EgldOrEsdtTokenIdentifier, BigUint>,
        token: EgldOrEsdtTokenIdentifier,
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
    fn accumulated_royalties(&self) -> MapMapper<EgldOrEsdtTokenIdentifier, BigUint>;

    #[view(getAccumulatedMintPayments)]
    #[storage_mapper("accumulatedMintPayments")]
    fn accumulated_mint_payments(&self) -> MapMapper<EgldOrEsdtTokenIdentifier, BigUint>;
}
