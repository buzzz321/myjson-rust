use std::fs;

use crate::myjson::{JSONValue, Parser, ParserData};

mod myjson;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string("fsp.json")?;

    let mut parser = ParserData {
        data: &data,
        curr_pos: 0,
    };
    let parsed: Option<JSONValue> = parser.parse_value();
    match parsed {
        Some(v) => {
            println!("{:?}", v);
        }
        None => {
            println!("Couldnt parse data");
        }
    }

    println!("{:?}", data);

    Ok(())
}
