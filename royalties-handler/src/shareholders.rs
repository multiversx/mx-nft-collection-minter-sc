use nft_minter::common_storage::PaymentsVec;

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TypeAbi, TopEncode, TopDecode)]
pub struct RewardsForEntryNumber<M: ManagedTypeApi> {
    pub rewards: PaymentsVec<M>,
}

#[elrond_wasm::module]
pub trait ShareholdersModule: crate::common_storage::CommonStorageModule {
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

        let mut mapper = self.accumulated_payments();
        for it in mapper.iter() {
            let (token_id, amount): (TokenIdentifier, BigUint) = it;

            // nothing to split
            if amount < nr_shareholders {
                continue;
            }

            let amount_per_holder = &amount / nr_shareholders;
            let dust = amount - (&amount_per_holder * nr_shareholders);
            let _ = mapper.insert(token_id.clone(), dust);
        }
    }

    fn store_new_reward_entry(&self, entry: RewardsForEntryNumber<Self::Api>) {}

    #[view(getLastRewardEntryEpoch)]
    #[storage_mapper("lastRewardEntryEpoch")]
    fn last_reward_entry_epoch(&self) -> SingleValueMapper<u64>;

    #[view(getFirstEntryNumber)]
    #[storage_mapper("firstEntryNumber")]
    fn first_entry_number(&self) -> SingleValueMapper<usize>;

    #[view(getLastEntryNumber)]
    #[storage_mapper("lastEntryNumber")]
    fn last_entry_number(&self) -> SingleValueMapper<usize>;

    #[view(claimableTokensForRewardEntry)]
    #[storage_mapper("claimableTokensForRewardEntry")]
    fn claimable_tokens_for_reward_entry(
        &self,
        nr_entry: usize,
    ) -> SingleValueMapper<RewardsForEntryNumber<Self::Api>>;

    #[view(getClaimWhitelistForEntry)]
    #[storage_mapper("claimWhitelistForEntry")]
    fn claim_whitelist_for_entry(&self, nr_entry: usize) -> UnorderedSetMapper<ManagedAddress>;

    #[view(getShareholders)]
    #[storage_mapper("shareholders")]
    fn shareholders(&self) -> UnorderedSetMapper<ManagedAddress>;
}
