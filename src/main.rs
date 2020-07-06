extern crate rand;

use std::env;
use std::result::Result;
use ::enigma::rotors::*;
use ::enigma::encoder::*;

enum Cmd {
    Generate,
    Encode,
    Decode,
}

struct Config {
    cmd: Cmd,
    input: String,
}

fn main() {
    let config = parse_args().unwrap();

    match config.cmd {
        Cmd::Generate => {
            rotors::generate().unwrap();
        }
        Cmd::Encode => {
            println!("Encoding input");
            let mut rotors: [String; 3] = rotors::read().unwrap();
            let encoded = encoder::encode(&mut rotors, &config.input);

            println!("{}", encoded);
        }
        Cmd::Decode => {
            println!("Decoding input");
            let mut rotors: [String; 3] = rotors::read().unwrap();
            let decoded = encoder::encode(&mut rotors, &config.input);
    
            println!("{}", decoded);
        }
    }
}

fn parse_args() -> Result<Config, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 0 {
        return Err(String::from("No argument specified"));
    }

    if args[1].to_lowercase() == "generate" {
        return Ok(Config {cmd: Cmd::Generate, input: String::new()});
    }
    if args[1].to_lowercase() == "encode" {
        if args.len() == 2 {
            return Err(String::from("Please enter string to encode"))
        }
        return Ok(Config {cmd: Cmd::Encode, input: args[2].to_lowercase()});
    }
    if args[1].to_lowercase() == "decode" {
        if args.len() == 2 {
            return Err(String::from("Please enter string to decode"))
        }
        return Ok(Config {cmd: Cmd::Decode, input: args[2].to_lowercase()});
    }

    Err(String::from("Invalid command"))
}
