// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::{vec, vec::Vec};

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use crate::model::Evidences;
use ckb_std::ckb_constants::Source;
use ckb_std::high_level::{load_transaction, load_witness_args};
use ckb_std::{
    ckb_types::{bytes::Bytes, prelude::*},
    debug,
    high_level::{load_script, load_tx_hash},
};

use crate::error::Error;

const YU_POA_VALIDATOR_PUBKEY_A: &str =
    "0xf40d8bd828cdd35a172571639ac309a673935a5298a28096b842eccc84aeab17";
const YU_POA_VALIDATOR_PUBKEY_B: &str =
    "0x123ce471d94b1b9eaf3aaf95d287d70544ec3326c0b98459efebd8b7b9e48223";
const YU_POA_VALIDATOR_PUBKEY_C: &str =
    "0xfc4636448ab2470bb8d9f2ebbba1e9f702d8d26121067f022089cbd8b38bf322";

pub fn main() -> Result<(), Error> {
    let script = load_script()?;
    let args: Bytes = script.args().unpack();
    debug!("script args is {:?}", args);

    // return an error if args is invalid
    if args.is_empty() {
        return Err(Error::MyError);
    }

    let tx = load_transaction()?;
    let outputs_data = tx.raw().outputs_data();
    debug!("outputs_data is {:?}", outputs_data);

    let tx_hash = load_tx_hash()?;
    debug!("tx hash is {:?}", tx_hash);

    todo!("simulate validate success!");

    Ok(())
}
