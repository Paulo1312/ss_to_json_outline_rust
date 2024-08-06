use regex::{Captures, Regex};
use std::fmt;
use urlencoding::decode;
pub struct ParsingError;

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error with file") // user-facing output
    }
}

impl fmt::Debug for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

#[macro_use] extern crate serde_derive;
#[derive(Serialize, Deserialize)]
pub struct ShadowSocksJSON {
    pub server: String,
    pub server_port: u32,
    pub local_address: String,
    pub local_port: u32,
    pub password: String,
    pub method : String,
    pub remarks: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShadowSocksJSONShort {
    pub server: String,
    pub server_port: u32,
    pub password: String,
    pub method : String,
    pub remarks: String,
}

impl ShadowSocksJSON {
    pub fn to_short(self) -> ShadowSocksJSONShort {
        ShadowSocksJSONShort{
            server: self.server,
            server_port: self.server_port,
            password: self.password,
            method: self.method,
            remarks: self.remarks
        }
    }
}

pub fn decode_url(url_string: String) -> Result<String, ParsingError>{
    use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
    let bytes_url = match  engine::GeneralPurpose::new(
                 &alphabet::URL_SAFE,
                 general_purpose::NO_PAD)
        .decode(url_string){
            Ok(data) => Ok(data),
            Err(_) => Err(ParsingError)
        }?;
    match String::from_utf8(bytes_url) {
        Ok(data) => Ok(data),
        Err(_) => Err(ParsingError)
    }
}

pub fn config_from_string_type_1(starting: Captures) -> Result<ShadowSocksJSON, ParsingError> {
    let new_re = Regex::new(r"(?P<Method>.+):(?P<Password>.+)@(?P<Address>.+)(?P<Port>\d{1,6})").unwrap();
    let text: String = decode_url(starting.name("base64text").unwrap().as_str().to_string()).unwrap().to_string();

    let data1 = new_re.captures(&text).unwrap();
    println!("TEST1");
    Ok(ShadowSocksJSON{
        server: data1.name("Address").unwrap().as_str().to_string(),
        server_port: match data1.name("Port").unwrap().as_str().parse::<u32>(){
            Ok(data3) => Ok(data3 as u32),
            Err(_) => Err(ParsingError)
        }?,
        local_address: "127.0.0.1".to_string(),
        local_port: 1000,
        password: data1.name("Password").unwrap().as_str().to_string(),
        method: data1.name("Method").unwrap().as_str().to_string(),
        remarks: decode(starting.name("Remarks").unwrap().as_str()).unwrap().replace("+", "_").to_string()
    })

}

pub fn config_from_string_type_2(starting: Captures) -> Result<ShadowSocksJSON, ParsingError> {
    println!("TEST2");
    let new_re = Regex::new(r"(?P<Method>.+):(?<Password>.+)").unwrap();
    let text: String = decode_url(starting.name("base64text").unwrap().as_str().to_string()).unwrap().to_string();
    
    let data1 = new_re.captures(&text).ok_or(ParsingError)?;
    Ok(ShadowSocksJSON{
        server: starting.name("IpAddr").ok_or(ParsingError)?.as_str().to_string(),
        server_port: match starting.name("Port").ok_or(ParsingError)?.as_str().parse::<u32>(){
            Ok(data3) => Ok(data3 as u32),
            Err(_) => Err(ParsingError)
        }?,
        local_address: "127.0.0.1".to_string(),
        local_port: 1000,
        password: data1.name("Password").ok_or(ParsingError)?.as_str().to_string(),
        method: data1.name("Method").ok_or(ParsingError)?.as_str().to_string(),
        remarks: decode(starting.name("Remarks").unwrap().as_str()).unwrap().replace("+", "_").to_string()
    })
}

pub fn ss_to_json(starting: String) -> Result<ShadowSocksJSON, ParsingError> {
    println!("TEST");
    // Old ss://<base64 coded config>#<remark>
    let type_1_ss_config_reges = Regex::new(r"(?P<Prefix>[s]{2}[:][/]{2})(?P<base64text>[[:alnum:]]+)#(?P<Remarks>\d{1,3}\+((%\w{2})|\+)+)").unwrap(); 
    // New ss://<base64 coded config>@<addr>:<port>?<type>prefix=<prefix> But this is configure for outline configs. 
    // May be you have other attributes. In this case you should do other thing (And, please, write to me)
    let type_2_ss_config_regex = Regex::new(r"[s]{2}[:][/]{2}(?P<base64text>.+)@(?P<IpAddr>((\d{1,3})(.\d{1,3}){3})|(.+)):(?P<Port>\d{1,6})/\?(?P<Params>((\w+)=.+)+)#(?P<Remarks>.+)").unwrap(); 

    let json = match type_1_ss_config_reges.captures(starting.as_str()){
        Some(data1) => {
            config_from_string_type_1(data1)
        },
        None => {
            let data1 = type_2_ss_config_regex.captures(starting.as_str()).ok_or(ParsingError)?;
            config_from_string_type_2(data1)
        }
    }?;

    Ok(json)

}


pub fn ss_to_json_port_address(starting: String, port: u32, address: String) -> Result<ShadowSocksJSON, ParsingError> {
    let mut old_json = ss_to_json(starting)?;
    old_json.local_address = address;
    old_json.local_port = port;
    
    Ok(old_json)

}

pub fn ss_to_json_short(starting: String) -> Result<ShadowSocksJSONShort, ParsingError> {
    Ok(ss_to_json(starting)?.to_short())
}
