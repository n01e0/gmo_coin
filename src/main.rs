use gmo_coin::*;
use std::str::FromStr;

fn main() {
    let resp = private::api::active_orders(Symbol::BTC, Some(1), Some(1));
    println!("{:#?}", resp);

    println!("testing public API...");
    test_exchange_status();
    test_ticker();
    test_orderbooks();
    test_trades();

    println!("testing private API...");
    test_margin();
    test_assets();
    test_active_orders();
    test_latest_executions();
    test_from_str();
}

fn ok() {
    println!("ok");
}

fn failure() {
    println!("failure");
}

fn test_exchange_status() {
    print!("[+] test public::api::status ... ");
    match public::api::status() {
        Ok(resp) => {
            match &*resp.data.status {
                "MAINTENANCE" | "PREOPEN" | "OPEN" => ok(),
                _ => failure(),
            }
        },
        Err(e) => {
            failure();
            eprintln!("{:?}", e);
        }
    }
}

fn test_ticker() {
    print!("[+] test public::api::ticker ... ");
    match public::api::ticker(Some(Symbol::BTC)) {
        Ok(resp) => {
            match &*resp.data[0].symbol {
                "BTC" => ok(),
                _ => failure()
            }
        },
        Err(e) => {
            failure();
            eprintln!("{:?}", e);
        }
    }
}

fn test_orderbooks() {
    print!("[+] test public::api::orderbooks ... ");
    match public::api::orderbooks(Symbol::BTC) {
        Ok(resp) => {
            match &*resp.data.symbol {
                "BTC" => ok(),
                _ => failure()
            }
        },
        Err(e)=> {
            failure();
            eprintln!("{:?}", e);
        }
    }
}

fn test_trades() {
    print!("[+] test public::api::trades ... ");
    match public::api::trades(Symbol::BTC, Some(1), Some(1)) {
        Ok(resp) => {
            if !resp.data.list.is_empty() {
                ok()
            } else {
                failure()
            }
        },
        Err(e) => {
            failure();
            eprintln!("{:?}", e);
        }
    }
}

fn test_margin() {
    print!("[+] test private::api::margin ... ");
    match private::api::margin() {
        Ok(_) => ok(),
        Err(e) => {
            failure();
            eprintln!("{:?}", e);
        }
    }
}

fn test_assets() {
    print!("[+] test private::api::assets ... ");
    match private::api::assets() {
        Ok(_) => ok(),
        Err(e) => {
            failure();
            eprintln!("{:?}", e);
        }
    }
}

fn test_active_orders() {
    print!("[+] test private::api::active_orders ... ");
    match private::api::active_orders(Symbol::BTC, Some(1), Some(1)) {
        Ok(_) => ok(),
        Err(e) => {
            failure();
            eprintln!("{:?}", e);
        }
    }
}

fn test_latest_executions() {
    print!("[+] test private::api::latest_executions ... ");
    match private::api::latest_executions(Symbol::BTC, None, None) { 
        Ok(_) => ok(),
        Err(e) => {
            failure();
            eprintln!("{:?}", e);
        }
    }
}

fn test_from_str() {
    print!("[+] test ::from_str ...");
    match Symbol::from_str("BTC") {
        Ok(_) => ok(),
        Err(e) => {
            failure();
            eprintln!("{}", e);
        },
    }

    match Side::from_str("buy") {
        Ok(_) => ok(),
        Err(e) => {
            failure();
            eprintln!("{:?}", e);
        }
    }
}
