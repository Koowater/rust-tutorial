fn do_something_awesome(x: i16){
    println!("This function will do something awesome, but not implemented yet!\nJust print {}", x);
    todo!("Please implement me...")
}

fn main(){
    let a_number = 8;
    do_something_awesome(a_number);
}
