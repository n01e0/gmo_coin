use gmo_coin::*;

fn main() {
    let resp = public::api::trades(Symbol::BTC, Some(1), Some(1));
    println!("{:#?}", resp);

    println!("testing public API...");
    test_exchange_status();
    test_ticker();
    test_orderbooks();
    test_trades();

    println!("testing private API...");
    
}

fn ok() {
    println!("ok");
}

fn failure() {
    println!("failure");
}

fn test_exchange_status() {
    print!("[+] test public::api::status ... ");
    if let Ok(resp) = public::api::status() {
        match &*resp.data.status {
            "MAINTENANCE" | "PREOPEN" | "OPEN" => ok(),
            _ => failure(),
        }
    } else {
        failure();
    }
}

fn test_ticker() {
    print!("[+] test public::api::ticker ... ");
    if let Ok(resp) = public::api::ticker(Some(Symbol::BTC)) {
        match &*resp.data[0].symbol {
            "BTC" => ok(),
            _ => failure()
        }
    } else {
        failure()
    }
}

fn test_orderbooks() {
    print!("[+] test public::api::orderbooks ... ");
    if let Ok(resp) = public::api::orderbooks(Symbol::BTC){
        match &*resp.data.symbol {
            "BTC" => ok(),
            _ => failure()
        }
    } else {
        failure()
    }
}

fn test_trades() {
    print!("[+] test public::api::trades ... ");
    let trades = public::api::trades(Symbol::BTC, Some(1), Some(1));
    if let Ok(resp) = trades {
        if resp.data.list.len() > 0 {
            ok()
        } else {
            failure()
        }
    } else {
        failure()
    }
}

