pub fn hello() {
    println!("Hello World!");
}


// fn sha512n(message: u128, n: u8) {
//     let mut hasher = Sha512::new(); 
// }


// fn sha512_n(message: &[u8], n: u8) -> Vec<u8> {
//     let num_bytes = n / 8;
//     let mut hasher = Sha512::new();

//     hasher.update(message);
//     let result: Vec<u8> = hasher.finalize().to_vec();

//     result[0..(num_bytes as usize)].to_vec()
// }



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }


// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }
