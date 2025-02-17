
pub fn collection_demo() {
    vector_demo();
}

fn vector_demo() {
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);
    println!("{:?}", v);
}