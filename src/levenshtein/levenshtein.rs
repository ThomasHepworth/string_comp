use std::cmp::{max, min};

// https://stackoverflow.com/questions/16036140/highly-parallelizable-levenstein-distance-algorithm

fn smallest_of_three<T: Ord>(a: T, b: T, c: T) -> T {
    min(min(a, b), c)
}

pub fn levenshtein_distance(str1: &str, str2: &str) -> usize {
    // If either of the strings are empty, return the length of the non-empty string
    if str1.is_empty() | str2.is_empty() {
        return max(str1.len(), str2.len());
    }

    // Declare vector 'matrix_row' with elements from 0 to len(b).
    // For 'MALL' top_row starts as: [0, 1, 2, 3, 4]
    let mut matrix_row: Vec<usize> = (0..=str2.len()).collect();

    for (str1_index, str1_char) in str1.chars().enumerate() {
        for (str2_index, str2_char) in str2.chars().enumerate() {
            let cost_to_reach = if str1_char == str2_char {
                matrix_row[0]
            } else {
                smallest_of_three(
                    matrix_row[str2_index],     // Replacement cost
                    matrix_row[str2_index + 1], // Insertion cost
                    matrix_row[0],              // Deletion cost
                ) + 1
            };

            // So we can do this entirely in place, add the insertion cost
            // as the replacement cost.
            matrix_row[0] = matrix_row[str2_index + 1];
            matrix_row[str2_index + 1] = cost_to_reach;
        }
        // At the end of each iteration, replace our trailing value - needs to be +1
        // as enumerate indexes from 0
        matrix_row[0] = str1_index + 1;
    }
    matrix_row[str2.len()]
}
