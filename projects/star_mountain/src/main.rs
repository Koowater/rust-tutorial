fn main() {
    println!(
        concat!(
            "[Star Mountain]\n", 
            " - Welcome to Star Mountain!\n",
            "   This program is a tutorial using loop in rust code."
        )
    );

    let mut cnt_loop: u32 = 0;
    
    println!("Shoot five stars.");
    loop {
        println!(" - Shooting star ~ ⭐");
        cnt_loop += 1;
        if cnt_loop >= 5 {
            break;
        }
    }
    
    cnt_loop = 0;
    println!("Shoot three stars.");
    while cnt_loop != 3 {
        println!(" - Shooting star ~ ⭐");
        cnt_loop += 1;
    }

    let stars = ["⭐", "💫", "🌟", "🌠" ];
    
    println!("Four gods of star in Star Mountain");
    for star in stars.iter() {
        println!(" - {star}: Hello.");
    }
    
    // print three gods of star using range.
    println!("Three gods of star in Star Mountain");
    for number in 1..4 {
        println!(" - {}: I said \"{}\".", stars[number], number);
    }
    println!("Reverse.");
    for number in (1..4).rev() {
        println!(" - {}: I said \"{}\".", stars[number], number);
    }
}
