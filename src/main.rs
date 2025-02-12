use polkavm_linker::{program_from_elf, Config};
use revive_runner::*;

// Is expected to return 0.
fn mload() {
    let elf = include_bytes!("mload.so");
    let mut config = Config::default();
    config.set_optimize(true);
    let blob = program_from_elf(config, elf).unwrap();

    // reproduce the bug
    let specs = Specs {
        actions: vec![
            SpecsAction::Instantiate {
                origin: TestAddress::Alice,
                value: 0,
                gas_limit: None,
                storage_deposit_limit: Some(10_000_000),
                code: Code::Bytes(blob),
                data: vec![1; 20],
                salt: Default::default(),
            },
            SpecsAction::Call {
                origin: TestAddress::Alice,
                dest: TestAddress::Instantiated(0),
                value: 0,
                gas_limit: None,
                storage_deposit_limit: Some(10_000_000),
                data: vec![0xe2, 0x17, 0x9b, 0x8e],
            },
            SpecsAction::VerifyCall(VerifyCallExpectation {
                output: vec![0; 32].into(),
                ..Default::default()
            }),
        ],
        ..Default::default()
    };

    let result = specs.run();

    dbg!(result);
}

// Traps with `OutOfBounds`, is expected to succeed (or another error but not OOB).
fn erc() {
    let elf = include_bytes!("erc.so");
    let mut config = Config::default();
    config.set_optimize(true);
    let blob = program_from_elf(config, elf).unwrap();

    // reproduce the bug
    let specs = Specs {
        actions: vec![SpecsAction::Instantiate {
            origin: TestAddress::Alice,
            value: 0,
            gas_limit: None,
            storage_deposit_limit: Some(100_000_000),
            code: Code::Bytes(blob),
            data: vec![
                00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 11, 11, 11, 11, 11, 11, 11, 11, 11,
                11, 11, 11, 11, 11, 11, 11, 11, 11, 11, 11,
            ],
            salt: Default::default(),
        }],
        ..Default::default()
    };

    let result = specs.run();

    dbg!(result);
}

fn main() {
    erc();
    mload();
}
