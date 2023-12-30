fn main() {
    let mut x = vec![1, 2, 3];

    let last = x.last().unwrap();
    x.push(4);


    println!("{:?}", last);
}