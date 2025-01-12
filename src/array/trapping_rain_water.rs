#![allow(dead_code)]
struct Solution;

impl Solution {
    fn trap(height: Vec<i32>) -> i32 {
        if height.is_empty() {
            return 0;
        }

        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (height[left], height[right]);
        let mut result = 0;

        while left < right {
            if left_max < right_max {
                left += 1;
                left_max = left_max.max(height[left]);
                result += left_max - height[left];
            } else {
                right -= 1;
                right_max = right_max.max(height[right]);
                result += right_max - height[right];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trap_v1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let result = Solution::trap(height);

        assert_eq!(result, 6);
    }

    #[test]
    fn trap_v2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        let result = Solution::trap(height);

        assert_eq!(result, 9);
    }
}
