extern crate usdg;
extern crate env_logger;
extern crate rustc_hex;
extern crate web3;

use web3::contract::{Contract, Options};
use web3::futures::Future;
use web3::types::{Address, U256};

fn main() {
    env_logger::init();
    let (eloop, http) = web3::transports::Http::new("http://localhost:8545").unwrap();
    // run the event loop in the background
    eloop.into_remote();

    let web3 = web3::Web3::new(http);

    let my_account: Address = "d028d24f16a8893bd078259d413372ac01580769".parse().unwrap();

    // Accessing existing contract
    let contract_address: Address = "4a10d2c8A97e98Db164E945d750C4b96CcC895d8".parse().unwrap();
    let contract = Contract::from_json(
        web3.eth(),
        contract_address,
        include_bytes!("./contract/USDG.json"),
    )
        .unwrap();

    let result = contract.query("balanceOf", (my_account,), None, Options::default(), None);
    let balance_of: U256 = result.wait().unwrap();
    println!("{}",balance_of);
}