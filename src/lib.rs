use std::string::FromUtf8Error;

pub fn decode_url(url_string: String) -> Result<String, FromUtf8Error>{
    use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
    let bytes_url = engine::GeneralPurpose::new(
                 &alphabet::URL_SAFE,
                 general_purpose::NO_PAD)
        .decode(url_string).unwrap();
    String::from_utf8(bytes_url)
}

pub fn clear_ss(url_string: String) -> String{
    let url_split = if url_string.contains("ss:"){
        url_string.split("ss://").collect::<Vec<&str>>()[1]
        
    }
    else{
        &url_string
    };
    if url_split.contains("#"){                       
        url_split.split("#").collect::<Vec<&str>>()[0]     
    }       
    else{
        &url_split
    }.to_string()      


}
