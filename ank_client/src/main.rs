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
    clap::{
        App,
        Arg,
    },
};

// anchor program imports
use ank;
use ank::accounts as ank_accounts;
use ank::instruction as ank_instruction;

// running nightly requires extern
use alloc::rc::Rc;
extern crate alloc;

fn main() {
    let app = App::new("ank_client")
        .version("0.1.0")
        .about("using rust anchor client to interact with anchor because fun")
        .author("dubbelosix")
        .arg(
            Arg::new("keypair")
                .short('k')
                .long("keypair")
                .help("path to the json keypair file for paying")
                .takes_value(true)
                .required(true)
        );
    let matches = app.get_matches();
    let mut keypair_file: String = String::new();
    if let Some(i) = matches.value_of("keypair") {
        keypair_file = i.to_string()
    };
    let kp = read_keypair_file(keypair_file).unwrap();
    let c = Client::new(Cluster::Localnet, Rc::new(kp));
    let program_id: Pubkey = Pubkey::new(
        &bs58::decode("DoGiWHsUGypgyxtofEdD3qUYxTTjxewr54eGUqtJQQ4").into_vec().unwrap());
    let prog = c.program(program_id);
    let signature = prog.request()
                                            .accounts(ank_accounts::Initialize{})
                                            .args(ank_instruction::Initialize {})
                                            .send();
    println!("{:?}",signature);
}
