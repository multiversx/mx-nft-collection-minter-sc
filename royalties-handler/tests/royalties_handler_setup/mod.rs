use crate::nft_minter_setup::NftMinterSetup;
use elrond_wasm::types::{Address, MultiValueEncoded};
use elrond_wasm_debug::{
    managed_address, rust_biguint,
    testing_framework::{BlockchainStateWrapper, ContractObjWrapper},
    tx_mock::TxResult,
    DebugApi,
};
use nft_minter::royalties::RoyaltiesModule;
use royalties_handler::nft_minter_interactor::NftMinterInteractorModule;
use royalties_handler::reward_entries::RewardEntriesModule;
use royalties_handler::shareholders::ShareholdersModule;
use royalties_handler::RoyaltiesHandler;

pub struct RoyaltiesHandlerSetup<RoyaltiesHandlerObjBuilder>
where
    RoyaltiesHandlerObjBuilder: 'static + Copy + Fn() -> royalties_handler::ContractObj<DebugApi>,
{
    pub b_mock: BlockchainStateWrapper,
    pub owner_address: Address,
    pub first_shareholder_address: Address,
    pub second_shareholder_address: Address,
    pub third_shareholder_address: Address,
    pub nft_minter_address: Address,
    pub rh_wrapper:
        ContractObjWrapper<royalties_handler::ContractObj<DebugApi>, RoyaltiesHandlerObjBuilder>,
}

impl<RoyaltiesHandlerObjBuilder> RoyaltiesHandlerSetup<RoyaltiesHandlerObjBuilder>
where
    RoyaltiesHandlerObjBuilder: 'static + Copy + Fn() -> royalties_handler::ContractObj<DebugApi>,
{
    pub fn new<NftMinterObjBuilder>(
        nm_builder: NftMinterObjBuilder,
        rh_builder: RoyaltiesHandlerObjBuilder,
    ) -> Self
    where
        NftMinterObjBuilder: 'static + Copy + Fn() -> nft_minter::ContractObj<DebugApi>,
    {
        let rust_zero = rust_biguint!(0);
        let mut nm_setup = NftMinterSetup::new(nm_builder);
        nm_setup.create_default_brands();

        let mut b_mock = nm_setup.b_mock;
        let owner_address = nm_setup.owner_address;
        let nm_wrapper = nm_setup.nm_wrapper;

        let first_shareholder_address = b_mock.create_user_account(&rust_zero);
        let second_shareholder_address = b_mock.create_user_account(&rust_zero);
        let third_shareholder_address = b_mock.create_user_account(&rust_zero);

        // init royalties handler SC
        let rh_wrapper =
            b_mock.create_sc_account(&rust_zero, Some(&owner_address), rh_builder, "roy path");

        b_mock
            .execute_tx(&owner_address, &rh_wrapper, &rust_zero, |sc| {
                let mut sh_addresses = MultiValueEncoded::new();
                sh_addresses.push(managed_address!(&first_shareholder_address));
                sh_addresses.push(managed_address!(&second_shareholder_address));
                sh_addresses.push(managed_address!(&third_shareholder_address));

                sc.init(managed_address!(nm_wrapper.address_ref()), sh_addresses);
            })
            .assert_ok();

        // set the roylaties handler SC as the claim address
        b_mock
            .execute_tx(&owner_address, &nm_wrapper, &rust_zero, |sc| {
                sc.set_royalties_claim_address(managed_address!(rh_wrapper.address_ref()));
                sc.set_mint_payments_claim_address(managed_address!(rh_wrapper.address_ref()));
            })
            .assert_ok();

        Self {
            b_mock,
            owner_address,
            first_shareholder_address,
            second_shareholder_address,
            third_shareholder_address,
            nft_minter_address: nm_wrapper.address_ref().clone(),
            rh_wrapper,
        }
    }

    pub fn call_claim_payments(&mut self) -> TxResult {
        self.b_mock.execute_tx(
            &self.owner_address,
            &self.rh_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.claim_nft_minter_payments_and_royalties();
            },
        )
    }

    pub fn call_create_new_reward_entry(&mut self) -> TxResult {
        self.b_mock.execute_tx(
            &self.owner_address,
            &self.rh_wrapper,
            &rust_biguint!(0),
            |sc| {
                sc.create_new_reward_entry();
            },
        )
    }

    pub fn call_claim_rewards(&mut self, caller: &Address, entry_ids: &[usize]) -> TxResult {
        self.b_mock
            .execute_tx(&caller, &self.rh_wrapper, &rust_biguint!(0), |sc| {
                let mut args = MultiValueEncoded::new();
                for id in entry_ids {
                    args.push(*id);
                }

                sc.claim_rewards(args);
            })
    }
}
