fn main() {
    println!("Hello, world!");
    let number = 3;
    if number !=0{
        println!("condition was true");
    }else{
        println!("condition was false");
    }

    for i in (0..4).rev() {
        println!("{}!",i);
    }
}
