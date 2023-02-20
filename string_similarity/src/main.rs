use std::time::Instant;

mod lib;

use crate::lib::get_jaro_winkler_similarity;


const N_ITERATIONS: usize = 10000;

fn main() {
    let str1 = "CRATdkjdakda;kflskl;afjdskl;jafsdkl;ajsakl;sfdE".to_lowercase().chars().collect();
    let str2 = "TRkjadlsk;jlasklj;afdslk;ajsfajk;lsfdACE".to_lowercase().chars().collect();
    let mut similarity = 0.0;
    let start = Instant::now();
    for _ in 0..N_ITERATIONS {
        similarity = get_jaro_winkler_similarity(&str1, &str2);
    }
    let duration = start.elapsed();
    println!("String 1: {:?}", str1);
    println!("String 2: {:?}", str2);
    println!("Jaro-Winkler Distance: {}", similarity);
    println!("Time Taken avg.: {:?}", duration / N_ITERATIONS as u32);
}


/*
fn main() {
    println!("Hello, world!");
}
*/
