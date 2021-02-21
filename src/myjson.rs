#[derive(Debug)]
pub enum JType {
    JString,
    JNumber,
    JArray,
    JObject,
}

#[derive(Debug)]
pub struct JSONValue {
    jtype: JType,
    str_value: &'static str,
    arr: Vec<JSONValue>,
}
trait Parser {
    fn consume_white_space(&mut self);
    fn get_data(&self) -> &str;
}

#[derive(Debug)]
pub struct ParserData {
    data: &'static str,
    curr_pos: usize,
}

impl Parser for ParserData {
    fn consume_white_space(&mut self) {
        for elem in self.data.chars().enumerate() {
            //println!("{} |{}|", self.curr_pos, elem.1);
            if elem.1 != ' ' && elem.1 != '\n' && elem.1 != '\t' && elem.1 != '\r' {
                break;
            }
            self.curr_pos += 1;
        }
    }
    fn get_data(&self) -> &str {
        //println!("{} ", self.curr_pos);
        &self.data[self.curr_pos..self.curr_pos + 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_data_test() {
        let uat = ParserData {
            data: ".",
            curr_pos: 0,
        };

        let ans = uat.get_data();
        assert_eq!(".", ans);
    }

    #[test]
    fn consume_whitespace_test() {
        let mut uat = ParserData {
            data: "           .",
            curr_pos: 0,
        };

        uat.consume_white_space();
        let ans = uat.get_data();
        assert_eq!(".", ans);
    }
}

pub fn hello() {
    println!("hello from myjson");
}
