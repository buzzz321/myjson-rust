#[derive(Debug, Clone)]
pub enum JType {
    JString,
    JNumber,
    JArray,
    JObject,
}

impl Default for JType {
    fn default() -> Self {
        JType::JObject
    }
}

#[derive(Debug, Default, Clone)]
pub struct JSONValue {
    pub jtype: JType,
    pub str_key: String,
    pub str_value: String,
    pub arr: Vec<Box<JSONValue>>,
}
trait Parser {
    fn parse_array(&mut self) -> Option<JSONValue>;
    fn parse_value(&mut self) -> Box<JSONValue>;
    fn parse_qouted_string(&mut self) -> String;
    fn parse_number(&mut self) -> String;
    fn peek(&mut self, token: &str, ahead: usize) -> bool;
    fn consume_white_space(&mut self);
    fn consume(&mut self, token: &str) -> bool;
    fn get_data(&self) -> &str;
    fn is_digit(&self, val: &str) -> bool;
}

#[derive(Debug, Default, Clone)]
pub struct ParserData<'a> {
    data: &'a str,
    curr_pos: usize,
}


impl<'a> Parser for ParserData<'a> {
    fn parse_array(&mut self) -> Option<JSONValue> {
        /*
        "key" : [1, 2 , 3, 4]
        */
        let mut ret_val = JSONValue {
            ..Default::default()
        };
    
        ret_val.jtype = JType::JArray;
        self.consume_white_space();
        ret_val.str_key = self.parse_qouted_string();
        self.consume_white_space();
        self.consume(":");
        self.consume_white_space();
    
        if !self.consume("[") {
            return None;
        }
    
        self.consume_white_space();
    
        let mut tmp = Vec::<Box<JSONValue>>::new();
       
        while ! self.peek("]", 0) {
           let ans = self.parse_value();
           tmp.push(ans);          
        }
        ret_val.arr = tmp;
        Some(ret_val)
    }
    fn parse_value(&mut self) -> Box<JSONValue> {
        let mut ret_val = JSONValue {
            ..Default::default()
        };
        self.consume_white_space();
        let ch: char = self.data.chars().nth(self.curr_pos).unwrap();

        if self.is_digit(&ch.to_string()) {
            ret_val.str_value = self.parse_number();
            ret_val.jtype = JType::JNumber;
        } else if ch == '{' {
            todo!();
        } else {
            println!("Error {}", ch);
        }

        Box::new(ret_val)
    }

    fn parse_qouted_string(&mut self) -> String {
        self.consume_white_space();
        let mut iter = self.data.char_indices().skip(self.curr_pos);

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

    fn parse_number(&mut self) -> String {
        self.consume_white_space();
        let iter = self.data.char_indices().skip(self.curr_pos);
        let mut end_pos: usize = 0;
        let mut found = false;

        for elem in iter {
            let tmp = elem.1;
            //println!("=>{}",tmp);
            if !tmp.is_numeric() && tmp != '-' && tmp != '.' {
                end_pos = elem.0;
                found = true;
            }
        }

        //println!(" start pos = {}",self.curr_pos);
        if found {
            let ret_val = self.data[self.curr_pos..end_pos].to_string();
            self.curr_pos += end_pos;
            return ret_val;
        }
        return "".to_string();
    }

    fn peek(&mut self, token: &str, ahead: usize) -> bool {
        if self.data.chars().nth(self.curr_pos + ahead).unwrap() == token.chars().nth(0).unwrap() {
            true
        } else {
            false
        }
    }

    fn consume_white_space(&mut self) {
        let iter = self.data.char_indices().skip(self.curr_pos);

        for elem in iter {
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
        for elem in val.char_indices().skip(self.curr_pos) {
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
    #[test]
    fn parse_number_test() {
        let mut uat = ParserData {
            data: " 123.45 ",
            curr_pos: 0,
        };

        let ans = uat.parse_number();

        assert_eq!("123.45", ans);
    }

    #[test]
    fn parse_value_test() {
        println!("Start test value");
        let mut uat = ParserData {
            data: " 123.45 ",
            curr_pos: 0,
        };

        let ans = uat.parse_value();

        assert_eq!("123.45", (*ans).str_value);
    }
    #[test]
    fn parse_array_test() {
        println!("Start test value");
        let mut uat = ParserData {
            data: "\"arr\": [1 2 3 4 5 6]",
            curr_pos: 0,
        };
        let ans = uat.parse_array();
        match ans{
            Some(v) => {
                assert_eq!("1", v.arr[0].str_value);
            }
            None => {
                assert!(false)
            }
        }
        
    }
}

pub fn hello() {
    println!("hello from myjson");
}
