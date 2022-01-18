use {
    anchor_client::{
        Client,
        Cluster,
    },
    solana_sdk::{
        signer::keypair::read_keypair_file,
        pubkey::Pubkey,
    },
    anchor_lang::prelude::*,
};

// anchor program imports
use ank;
use ank::accounts as ank_accounts;
use ank::instruction as ank_instruction;

// running nightly requires extern
use alloc::rc::Rc;
extern crate alloc;

fn main() {
    let kp = read_keypair_file("/Users/rohan/.solw/dub.json").unwrap();
    let c = Client::new(Cluster::Localnet, Rc::new(kp));
    let program_id: Pubkey = Pubkey::new(
        &bs58::decode("DoGiWHsUGypgyxtofEdD3qUYxTTjxewr54eGUqtJQQ4").into_vec().unwrap());
    let prog = c.program(program_id);
    prog.request().accounts(ank_accounts::Initialize{}).args(ank_instruction::Initialize {}).send();
}
