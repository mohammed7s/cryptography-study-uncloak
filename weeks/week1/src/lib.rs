pub fn vigenere(_key: &str , _message: &str ) {
   
    let m = _key.len(); 
    let key_vec: Vec<char> = _key.chars().collect();   // convert key into indexed vector 
    let mut i = 0; 
    let mut cipher_text = "".to_string();    

    for letter in _message.chars() {

        let mut k = key_vec[i % m] as u32 -97;   // convert key[i] to u32 unicode 
        let mut l = letter as u32 -97;           // convert message to u32 unicode 
        let mut c = (l + k) %26 + 97;             // Vigenere cipher logic
        let mut c_char = (char::from_u32(c)).unwrap();  // convert back to string characters  
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

pub fn sort_arr<T: Ord>(arr: &mut [T]) {
    sorting::bubble_sort(arr);
}

mod sorting {
    pub fn selection_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        for i in 0..len {
            let mut min_idx = i;
            for j in (i + 1)..len {
                if arr[j] < arr[min_idx] {
                    min_idx = j;
                }
            }
            arr.swap(min_idx, i);
        }
    }
    
    pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
        let len = arr.len();
        for i in 0..len {
            for j in 0..len - i - 1 {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_bubble_sort() {
            let mut arr = [6, 2, 4, 1, 9, -2, 5];
            bubble_sort(&mut arr);
            assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
        }

        #[test]
        fn test_selection_sort() {
            let mut arr = [6, 2, 4, 1, 9, -2, 5];
            selection_sort(&mut arr);
            assert_eq!(arr, [-2, 1, 2, 4, 5, 6, 9]);
        }
    }
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
