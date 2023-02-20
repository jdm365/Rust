use std::cmp;
use bitmaps::Bitmap;

use pyo3::prelude::*;
use pyo3::types::PyString;

// use std::arch::x86_64::*;

#[pyfunction]
fn jaro_winkler(_py: Python, str1: &PyString, str2: &PyString) -> PyResult<f32> {
    Ok(get_jaro_winkler_similarity(
            &str1.to_string().chars().collect(), 
            &str2.to_string().chars().collect()
            ))
}

#[pyfunction]
fn jaro_winkler_vector(_py: Python, str1: &PyString, str2: &PyString) -> PyResult<f32> {
    Ok(get_jaro_winkler_similarity_vector(
            &str1.to_string().as_bytes().to_vec(), 
            &str2.to_string().as_bytes().to_vec()
            ))
}

#[pyfunction]
fn jaro_winkler_simd(_py: Python, str1: &PyString, str2: &PyString) -> PyResult<f32> {
    unsafe {
        Ok(get_jaro_winkler_similarity_simd(
                &str1.to_string().as_bytes().to_vec(), 
                &str2.to_string().as_bytes().to_vec()
                ))
    }
}


#[pymodule]
fn string_similarity(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(jaro_winkler, m)?)?;
    m.add_function(wrap_pyfunction!(jaro_winkler_vector, m)?)?;
    m.add_function(wrap_pyfunction!(jaro_winkler_simd, m)?)?;
    Ok(())
}


pub fn get_jaro_winkler_similarity(str1: &Vec<char>, str2: &Vec<char>) -> f32 {
    if str1 == str2 {
        return 1.0;
    }

    let len1 = str1.len();
    let len2 = str2.len();

    if len1 == 0 || len2 == 0 {
        return 0.0;
    }

    let search_range = (cmp::max(len1, len2) / 2) - 1;

    let mut n_matches = 0;

    let mut hash1: Bitmap<1024> = Bitmap::new();
    let mut hash2: Bitmap<1024> = Bitmap::new();

    for i in 0..search_range {
        let end = cmp::min(len2, i + search_range + 1);

        for j in 0..end {
            if (str1[i] == str2[j]) && (!hash2.get(j)) {
                hash1.set(i, true);
                hash2.set(j, true);
                n_matches += 1;
                break;
            }
        }
    }

    for i in search_range..len1 {
        let end = cmp::min(len2, i + search_range + 1);

        for j in (i - search_range)..end {
            if (str1[i] == str2[j]) && (!hash2.get(j)) {
                hash1.set(i, true);
                hash2.set(j, true);
                n_matches += 1;
                break;
            }
        }
    }


    if n_matches == 0 {
        return 0.0;
    }

    let mut n_transpositions: f32 = 0.0;
    let mut idx;
    let mut i;


    match hash1.first_index() {
        Some(x) => {
            match hash2.first_index() {
                Some(y) => idx = y,
                None => panic!("String is empty"),
            }
            if str1[x] != str2[idx] {
                n_transpositions += 1.0;
            }
            i = x;
        },
        None => panic!("String is empty"),
    }


    loop {
        match hash1.next_index(i) {
            Some(x) => {
                match hash2.next_index(idx) {
                    Some(y) => {
                        idx = y;
                    },
                    None => {},
                }

                if str1[x] != str2[idx] {
                    n_transpositions += 1.0;
                }
                i = x;
            },
            None => break,
        }
    }
    n_transpositions = (0.5 * n_transpositions.floor()).floor();

    /*
    println!("====RUST====");
    println!("n_matches: {}", n_matches);
    println!("n_transpositions: {}", n_transpositions);
    println!("len1: {}", len1);
    println!("len2: {}", len2);
    */
    return ((n_matches as f32 / (len1 as f32)) + (n_matches as f32 / (len2 as f32)) + ((n_matches as f32 - n_transpositions) / n_matches as f32)) / 3.0;
}



pub fn get_jaro_winkler_similarity_vector(str1: &Vec<u8>, str2: &Vec<u8>) -> f32 {
    if str1 == str2 {
        return 1.0;
    }

    let len1 = str1.len();
    let len2 = str2.len();

    if len1 == 0 || len2 == 0 {
        return 0.0;
    }

    let search_range = (cmp::max(len1, len2) / 2) - 1;

    let mut n_matches = 0;

    let mut hash1: Vec<u8> = vec![0; len1];
    let mut hash2: Vec<u8> = vec![0; len2];

    /*
    for i in 0..search_range {
        let end = cmp::min(len2, i + search_range + 1);

        for j in 0..end {
            if (str1[i] == str2[j]) && (hash2[j] == 0) {
                hash1[i] = 1;
                hash2[j] = 1;
                n_matches += 1;
                break;
            }
        }
    }

    for i in search_range..len1 {
        let end = cmp::min(len2, i + search_range + 1);

        for j in (i - search_range)..end {
            if (str1[i] == str2[j]) && (hash2[j] == 0) {
                hash1[i] = 1;
                hash2[j] = 1;
                n_matches += 1;
                break;
            }
        }
    }
    */

    /*
    for i in 0..len1 {
        let start = cmp::max(0, i as i32 - search_range as i32) as usize;
        let end   = cmp::min(len2, i + search_range + 1);

        for j in start..end {
            if (str1[i] == str2[j]) && (hash2[j] == 0) {
                hash1[i] = 1;
                hash2[j] = 1;
                n_matches += 1;
                break;
            }
        }
    }
    */
    for i in 0..len1 {
        let start = cmp::max(0, i as i32 - search_range as i32) as usize;
        let end   = cmp::min(len2, i + search_range + 1);

        for j in start..end {
            if (str1[i] == str2[j]) && (hash2[j] == 0) {
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

    let mut n_transpositions: f32 = 0.0;
    let mut idx = 0;

    for i in 0..len1 {
        if hash1[i] == 1 {
            while hash2[idx] == 0 {
                idx += 1;
            }
            if str1[i] != str2[idx] {
                n_transpositions += 1.0;
            }
            idx += 1;
        }
    }
    n_transpositions = (0.5 * n_transpositions.floor()).floor();


    /*
    println!("====RUST====");
    println!("n_matches: {}", n_matches);
    println!("n_transpositions: {}", n_transpositions);
    println!("len1: {}", len1);
    println!("len2: {}", len2);
    */
    return ((n_matches as f32 / (len1 as f32)) + (n_matches as f32 / (len2 as f32)) + ((n_matches as f32 - n_transpositions) / n_matches as f32)) / 3.0;
}




#[target_feature(enable = "avx2")]
pub unsafe fn get_jaro_winkler_similarity_simd(str1: &Vec<u8>, str2: &Vec<u8>) -> f32 {
    if str1 == str2 {
        return 1.0;
    }

    let len1 = str1.len();
    let len2 = str2.len();

    if len1 == 0 || len2 == 0 {
        return 0.0;
    }

    let search_range = (cmp::max(len1, len2) / 2) - 1;

    let mut n_matches = 0;

    let mut hash1: Vec<u8> = vec![0; len1 + (len1 % 32)];
    let mut hash2: Vec<u8> = vec![0; len2 + (len2 % 32)];

    /*
    for i in 0..search_range {
        let end = cmp::min(len2, i + search_range + 1);

        for j in 0..end {
            if (str1[i] == str2[j]) && (hash2[j] == 0) {
                hash1[i] = 1;
                hash2[j] = 1;
                n_matches += 1;
                break;
            }
        }
    }

    for i in search_range..len1 {
        let end = cmp::min(len2, i + search_range + 1);

        for j in (i - search_range)..end {
            if (str1[i] == str2[j]) && (hash2[j] == 0) {
                hash1[i] = 1;
                hash2[j] = 1;
                n_matches += 1;
                break;
            }
        }
    }
    */

    /*
    for i in 0..len1 {
        let start = cmp::max(0, i as i32 - search_range as i32) as usize;
        let end   = cmp::min(len2, i + search_range + 1);

        for j in start..end {
            if (str1[i] == str2[j]) && (hash2[j] == 0) {
                hash1[i] = 1;
                hash2[j] = 1;
                n_matches += 1;
                break;
            }
        }
    }
    */

    for i in 0..len1 {
        let start = cmp::max(0, i as i32 - search_range as i32) as usize;
        let end   = cmp::min(len2, i + search_range + 1);

        for j in start..end {
            if (str1[i] == str2[j]) && (hash2[j] == 0) {
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

    let mut n_transpositions: f32 = 0.0;
    let mut idx = 0;

    for i in 0..len1 {
        if hash1[i] == 1 {
            while hash2[idx] == 0 {
                idx += 1;
            }
            if str1[i] != str2[idx] {
                n_transpositions += 1.0;
            }
            idx += 1;
        }
    }
    n_transpositions = (0.5 * n_transpositions.floor()).floor();


    /*
    println!("====RUST====");
    println!("n_matches: {}", n_matches);
    println!("n_transpositions: {}", n_transpositions);
    println!("len1: {}", len1);
    println!("len2: {}", len2);
    */
    return ((n_matches as f32 / (len1 as f32)) + (n_matches as f32 / (len2 as f32)) + ((n_matches as f32 - n_transpositions) / n_matches as f32)) / 3.0;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_error() {
        let str1: Vec<char> = "testdklfj;asdkljfakl;jsdlk;fjasklj;df".chars().collect();
        let str2: Vec<char> = "tasdklfaskl;djfjas;lkjdfkl;jasdest".chars().collect();

        let similarity = get_jaro_winkler_similarity(&str1, &str2);

        assert!(similarity <= 1.0);
        assert!(similarity >= 0.0);
    }

}
