use polkavm_linker::{program_from_elf, Config};
use revive_runner::*;

const CALLEE_HASH: [u8; 32] = [
    18, 33, 95, 162, 209, 246, 157, 38, 237, 5, 183, 222, 86, 105, 100, 222, 148, 9, 244, 196, 163,
    60, 180, 193, 130, 168, 26, 231, 109, 14, 110, 203,
];

fn main() {
    let elf = include_bytes!("buggy.so");
    let mut config = Config::default();
    config.set_optimize(true); // doesn't repro if `false`
    let blob = program_from_elf(config, elf).unwrap();

    // reproduce the bug
    let specs = Specs {
        actions: vec![
            SpecsAction::Instantiate {
                origin: TestAddress::Alice,
                value: 100_000,
                gas_limit: None,
                storage_deposit_limit: Some(10_000_000),
                code: Code::Bytes(blob),
                data: vec![],
                salt: Default::default(),
            },
            SpecsAction::Upload {
                origin: TestAddress::Alice,
                code: Code::Bytes(include_bytes!("return_with_data.polkavm").to_vec()),
                storage_deposit_limit: Some(10_000_000),
            },
            SpecsAction::Call {
                origin: TestAddress::Alice,
                dest: TestAddress::Instantiated(0),
                value: 100_000,
                gas_limit: None,
                storage_deposit_limit: Some(10_000_000),
                data: CALLEE_HASH.to_vec(),
            },
        ],
        ..Default::default()
    };

    let result = specs.run();

    dbg!(result);
}
