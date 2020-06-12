use gmo_coin::*;

fn main() {
    let status = public::api::status().unwrap();
    if &status.status == "OPEN" {
        let rate = public::api::ticker(Some(Symbol::BTC));
        println!("{:?}", rate);
    }
}
