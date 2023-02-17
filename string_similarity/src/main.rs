use std::cmp;


fn get_jaro_winkler_distance(str1: &str, str2: &str) -> f32 {
    if str1 == str2 {
        return 1.0;
    }

    let len1 = str1.len();
    let len2 = str2.len();

    if len1 == 0 || len2 == 0 {
        return 0.0;
    }

    let search_range = (cmp::max(len1, len2) / 2) as usize - 1;

    let mut n_matches = 0;

    let mut hash1 = vec![0; len1];
    let mut hash2 = vec![0; len2];

    for i in 0..len1 {
        let start = cmp::max(0, i - search_range) as usize;
        let end   = cmp::min(i + search_range + 1, len2);

        for j in start..end {
            if str1.chars().nth(i).unwrap() == str2.chars().nth(j).unwrap() && hash2[j] == 0 {
                hash1[i] = 1;
                hash2[j] = 1;
                n_matches += 1;
                break;
            }
        }
    }

    if n_matches == 0 {
        return 0.0;
    }

    let mut n_transpositions = 0;
    let mut idx = 0;
    for i in 0..len1 {
        if hash1[i] == 0 {
            continue;
        }

        while hash2[idx] == 0 {
            idx += 1;
        }

        if str1.chars().nth(i).unwrap() != str2.chars().nth(idx).unwrap() {
            n_transpositions += 1;
        }
        idx += 1;
    }
    let n_transpositions = (n_transpositions / 2) as f32;

    let n_matches = n_matches as f32;

    let jaro_similarity = ((n_matches / (len1 as f32)) + 
                           (n_matches / (len2 as f32)) + 
                           ((n_matches - n_transpositions) / n_matches)
                          ) / 3.0;
    return jaro_similarity;
}




fn main() {
    let str1 = "CRATEaskldj;fkjl;asdklj;dfsklj;afsdklj;kl;afdsklj;fadsklj;klj;afdsklj;dfsakjl;klj;afsdklasfdjkldsfalkjfkldasjkl;fdsakl;afsdkjladfsklj;faklsdj;kjl;".to_lowercase();
    let str2 = "TRACEkjalsdkjl;fakljjfq 3iooepijijopajsdlhkgh;ladfh;lgajs;dklhjgfkl;kja;lsdjklfklj;dasfkl;dfsajkadfl;sk;lafds;ljkfads;ljk;aljkfdl;kjafdl;kjadfsk;asdfasd".to_lowercase();
    let mut distance = 0.0;
    for _ in 0..1000 {
        distance = get_jaro_winkler_distance(&str1, &str2);
    }
    println!("String 1: {}", str1);
    println!("String 2: {}", str2);
    println!("Jaro-Winkler Distance: {}", distance);
}
