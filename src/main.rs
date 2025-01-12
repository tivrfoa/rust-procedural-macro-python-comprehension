use comp::comp;
fn main() {
    test_echo_no_if();
    test_echo_reference_no_if();

    // test_single_if();

    // test_two_ifs();

    test_and();
}

fn test_echo_no_if() {
    let a: Vec<i32> = comp!(x for x in vec![1, 2, 3]).collect();
    dbg!(a);
}

fn test_expression_in_mapping() {
    let a: Vec<i32> = comp!(x * 3 for x in vec![1, 2, 3]).collect();
    dbg!(a);
}

fn test_echo_reference_no_if() {
    let v: Vec<i32> = vec![1, 2, 3];
    let a: Vec<i32> = comp!(x for x in v).collect();
    dbg!(a);
}

fn test_single_if() {
    let a: Vec<i32> = comp!(x for x in vec![1, 2, 3] if x > 1).collect();
    dbg!(a);
}

fn test_and() {
    let a: Vec<i32> = comp!(x for x in vec![1, 2, 3] if x > 1 && x == 2).collect();
    dbg!(a);
}
