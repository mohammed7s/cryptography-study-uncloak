use week3::hello; 
use hex_literal::hex;
use sha2::{Sha256, Sha512, Digest};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {

    hello(); 

    let arr = [1,2,3,4,5]; 
    println!("Length of the array{}", arr.len()); 

    // create a Sha256 object
    let mut hasher= Sha512::new();

    // write input message
    hasher.update(b"hello world");

    // read hash digest and consume hasher
    let result = hasher.finalize(); 

    assert_eq!(result[..], hex!("
    309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f
    ")[..]);

    print_type_of(&result);

    println!("Result is {:x?}", result); 
 
}
