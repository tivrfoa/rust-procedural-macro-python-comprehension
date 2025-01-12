use comp::comp;
fn main() {
    // no if
    let a: Vec<i32> = comp!(x for x in vec![1, 2, 3]).collect();
    dbg!(a);

    // // single if
    // let a: Vec<i32> = comp!(x for x in vec![1, 2, 3] if x > 1).collect();
    // dbg!(a);

    // // expression in mapping
    // let a: Vec<i32> = comp!(x * 3 for x in vec![1, 2, 3]).collect();
    // dbg!(a);
    
    // // Using a reference
    // let v: Vec<i32> = vec![1, 2, 3];
    // let a: Vec<i32> = comp!(x for x in v).collect();
    // dbg!(a);

    // // two ifs

    // // Testing && and ||
    // let a: Vec<i32> = comp!(x * 10 for x in vec![1, 2, 3] if x > 1 && (x == 2 || x == 3)).collect();
    // dbg!(a);
}
