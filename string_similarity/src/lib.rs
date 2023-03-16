use std::cmp;

use pyo3::prelude::*;
use pyo3::types::PyString;


#[pyfunction]
#[pyo3(signature = "(str1, str2, max_prefix_length=4, scaling_factor=0.1)")]
fn jaro_winkler_similarity(
    _py: Python, 
    str1: &PyString, 
    str2: &PyString,
    max_prefix_length: Option<usize>,
    scaling_factor: Option<f32>,
    ) -> PyResult<f32> {
    unsafe {
        Ok(get_jaro_winkler_similarity_simd(
                &str1.to_string().as_bytes().to_vec(), 
                &str2.to_string().as_bytes().to_vec(),
                match max_prefix_length {
                    Some(x) => x,
                    None => 4,
                },
                match scaling_factor {
                    Some(x) => x,
                    None => 0.1,
                }
                ))
    }
}

#[pyfunction]
#[pyo3(signature = "(str1, str2, deletion_cost=1, insertion_cost=1, substitution_cost=1)")]
fn weighted_levenshtein_distance(
    _py: Python, 
    str1: &PyString, 
    str2: &PyString,
    deletion_cost: Option<usize>,
    insertion_cost: Option<usize>,
    substitution_cost: Option<usize>,
    ) -> PyResult<usize> {
    Ok(get_weighted_levenshtein_distance(
            &str1.to_string().as_bytes().to_vec(), 
            &str2.to_string().as_bytes().to_vec(),
            match deletion_cost {
                Some(x) => x,
                None => 1,
            },
            match insertion_cost {
                Some(x) => x,
                None => 1,
            },
            match substitution_cost {
                Some(x) => x,
                None => 1,
            }
            ))
}


#[pymodule]
fn weighted_levenshtein(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(jaro_winkler_similarity, m)?)?;
    m.add_function(wrap_pyfunction!(weighted_levenshtein_distance, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", env!("CARGO_PKG_AUTHORS"))?;
    m.add("__description__", env!("CARGO_PKG_DESCRIPTION"))?;
    Ok(())
}



#[target_feature(enable = "avx2")]
pub unsafe fn get_jaro_winkler_similarity_simd(
    str1: &Vec<u8>, 
    str2: &Vec<u8>,
    max_prefix_length: usize,
    scaling_factor: f32,
    ) -> f32 {
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


    let sim = ((n_matches as f32 / (len1 as f32)) + (n_matches as f32 / (len2 as f32)) + ((n_matches as f32 - n_transpositions) / n_matches as f32)) / 3.0;

    // Now get convert jaro to jaro_winkler_similarity
    let mut prefix = 0.0;
    for i in 0..cmp::min(max_prefix_length, cmp::min(len1, len2)) {
        if str1[i] == str2[i] {
            prefix += 1.0;
        } else {
            break;
        }
    }
    return sim + (prefix * scaling_factor * (1.0 - sim));


}


pub fn get_weighted_levenshtein_distance(
    str1: &Vec<u8>, 
    str2: &Vec<u8>,
    deletion_cost: usize,
    insertion_cost: usize,
    substitution_cost: usize
    ) -> usize {
    if str1 == str2 {
        return 1;
    }

    let len1 = str1.len();
    let len2 = str2.len();

    if len1 == 0 || len2 == 0 {
        return 0;
    }

    let mut table: Vec<Vec<usize>> = vec![vec![0; len2 + 1]; len1 + 1];

    for idx in 1..(len1 + 1) {
        table[idx][0] = table[idx - 1][0] + deletion_cost;
    }

    for idx in 1..(len2 + 1) {
        table[0][idx] = table[0][idx - 1] + insertion_cost;
    }

    for (idx, c1) in str1.iter().enumerate() {
        for (jdx, c2) in str2.iter().enumerate() {
            let sub_cost = if c1 == c2 { 0 } else { substitution_cost };
            table[idx + 1][jdx + 1] = cmp::min(
                cmp::min(
                    table[idx][jdx + 1] + deletion_cost, 
                    table[idx + 1][jdx] + insertion_cost
                    ), 
                table[idx][jdx] + sub_cost
                );
        }
    }
    return table[len1][len2];
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_error() {
        let str1: Vec<char> = "testdklfj;asdkljfakl;jsdlk;fjasklj;df".chars().collect();
        let str2: Vec<char> = "tasdklfaskl;djfjas;lkjdfkl;jasdest".chars().collect();

        let similarity_simd = get_jaro_winkler_similarity_simd(&str1, &str2);
        let similarity_wlev = get_weighted_levenshtein_distance(&str1, &str2, 1, 1, 1);

        assert!(similarity <= 1.0);
        assert!(similarity >= 0.0);
    }

}
