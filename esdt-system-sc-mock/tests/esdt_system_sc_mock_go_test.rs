use multiversx_sc_scenario::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn issue_go() {
    world().run("scenarios/esdt_system_sc.scen.json");
}
