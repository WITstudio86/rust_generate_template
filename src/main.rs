fn main() {
    println!("{}", hello("tom"))
}

fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod test {
    use crate::hello;

    #[test]
    fn test_hello() {
        assert_eq!(hello("join"), "Hello, join!".to_string());
    }
}
