elrond_wasm::imports!();
elrond_wasm::derive_imports!();

use nft_minter::common_storage::PaymentsVec;

pub const FIRST_ENTRY_ID: usize = 1;

#[derive(TypeAbi, TopEncode, TopDecode, PartialEq, Debug)]
pub struct RewardEntry<M: ManagedTypeApi> {
    pub egld_amount: BigUint<M>,
    pub esdt_payments: PaymentsVec<M>,
}

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

        let mut egld_amount = BigUint::zero();
        let mut esdt_payments = PaymentsVec::new();
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

            if token_id.is_egld() {
                egld_amount = amount_per_holder;
            } else {
                esdt_payments.push(EsdtTokenPayment::new(
                    token_id.unwrap_esdt(),
                    0,
                    amount_per_holder,
                ));
            }
        }

        if egld_amount > 0 || !esdt_payments.is_empty() {
            let entry_id = self.store_new_reward_entry(&RewardEntry {
                egld_amount,
                esdt_payments,
            });
            self.copy_shareholders_to_claim_whitelist(entry_id);
        }
    }

    fn store_new_reward_entry(&self, entry: &RewardEntry<Self::Api>) -> usize {
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

    #[view(getLastEntryId)]
    #[storage_mapper("lastEntryId")]
    fn last_entry_id(&self) -> SingleValueMapper<usize>;

    #[storage_mapper("claimableTokensForRewardEntry")]
    fn claimable_tokens_for_reward_entry(
        &self,
        entry_id: usize,
    ) -> SingleValueMapper<RewardEntry<Self::Api>>;

    #[view(getClaimWhitelistForEntry)]
    #[storage_mapper("claimWhitelistForEntry")]
    fn claim_whitelist_for_entry(&self, entry_id: usize) -> UnorderedSetMapper<ManagedAddress>;
}
