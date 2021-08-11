// You are given a license key represented as a string s that consists of only alphanumeric characters and dashes.
// The string is separated into n + 1 groups by n dashes. You are also given an integer k.

// We want to reformat the string s such that each group contains exactly k characters, except for the first group, 
// which could be shorter than k but still must contain at least one character. 
// Furthermore, there must be a dash inserted between two groups, and you should convert all lowercase letters to uppercase.

// Return the reformatted license key.
struct Solution {

}

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let mut result = "".to_string();
        let mut c_count = 0;
        for (i, c) in s.chars().rev().enumerate() {     
            let is_dash = c == '-';
            let needs_dash = (c_count) % k == 0 && !is_dash && c_count != 0;

            if !is_dash {
                c_count += 1;
            }

            if needs_dash {
                result.push('-');
            }
            if !is_dash {
                let char_upper = if c.is_ascii() { c.to_ascii_uppercase()} else { c };
                result.push(char_upper);
            }
        }

        return result.chars().rev().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_case_one() {
        let s = "5F3Z-2e-9-w".to_string();
        let k = 4;
        let result = Solution::license_key_formatting(s, k);
        assert_eq!(result, "5F3Z-2E9W".to_string());
    }

    #[test]
    fn test_solution_case_two() {
        let s = "2-5g-3-J".to_string();
        let k = 2;
        let result = Solution::license_key_formatting(s, k);
        assert_eq!(result, "2-5G-3J".to_string());
    }


    #[test]
    fn test_solution_case_three() {
        let s = "a-a-a-a-".to_string();
        let k = 1;
        let result = Solution::license_key_formatting(s, k);
        assert_eq!(result, "A-A-A-A".to_string());
    }
}