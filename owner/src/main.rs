fn main() {
    println!("Hello, world!");
    let s = "hello";
    println!("{}", s);
    let mut s2 = String::from("world");
    s2.push_str(", is my world");
    println!("{}", s2);

    let world = first_word(&s2);

    println!("{}", world);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
