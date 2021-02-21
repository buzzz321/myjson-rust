mod myjson;

fn main() {
    println!("Hello, world!");
    myjson::hello();
}

#[cfg(test)]
mod tests {
    use super::myjson;
    #[test]
    fn it_works() {
        myjson::hello();
        assert_eq!(2 + 2, 4);
    }
}
