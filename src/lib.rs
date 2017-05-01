extern crate regex;

use std::env;
use std::io::{self, Read};
use std::collections::HashMap;
use regex::Regex;
use std::str;


const BLOCK_SIZE: usize = 256;

pub fn get_header(evk: &str) -> String{
    match env::var(evk) {
        Ok(val) => val,
        Err(e) => panic!("{}", e),
    }
}

fn percent_remove(payload: String) -> String{
    let mut out = String::from(&payload[..]);
    let re = Regex::new("%([0-9A-F]{2})*").unwrap();
    for cg in re.captures_iter(&payload[..]){
	let replacement = u8::from_str_radix(&cg[1], 16).unwrap();
        out = out.replace(&cg[0], str::from_utf8(&[replacement]).unwrap());
    }
    out.replace("+", " ")
}

pub fn get_payload() -> String{
    let mut out = String::new();
    let cont_len:usize = get_header("CONTENT_LENGTH").parse().unwrap();
    let mut read = 0usize; //That is the past tense of "read" not the present
    let mut buf = [0u8; BLOCK_SIZE];
    let mut stdin = io::stdin();
    while read < cont_len {
        match stdin.read(&mut buf[..]){
            Ok(n) => read += n,
            Err(er) => panic!("{}", er)
        }
        out += String::from_utf8_lossy(&buf).trim();
    }
    String::from(&out[0..cont_len])
}

pub fn get_payload_form_data() -> HashMap<String, String>{
    let mut out = HashMap::new();
    let payload = get_payload();
    let re = Regex::new("\\?*([^?/&=\\s]+)=([^?/&=\\s]+)&*").unwrap();
    for cg in re.captures_iter(&payload[..]){
        out.insert(cg[1].to_string(), percent_remove(cg[2].to_string()));
    }
    out
}

pub fn get_url_form_data() -> HashMap<String, String>{
    let mut out = HashMap::new();
    let payload = get_header("QUERY_STRING");
    let re = Regex::new("([\\w]+)=([\\w]+)&*").unwrap();
    for cg in re.captures_iter(&payload[..]){
        out.insert(cg[0].to_string(), cg[1].to_string());
    }
    out
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
