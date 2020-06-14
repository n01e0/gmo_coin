use gmo_coin::*;

fn main() {
    let status = public::api::status().unwrap();
    if &status.data.status == "OPEN" {
        println!("{:#?}", public::api::orderbooks(Symbol::BTC));
    }
}
