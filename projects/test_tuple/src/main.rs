fn main() {
    let mut tuple = ("Hello", "world", 9);
    println!("{:?}", tuple);
    
    tuple.1 = "my home";
    println!("{:?}", tuple);
}
