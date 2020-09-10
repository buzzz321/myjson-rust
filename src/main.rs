enum JType {
    JString,
    JNumber,
    JArray,
    JObject,
}

struct JSONValue {
    jtype: JType,
    strValue: String,
    arr: Vec<JSONValue>,
}

fn main() {
    println!("Hello, world!");
}
