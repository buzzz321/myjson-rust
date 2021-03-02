#[derive(Debug)]
pub enum JType {
    JString,
    JNumber,
    JArray,
    JObject,
}

#[derive(Debug)]
pub struct JSONValue<'a> {
    jtype: JType,
    str_value: &'a str,
    arr: Vec<JSONValue<'a>>,
}
trait Parser {
    fn parse_qouted_string(&mut self) -> String;
    fn consume_white_space(&mut self);
    fn consume(&mut self, token: &str) -> bool;
    fn get_data(&self) -> &str;
    fn is_digit(&self, val: &str) -> bool;
}

pub struct ParserData<'a> {
    data: &'a str,
    curr_pos: usize,
}

impl<'a> Parser for ParserData<'a> {
    fn parse_qouted_string(&mut self) -> String {
        self.consume_white_space();
        let mut iter = self.data.char_indices();

        let first: (usize, char) = iter.next().unwrap();
        if first.1 != '"' {
            return "".to_string();
        }
        let mut end_found = false;
        let mut end_pos: usize = 0;
        self.curr_pos += 1;

        for elem in iter {
            //println!("{}", elem.1);
            if elem.1 == '"' {
                end_found = true;
                end_pos = elem.0;
                break;
            }
        }

        if end_found {
            let ret_val = self.data[self.curr_pos..end_pos].to_string();
            self.curr_pos += end_pos;
            return ret_val;
        }
        return "".to_string();
    }

    fn consume_white_space(&mut self) {
        for elem in self.data.chars().enumerate() {
            //println!("{} |{}|", self.curr_pos, elem.1);
            if elem.1 != ' ' && elem.1 != '\n' && elem.1 != '\t' && elem.1 != '\r' {
                break;
            }
            self.curr_pos += 1;
        }
    }

    fn consume(&mut self, token: &str) -> bool {
        if token.chars().nth(0).unwrap() != self.data.chars().nth(self.curr_pos).unwrap() {
            return false;
        }
        self.curr_pos += token.len();

        if self.curr_pos > self.data.len() {
            self.curr_pos = self.data.len() - 1;
        }

        return true;
    }

    fn get_data(&self) -> &str {
        //println!("{} ", self.curr_pos);
        &self.data[self.curr_pos..self.curr_pos + 1]
    }

    fn is_digit(&self, val: &str) -> bool {
        for elem in val.chars().enumerate() {
            if elem.1 == '-' {
                continue;
            }
            let isnum = elem.1 as i32 - '0' as i32;
            if isnum <= 9 && isnum >= 0 {
                return true;
            } else {
                return false;
            }
        }
        false
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

    #[test]
    fn consume_dot_test() {
        let mut uat = ParserData {
            data: ".#",
            curr_pos: 0,
        };

        uat.consume(".");
        let ans = uat.get_data();
        assert_eq!("#", ans);
    }

    #[test]
    fn is_digits() {
        let uat = ParserData {
            data: "0",
            curr_pos: 0,
        };

        assert_eq!(true, uat.is_digit("0"));
        assert_eq!(true, uat.is_digit("9"));
        assert_eq!(false, uat.is_digit("a"));
        assert_eq!(true, uat.is_digit("-9"));
    }

    #[test]
    fn parse_key_test() {
        let mut uat = ParserData {
            data: "\"key:\": value",
            curr_pos: 0,
        };

        let ans = uat.parse_qouted_string();

        assert_eq!("key:", ans);
    }
}

pub fn hello() {
    println!("hello from myjson");
}
