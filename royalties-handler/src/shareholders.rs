elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use crate::reward_entries::{RewardEntry, FIRST_ENTRY_ID};

#[elrond_wasm::module]
pub trait ShareholdersModule:
    crate::common_storage::CommonStorageModule
    + crate::reward_entries::RewardEntriesModule
    + crate::token_balance::TokenBalanceModule
{
    #[only_owner]
    #[endpoint(addShareholders)]
    fn add_shareholders(&self, shareholders: MultiValueEncoded<ManagedAddress>) {
        let mut mapper = self.shareholders();
        for sh in shareholders {
            let _ = mapper.insert(sh);
        }
    }

    #[only_owner]
    #[endpoint(removeShareholders)]
    fn remove_shareholders(&self, shareholders: MultiValueEncoded<ManagedAddress>) {
        let mut mapper = self.shareholders();
        for sh in shareholders {
            let _ = mapper.swap_remove(&sh);
        }
    }

    #[endpoint(claimRewards)]
    fn claim_rewards(&self, entry_ids: MultiValueEncoded<usize>) {
        let caller = self.blockchain().get_caller();
        for entry_id in entry_ids {
            let mut whitelist_mapper = self.claim_whitelist_for_entry(entry_id);
            if !whitelist_mapper.contains(&caller) {
                continue;
            }

            let rewards_entry_mapper = self.claimable_tokens_for_reward_entry(entry_id);
            if rewards_entry_mapper.is_empty() {
                continue;
            }

            let reward_entry: RewardEntry<Self::Api> = rewards_entry_mapper.get();

            let _ = whitelist_mapper.swap_remove(&caller);
            if whitelist_mapper.is_empty() {
                rewards_entry_mapper.clear();
            }

            if reward_entry.egld_amount > 0 {
                self.send().direct_egld(&caller, &reward_entry.egld_amount);
            }
            if !reward_entry.esdt_payments.is_empty() {
                self.send()
                    .direct_multi(&caller, &reward_entry.esdt_payments);
            }
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
    ) -> MultiValueEncoded<MultiValue2<EgldOrEsdtTokenIdentifier, BigUint>> {
        let mut result = MultiValueEncoded::new();
        let reward_entry: RewardEntry<Self::Api> =
            self.claimable_tokens_for_reward_entry(entry_id).get();

        if reward_entry.egld_amount > 0 {
            result.push((EgldOrEsdtTokenIdentifier::egld(), reward_entry.egld_amount).into());
        }
        for p in &reward_entry.esdt_payments {
            result.push(
                (
                    EgldOrEsdtTokenIdentifier::esdt(p.token_identifier),
                    p.amount,
                )
                    .into(),
            );
        }

        result
    }
}
