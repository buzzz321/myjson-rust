use std::{borrow::Borrow, collections::HashMap};

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
    pub str_value: String,
    pub arr: Vec<Box<JSONValue>>,
    pub object: HashMap<String, JSONValue>,
}
pub trait Parser {
    fn consume(&mut self, token: &str) -> bool;
    fn consume_white_space(&mut self);
    fn get_data(&self) -> &str;
    fn is_digit(&self) -> bool;
    fn parse_array(&mut self) -> Option<JSONValue>;
    fn parse_number(&mut self) -> String;
    fn parse_object(&mut self) -> Option<JSONValue>;
    fn parse_qouted_string(&mut self) -> String;
    fn parse_value(&mut self) -> Option<JSONValue>;
    fn peek(&mut self, token: &str, ahead: usize) -> bool;
}

#[derive(Debug, Default, Clone)]
pub struct ParserData<'a> {
    pub data: &'a str,
    pub curr_pos: usize,
}

impl<'a> Parser for ParserData<'a> {
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
    fn consume_white_space(&mut self) {
        let iter = self.data.char_indices().skip(self.curr_pos);

        for elem in iter {
            if elem.1 != ' ' && elem.1 != '\n' && elem.1 != '\t' && elem.1 != '\r' {
                break;
            }
            self.curr_pos += 1;
        }
    }
    fn get_data(&self) -> &str {
        &self.data[self.curr_pos..self.curr_pos + 1]
    }
    fn is_digit(&self) -> bool {
        let iter = self.data.char_indices().skip(self.curr_pos).peekable();
        for elem in iter {
            if elem.1 == '-' || elem.1 == '.' {
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

    fn parse_array(&mut self) -> Option<JSONValue> {
        /*
        "key" : [1, 2 , 3, 4]
        */
        let mut ret_val = JSONValue {
            ..Default::default()
        };

        ret_val.jtype = JType::JArray;
        self.consume_white_space();
        ret_val.str_value = self.parse_qouted_string();
        self.consume_white_space();
        self.consume(":");
        self.consume_white_space();

        if !self.consume("[") {
            return None;
        }

        let mut tmp = Vec::<Box<JSONValue>>::new();

        while !self.peek("]", 0) {
            self.consume_white_space();
            let ans = self.parse_value()?;
            tmp.push(Box::new(ans));
            self.consume_white_space();
            self.consume(",");
        }
        self.curr_pos += 1;
        ret_val.arr = tmp;
        Some(ret_val)
    }

    fn parse_number(&mut self) -> String {
        self.consume_white_space();
        let iter = self.data.char_indices().skip(self.curr_pos);
        let mut end_pos: usize = 0;
        let mut found = false;

        for elem in iter {
            let tmp = elem.1;
            if (!tmp.is_numeric() && tmp != '-' && tmp != '.') || tmp == ' ' {
                end_pos = elem.0;
                found = true;
                break;
            }
        }

        if found {
            let ret_val = self.data[self.curr_pos..end_pos].to_string();
            self.curr_pos += end_pos - self.curr_pos;
            return ret_val;
        }
        return "".to_string();
    }

    fn parse_object(&mut self) -> Option<JSONValue> {
        let mut ret_val = JSONValue {
            ..Default::default()
        };

        loop {
            // we only allow 1000000 nested objects..
            self.consume_white_space();
            if !self.consume("{") {
                break; // no start so bail out
            }

            if self.consume("}") {
                break;
            }

            self.consume_white_space();

            ret_val.jtype = JType::JObject;
            let key = self.parse_qouted_string();
            self.consume_white_space();
            self.consume(":");
            self.consume_white_space();
            let ans = self.parse_value()?;

            ret_val.object.insert(key, ans);
            self.consume_white_space();

            if !self.consume(",") {
                break;
            }
        }

        Some(ret_val)
    }

    fn parse_qouted_string(&mut self) -> String {
        self.consume_white_space();
        let mut iter = self.data.char_indices().skip(self.curr_pos);

        let first: (usize, char) = iter.next().unwrap();
        if first.1 != '"' {
            return "".to_string();
        }
        let mut end_found = false;
        let mut escape = false;
        let mut end_pos: usize = 0;
        self.curr_pos += 1;

        for elem in iter {
            if elem.1 == '\\' {
                escape = true;
            }
            if elem.1 == '"' && !escape {
                end_found = true;
                end_pos = elem.0;
                break;
            } else if escape {
                escape = false; //close the escape after we check for quote
            }
        }

        if end_found {
            let ret_val = self.data[self.curr_pos..end_pos].to_string();
            self.curr_pos = end_pos + 1;
            return ret_val;
        }
        return "".to_string();
    }

    fn parse_value(&mut self) -> Option<JSONValue> {
        let mut ret_val = JSONValue {
            ..Default::default()
        };
        self.consume_white_space();
        let ch = self.data[self.curr_pos..self.curr_pos + 1].borrow();
        if self.is_digit() {
            ret_val.str_value = self.parse_number();
            ret_val.jtype = JType::JNumber;
        } else if ch == "\"" {
            ret_val.str_value = self.parse_qouted_string();
            ret_val.jtype = JType::JString;
        } else if ch == "{" {
            ret_val = self.parse_object().unwrap();
            ret_val.jtype = JType::JObject;
        } else if ch == "[" {
            ret_val.arr.push(Box::new(self.parse_array()?));
            ret_val.jtype = JType::JArray;
        } else {
            println!("Error {}", ch.to_string());
            return None;
        }

        Some(ret_val)
    }

    fn peek(&mut self, token: &str, ahead: usize) -> bool {
        if self.data.chars().nth(self.curr_pos + ahead).unwrap() == token.chars().nth(0).unwrap() {
            true
        } else {
            false
        }
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
        let mut uat = ParserData {
            data: "0",
            curr_pos: 0,
        };

        assert_eq!(true, uat.is_digit());
        uat.data = "9";
        assert_eq!(true, uat.is_digit());
        uat.data = "a";
        assert_eq!(false, uat.is_digit());
        uat.data = "-9";
        assert_eq!(true, uat.is_digit());
        uat.data = "-9.01";
        assert_eq!(true, uat.is_digit());
    }

    #[test]
    fn parse_quoted_string_test() {
        let mut uat = ParserData {
            data: "\"key:\"",
            curr_pos: 0,
        };

        let ans = uat.parse_qouted_string();

        assert_eq!(r##"key:"##, ans);
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
        let mut uat = ParserData {
            data: " 123.45 ",
            curr_pos: 0,
        };

        let ans = uat.parse_value();

        assert_eq!("123.45", ans.unwrap().str_value);
    }
    #[test]
    fn parse_array_test() {
        let mut uat = ParserData {
            data: "\"arr\": [1, 2, 3, 4, 5, -6]",
            curr_pos: 0,
        };
        let ans = uat.parse_array();
        match ans {
            Some(v) => {
                assert_eq!("1", v.arr[0].str_value);
                assert_eq!("-6", v.arr[5].str_value);
            }
            None => {
                assert!(false)
            }
        }
    }
    #[test]
    fn parse_array_of_strings_test() {
        let mut uat = ParserData {
            data: "\"arr\": [\"1, 2, 3\", \"4, 5, -6\"]",
            curr_pos: 0,
        };
        let ans = uat.parse_array();
        match ans {
            Some(v) => {
                assert_eq!(r##"1, 2, 3"##, v.arr[0].str_value);
                assert_eq!(r##"4, 5, -6"##, v.arr[1].str_value);
            }
            None => {
                assert!(false)
            }
        }
    }

    #[test]
    fn parse_object_test() {
        let mut uat = ParserData {
            data: " {\"plura\": [1,2,3,4,5,6] }",
            curr_pos: 0,
        };
        let ans: Option<JSONValue> = uat.parse_value();
        match ans {
            Some(v) => {
                //println!("{:?}", v);
                assert_eq!("plura", v.object.keys().next().unwrap().to_string());
                // let val = *(v.arr[0]).clone();
                //assert_eq!(r##"1"##, *(val.arr[0]));
            }
            None => {
                assert!(false)
            }
        }
    }
    #[test]
    fn parse_objects_test() {
        let mut uat = ParserData {
            data: " {\"plura\": [1,2,3,4,5,6], \"mupp\": \"2-4\" }",
            curr_pos: 0,
        };
        let ans: Option<JSONValue> = uat.parse_value();
        match ans {
            Some(v) => {
                //println!("{:?}", v);
                assert_eq!("plura", v.object.keys().next().unwrap().to_string());
                // let val = *(v.arr[0]).clone();
                //assert_eq!(r##"1"##, *(val.arr[0]));
            }
            None => {
                assert!(false)
            }
        }
    }
}
