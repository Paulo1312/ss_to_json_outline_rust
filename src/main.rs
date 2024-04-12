use ss_to_json_outline;
use serde_json;
 
fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1{
        print!("{}", serde_json::to_string(&ss_to_json_outline::ss_to_json(args[1].clone()).unwrap()).unwrap());
    }
    else{
        println!("usage: ss-to_json *your_key*");
    }
}

