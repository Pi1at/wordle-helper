use std::collections::HashMap;

#[allow(dead_code)]
/// Computes `HashMap` where key is mask and value is vector with words belong
/// to this mask
fn fast_partition(words: &[String], guess: &str) -> HashMap<u8, Vec<String>> {
    let mut partition: HashMap<u8, Vec<String>> = HashMap::new();
    for word in words {
        // get mask as u8
        let match_score = fast_score(word, guess);
        let list = partition.entry(match_score).or_default();
        list.push(word.clone());
    }
    partition
}

// this function just get correct mask and transform it into usize with radix 3
#[must_use]
pub fn fast_score(correct_solution: &str, attempt: &str) -> u8 {
    const ABSENT: u8 = 0;
    const PRESENT: u8 = 1;
    const CORRECT: u8 = 2;
    const LENGTH: usize = 5;
    let mut result = [0; LENGTH];
    let mut incorrect = [false; LENGTH];
    let mut unused = [false; LENGTH];
    for i in 0..LENGTH {
        if correct_solution.chars().nth(i) == attempt.chars().nth(i) {
            result[i] = CORRECT;
            incorrect[i] = false;
            unused[i] = false;
        } else {
            result[i] = ABSENT;
            incorrect[i] = true;
            unused[i] = true;
        }
    }

    for (i, v) in incorrect.iter().enumerate().take(LENGTH) {
        if *v {
            let correct_letter = correct_solution.chars().nth(i);
            for k in 0..LENGTH {
                if unused[k] && correct_letter == attempt.chars().nth(k) {
                    result[k] = PRESENT;
                    unused[k] = false;
                    break;
                }
            }
        }
    }
    result[4] + 3 * result[3] + 9 * result[2] + 27 * result[1] + 81 * result[0]
}

// guess project words into different mask only once
// very strange code
fn fast_does_fully_partition(words: &[&str], guess: &str) -> bool {
    const SIZE: usize = 3 * 3 * 3 * 3 * 3;
    let mut partition_check_array = vec![false; SIZE];

    for word in words {
        let match_score = fast_score(word, guess);
        if partition_check_array[match_score as usize] {
            return false;
        }
        partition_check_array[match_score as usize] = true;
    }
    true
}

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>; // For tests.

    use super::*;
    use crate::wordlist::{self, pattern::pattern_to_string};

    #[test]
    fn test_name() {
        let words = wordlists::TINKOFF_5BUKV_RU
            .lines()
            .map(String::from)
            .collect::<Vec<_>>();

        let res = words
            .iter()
            .map(|w| (w, fast_partition(&words, w)))
            .max_by_key(|(w, p)| p.len())
            .unwrap();
        println!("{} {}", res.0, res.1.len());
        // for guess in &words {
        //     let result = fast_partition(&words, guess);
        // }
    }

    #[test]
    fn test_score() {
        let result = fast_score("коран", "агент");
        println!("{} {}", result, pattern_to_string(result as usize));
    }
}
