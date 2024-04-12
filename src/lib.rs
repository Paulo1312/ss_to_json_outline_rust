use urlencoding::decode;

use std::fmt;

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

pub struct ShadowSocksRaw {
    pub key: String,
    pub name: String
}
#[macro_use] extern crate serde_derive;

#[derive(Serialize, Deserialize)]
pub struct ShadowSocksJSON {
    pub server: String,
    pub server_port: u32,
    pub local_port: u32,
    pub password: String,
    pub method : String,
    pub remarks: String,
}



impl ShadowSocksJSON {
    pub fn from_decode_string(decode_string: String, name: String) -> Result<ShadowSocksJSON, ParsingError> {
        let text: Vec<&str> = decode_string.split("@").collect();
        let server_address_port: Vec<&str> = text.last().ok_or(ParsingError)?.split(":").collect();
        let server_port = match server_address_port.last().ok_or(ParsingError)?.parse::<u32>(){
            Ok(data) => Ok(data),
            Err(_) => Err(ParsingError)
        }?;
        let server_method_password: Vec<&str> = text.first().ok_or(ParsingError)?.split(":").collect();

        Ok(ShadowSocksJSON {
            server: server_address_port.first().ok_or(ParsingError)?.to_string(),
            server_port: server_port,
            local_port: 1000_u32,
            password: server_method_password.last().ok_or(ParsingError)?.to_string(),
            method: server_method_password.first().ok_or(ParsingError)?.to_string(),
            remarks: name
        })
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

pub fn clear_ss(url_string: String) -> ShadowSocksRaw{
    let url_split = if url_string.contains("ss:"){ //Delete ss:// if ss address contains it
        url_string.split("ss://").collect::<Vec<&str>>()[1]
        
    }
    else{
        &url_string
    };

    if url_split.contains("#"){       
        ShadowSocksRaw {
            key: url_split.split("#").collect::<Vec<&str>>()[0].to_string(),
            name: decode(url_split.split("#").collect::<Vec<&str>>()[1]).expect("UTF-8").replace("+", " ").to_string()
        }               
    }       
    else{
        ShadowSocksRaw {
            key: url_split.split("#").collect::<Vec<&str>>()[0].to_string(),
            name: "No_Name".to_string()
        }
    }    
}

pub fn ss_to_json(starting: String) -> Result<ShadowSocksJSON, ParsingError> {
    let shadow_socks_raw = clear_ss(starting);
    let text: String = decode_url(shadow_socks_raw.key)?;

    let json = ShadowSocksJSON::from_decode_string(text, shadow_socks_raw.name)?;
    
    Ok(json)

}