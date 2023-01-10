use week1::{
    sort_arr,
    vigenere
};

// add some items from criterion crate  
use criterion::{
    black_box, 
    criterion_group, 
    criterion_main,
    Criterion 
}; 

// setup test function 
fn sort_arr_benchmark(c: &mut Criterion) {
    
    // unsorted array. black box function prevents compiler from optimizing away computations 
    // let mut arr = black_box(
    //     [6,2,4,1,9,-2,-5]
    // ); 

    // c is criterion struct instance
    c.bench_function(
        "Vigenere algorithm", 
        |b| b.iter(||vigenere("lemon", "mohammed"))
    ); 
}

// Macro: define collection of fucntion. configuration. args = name of group, names of functions. 
criterion_group!(benches, sort_arr_benchmark); 
// Macro: expands to main function which runs all benchmark in a given group
criterion_main!(benches); 