use gmo_coin::*;

fn main() {
    let status = public::api::status();
    println!("{}", status.unwrap().status);
}
