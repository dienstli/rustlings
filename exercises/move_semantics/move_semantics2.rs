// move_semantics2.rs
// Make me compile without changing line 13!
// Execute `rustlings hint move_semantics2` for hints :)

fn main() {
    let mut vec0 = Vec::new();

    // version 1
    //  let mut vec1 = fill_vec(vec0.clone());
    // version 2
    // let mut vec1 = fill_vec(&vec0);
    // version 3
    let mut vec1 = fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

// version 2
// fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
//     let mut vec = vec.to_vec();

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);

//     vec
// }

// version 3
fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.to_vec()
}
