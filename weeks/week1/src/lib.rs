pub fn vigenere(_key: &str , _message: &str ) {
   
    let m = _key.len(); 
    let key_vec: Vec<char> = _key.chars().collect();   // convert key into indexed vector 
    let mut i = 0; 
    let mut cipher_text = "".to_string();    

    for letter in _message.chars() {

        let mut k = key_vec[i % m] as u32 -97;  
        let mut l = letter as u32 -97; 
        let mut c = (l + k) %26 + 97; 
        let mut c_char = (char::from_u32(c)).unwrap(); 
        cipher_text.push(c_char); 
        i+=1; 
    }

    println!("cipher text: {}", cipher_text);
}

pub fn vigenere_decrypt(_key: &str, _cipher_text: &str) {
    let m = _key.len(); 
    let key_vec: Vec<char> = _key.chars().collect();   // convert key into indexed vector 
    let mut i = 0; 
    let mut message = "".to_string();  

    for letter in _cipher_text.chars() {
        let mut k = -1*(key_vec[i % m] as i32 -97);  
        let mut l = letter as i32 -97; 
        let mut c = ((l + k) %26 + 97) as u32; 
        let mut c_char = (char::from_u32(c)).unwrap(); 
        message.push(c_char); 
        i+=1; 
    }

    println!("message: {}", message); 
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
