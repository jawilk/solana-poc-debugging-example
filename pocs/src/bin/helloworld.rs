use std::{env, str::FromStr};

use poc_framework::{
    keypair, solana_sdk::signer::Signer, Environment, LocalEnvironment, PrintableTransaction,
};
// Anchor
//use anchor_client::solana_sdk::system_instruction;
//use anchor_client::{RequestBuilder, RequestNamespace};
//use helloworld::accounts as helloworld_accounts;
//use helloworld::instruction as helloworld_instruction;

use solana_program::{native_token::sol_to_lamports, pubkey::Pubkey, system_program};

use solana_program::instruction::{AccountMeta, Instruction};

pub fn main() {
    let _env = setup();
}

fn setup() -> LocalEnvironment {
    let mut dir = env::current_exe().unwrap();
    let path_hello_world_binary = {
        dir.pop();
        dir.pop();
        dir.pop();
        dir.push("tests/elfs/helloworld_rust_unoptimized.so");
        dir.to_str()
    }
    .unwrap();

    let helloworld_program =
        Pubkey::from_str("H311ot3333333333333333333333333333333333333").unwrap();
    let payer = keypair(0);
    let greeting_account = keypair(1);
    let data: [u8; 4] = [0; 4];

    let mut env = LocalEnvironment::builder()
        .add_program(helloworld_program, path_hello_world_binary)
        .add_programs_to_debug(&[&helloworld_program])
        .add_account_with_lamports(payer.pubkey(), system_program::ID, sol_to_lamports(1.0))
        .add_account_with_data(greeting_account.pubkey(), helloworld_program, &data, false)
        .build();

    env.execute_as_transaction(
        &[Instruction {
            program_id: helloworld_program,
            accounts: vec![AccountMeta::new(greeting_account.pubkey(), true)],
            data: vec![1, 2, 3],
        }],
        &[&greeting_account],
    )
    .print();

    env
}
