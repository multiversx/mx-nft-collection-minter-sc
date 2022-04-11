elrond_wasm::imports!();

use nft_minter::common_storage::PaymentsVec;

#[elrond_wasm::module]
pub trait RewardEntriesModule:
    crate::common_storage::CommonStorageModule + crate::token_balance::TokenBalanceModule
{
    #[only_owner]
    #[endpoint(createNewRewardEntry)]
    fn create_new_reward_entry(&self) {
        let current_epoch = self.blockchain().get_block_epoch();
        let last_claim_epoch = self.last_claim_epoch().get();
        require!(
            current_epoch == last_claim_epoch,
            "Must claim rewards for this epoch first"
        );

        self.last_reward_entry_epoch()
            .update(|last_reward_entry_epoch| {
                require!(
                    *last_reward_entry_epoch != current_epoch,
                    "Already created reward entry for this epoch"
                );

                *last_reward_entry_epoch = current_epoch;
            });

        let nr_shareholders = self.shareholders().len() as u32;
        require!(nr_shareholders > 0, "No shareholders");

        let mut rewards_entry = PaymentsVec::new();
        for token_id in self.known_tokens().iter() {
            let balance_mapper = self.balance_for_token(&token_id);
            let balance = balance_mapper.get();

            // nothing to split
            if balance < nr_shareholders {
                continue;
            }

            let amount_per_holder = &balance / nr_shareholders;
            let dust = balance - (&amount_per_holder * nr_shareholders);
            balance_mapper.set(&dust);

            rewards_entry.push(EsdtTokenPayment::new(token_id, 0, amount_per_holder));
        }

        let entry_id = self.store_new_reward_entry(&rewards_entry);
        self.copy_shareholders_to_claim_whitelist(entry_id);
    }

    fn store_new_reward_entry(&self, entry: &PaymentsVec<Self::Api>) -> usize {
        let new_entry_id = self.last_entry_id().update(|id| {
            *id += 1;
            *id
        });
        self.claimable_tokens_for_reward_entry(new_entry_id)
            .set(entry);

        new_entry_id
    }

    fn copy_shareholders_to_claim_whitelist(&self, entry_id: usize) {
        let mut new_mapper = self.claim_whitelist_for_entry(entry_id);
        for sh in self.shareholders().iter() {
            new_mapper.insert(sh);
        }
    }

    #[view(getLastRewardEntryEpoch)]
    #[storage_mapper("lastRewardEntryEpoch")]
    fn last_reward_entry_epoch(&self) -> SingleValueMapper<u64>;

    #[view(getFirstEntryId)]
    #[storage_mapper("firstEntryId")]
    fn first_entry_id(&self) -> SingleValueMapper<usize>;

    #[view(getLastEntryId)]
    #[storage_mapper("lastEntryId")]
    fn last_entry_id(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("claimableTokensForRewardEntry")]
    fn claimable_tokens_for_reward_entry(
        &self,
        entry_id: usize,
    ) -> SingleValueMapper<PaymentsVec<Self::Api>>;

    #[view(getClaimWhitelistForEntry)]
    #[storage_mapper("claimWhitelistForEntry")]
    fn claim_whitelist_for_entry(&self, entry_id: usize) -> UnorderedSetMapper<ManagedAddress>;
}
