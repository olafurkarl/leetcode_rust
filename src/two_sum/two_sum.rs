
pub struct Solution {}

impl Solution {
    pub fn two_sum(arr: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, val1) in arr.iter().enumerate() {
            for j in i+1..arr.len() {
                if val1 + arr[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        
        panic!("No solution found!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution_case_one() {
        let input = vec![2,7,11,15];
        let target = 9;
        let result = Solution::two_sum(input, target);
        assert_eq!(result, [0, 1]);
    }

    #[test]
    fn test_solution_case_two() {
        let input = vec![3,2,4];
        let target = 6;
        let result = Solution::two_sum(input, target);
        assert_eq!(result, [1, 2]);
    }

    #[test]
    fn test_solution_case_three() {
        let input = vec![3,3];
        let target = 6;
        let result = Solution::two_sum(input, target);
        assert_eq!(result, [0,1]);
    }
}
