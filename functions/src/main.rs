use std::io;

fn main() {
    loop {
        let mut cntr = 0;

        println!("Input a number");
        let mut init = String::new();
        io::stdin()
            .read_line(&mut init)
            .expect("Failed to read line");

        let mut init: u128 = match init.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let init_cpy = init;

        while init != 1 {
            if init % 2 == 1 {
            init = times_three_plus_one(init);
            }
            else {
            init = divided_by_two(init);
            }
            cntr += 1;
        }
        println!("{} takes {} steps to reach 1", init_cpy, cntr);
    }
}

fn times_three_plus_one(x: u128) -> u128 {
    3 * x + 1
}

fn divided_by_two(x: u128) -> u128 {
    x / 2
}
