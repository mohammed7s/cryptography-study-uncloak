pub fn vigenere(_key: &str , _message: &str ) {
    println!("{}", _key);
    println!("{}", _message);
    println!("{}", _message.len()); 
}









pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
