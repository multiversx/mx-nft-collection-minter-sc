use nft_minter::common_storage::EgldValuePaymentsVecPair;

elrond_wasm::imports!();

#[elrond_wasm::module]
pub trait TokenBalanceModule {
    fn add_balance(&self, token: EgldOrEsdtTokenIdentifier, amount: &BigUint) {
        self.balance_for_token(&token).update(|b| {
            *b += amount;
        });
        let _ = self.known_tokens().insert(token);
    }

    fn update_balance_from_results(&self, result: EgldValuePaymentsVecPair<Self::Api>) {
        let (egld_value, other_payments) = result.into_tuple();

        if egld_value > 0 {
            self.add_balance(EgldOrEsdtTokenIdentifier::egld(), &egld_value);
        }
        for p in &other_payments {
            self.add_balance(
                EgldOrEsdtTokenIdentifier::esdt(p.token_identifier),
                &p.amount,
            );
        }
    }

    #[view(getTokenBalances)]
    fn get_token_balances(
        &self,
    ) -> MultiValueEncoded<MultiValue2<EgldOrEsdtTokenIdentifier, BigUint>> {
        let mut balances = MultiValueEncoded::new();

        for token_id in self.known_tokens().iter() {
            let balance_for_token = self.balance_for_token(&token_id).get();
            if balance_for_token > 0 {
                balances.push((token_id, balance_for_token).into());
            }
        }

        balances
    }

    #[storage_mapper("knownTokens")]
    fn known_tokens(&self) -> UnorderedSetMapper<EgldOrEsdtTokenIdentifier>;

    #[storage_mapper("balanceForToken")]
    fn balance_for_token(&self, token_id: &EgldOrEsdtTokenIdentifier)
        -> SingleValueMapper<BigUint>;
}
