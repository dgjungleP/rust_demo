fn main() {
    println!("Hello, world!");

    let mut pre = 0;
    let mut next = 1;
    let loop_index = 100;

    for _ in 0..loop_index {
        print!("{}\t", next);
        let temp = next;
        next = pre + temp;
        pre = temp;
    }
    println!();
}
