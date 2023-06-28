use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
    blockchain.set_current_dir_from_workspace("esdt-system-sc-mock");
    blockchain.register_contract(
        "file:output/esdt-system-sc-mock.wasm",
        esdt_system_sc_mock::ContractBuilder,
    );
    blockchain
}

#[test]
fn issue_rs() {
    multiversx_sc_scenario::run_rs("mandos/esdt_system_sc.scen.json", world());
}
