/*
 * Problem: 1700. Number of Students Unable to Eat Lunch
 * Link: https://leetcode.com/problems/number-of-students-unable-to-eat-lunch/
 * */

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut count = vec![0; 2];

        for i in 0..students.len() {
            count[students[i] as usize] += 1;
        }

        for i in 0..sandwiches.len() {
            if count[sandwiches[i] as usize] == 0 {
                return (sandwiches.len() - i) as i32;
            }

            count[sandwiches[i] as usize] -= 1;
        }

        return 0;
    }
}
