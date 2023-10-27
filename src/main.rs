use ss_to_json_outline;
 
fn main() {
    use std::env;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1{
        let text = ss_to_json_outline::decode_url(ss_to_json_outline::clear_ss(args[1].clone())).unwrap();
        println!("{}", text);
    }
    else{
        println!("usage: ss-to_json *your_key*");
    }
}

