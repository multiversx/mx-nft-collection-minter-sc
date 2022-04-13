elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::reward_entries::FIRST_ENTRY_ID;

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

    #[endpoint(claimRewards)]
    fn claim_rewards(&self, #[var_args] entry_ids: MultiValueEncoded<usize>) {
        let caller = self.blockchain().get_caller();
        for entry_id in entry_ids {
            let mut whitelist_mapper = self.claim_whitelist_for_entry(entry_id);
            if !whitelist_mapper.contains(&caller) {
                continue;
            }

            let rewards_entry_mapper = self.claimable_tokens_for_reward_entry(entry_id);
            let payments = rewards_entry_mapper.get();

            let _ = whitelist_mapper.swap_remove(&caller);
            if whitelist_mapper.is_empty() {
                rewards_entry_mapper.clear();
            }

            self.send().direct_multi(&caller, &payments, &[]);
        }
    }

    #[view(getClaimableEntryIdsForAddress)]
    fn get_claimable_entry_ids_for_address(
        &self,
        address: ManagedAddress,
        nr_entries_to_look_back: usize,
    ) -> MultiValueEncoded<usize> {
        let mut result = MultiValueEncoded::new();
        let last_id = self.last_entry_id().get();
        if last_id == 0 {
            return result;
        }

        let first_id = if nr_entries_to_look_back >= last_id {
            FIRST_ENTRY_ID
        } else {
            last_id - nr_entries_to_look_back
        };

        for id in first_id..=last_id {
            if self.claim_whitelist_for_entry(id).contains(&address) {
                result.push(id);
            }
        }

        result
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
