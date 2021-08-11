pub struct Solution {}

use std::collections::BTreeMap;
use std::ops::Bound::*;

impl Solution {
    pub fn odd_even_jumps(arr: Vec<i32>) -> i32 {
        let n = arr.len();

        let mut even = vec![false; n];
        let mut odd = vec![false; n];

        even[n - 1] = true;
        odd[n - 1] = true;

        let mut map = BTreeMap::<i32, usize>::new();
        map.insert(arr[n-1], n-1);
        let mut result = 1;

        for i in (0..n - 1).rev() {
            let lowest = map.range((Included(arr[i]), Unbounded)).next();
            if lowest.is_some() {
                let (_, j) = lowest.unwrap();
                odd[i] = even[*j];
            } else {
                odd[i] = false;
            }

            let highest = map.range((Unbounded, Included(arr[i]))).rev().next();
            if highest.is_some() {
                let (_, j) = highest.unwrap();
                even[i] = odd[*j];
            } else {
                even[i] = false;
            }

            map.insert(arr[i], i);

            if odd[i] {
                result += 1;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_case_one() {
        let input = vec![10, 13, 12, 14, 15];
        let result = Solution::odd_even_jumps(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solution_case_two() {
        let input = vec![2, 3, 1, 1, 4];
        let result = Solution::odd_even_jumps(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_solution_case_three() {
        let input = vec![1, 1, 4];
        let result = Solution::odd_even_jumps(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solution_case_four() {
        let input = vec![81, 54, 96, 60, 58];
        let result = Solution::odd_even_jumps(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_solution_case_five() {
        let input = vec![27, 81, 84, 89, 58, 94, 57, 45, 66, 99];
        let result = Solution::odd_even_jumps(input);
        assert_eq!(result, 6);
    }
}
