use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();
//    blockchain.set_current_dir_from_workspace("esdt-system-sc-mock");
    blockchain.register_contract(
        "mxsc:output/esdt-system-sc-mock.mxsc.json",
        esdt_system_sc_mock::ContractBuilder,
    );
    blockchain
}

#[test]
fn issue_rs() {
    world().run("scenarios/esdt_system_sc.scen.json");
}
