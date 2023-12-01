/*
 * Link: https://leetcode.com/problems/valid-parentheses/
 * Problem: 20. Valid Parentheses
 * */

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut st: Vec<char> = Vec::new();;

        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                st.push(c);
                continue;
            }

            if c == ')'{
                if st.len() <= 0 || st[st.len() - 1] != '('{
                    return false;
                }

                st.pop();
            }

            if c == ']'{
                if st.len() <= 0 || st[st.len() - 1] != '['{
                    return false;
                }

                st.pop();
            }

            if c == '}'{
                if st.len() <= 0 || st[st.len() - 1] != '{' {
                    return false;
                }


                st.pop();
            }
        }

        return st.len() == 0;
    }
}
