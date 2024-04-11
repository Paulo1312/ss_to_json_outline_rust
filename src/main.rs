use ss_to_json_outline;
use serde_json;
 
fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1{

        let shadow_socks_raw = ss_to_json_outline::clear_ss(args[1].clone());

        let text: String = ss_to_json_outline::decode_url(shadow_socks_raw.key).unwrap();

        let json = ss_to_json_outline::ShadowSocksJSON::from_decode_string(text, shadow_socks_raw.name);
        print!("{}", serde_json::to_string(&json.unwrap()).unwrap());
    }
    else{
        println!("usage: ss-to_json *your_key*");
    }
}

