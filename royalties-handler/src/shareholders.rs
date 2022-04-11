elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::module]
pub trait ShareholdersModule:
    crate::common_storage::CommonStorageModule
    + crate::reward_entries::RewardEntriesModule
    + crate::token_balance::TokenBalanceModule
{
    #[only_owner]
    #[endpoint(addShareholders)]
    fn add_shareholders(&self, #[var_args] shareholders: MultiValueEncoded<ManagedAddress>) {
        let mut mapper = self.shareholders();
        for sh in shareholders {
            let _ = mapper.insert(sh);
        }
    }

    #[only_owner]
    #[endpoint(removeShareholders)]
    fn remove_shareholders(&self, #[var_args] shareholders: MultiValueEncoded<ManagedAddress>) {
        let mut mapper = self.shareholders();
        for sh in shareholders {
            let _ = mapper.swap_remove(&sh);
        }
    }

    #[view(claimableTokensForRewardEntry)]
    fn get_claimable_tokens_for_reward_entry(
        &self,
        entry_id: usize,
    ) -> MultiValueEncoded<MultiValue2<TokenIdentifier, BigUint>> {
        let mut result = MultiValueEncoded::new();
        let payments = self.claimable_tokens_for_reward_entry(entry_id).get();
        for p in &payments {
            result.push((p.token_identifier, p.amount).into());
        }

        result
    }
}
