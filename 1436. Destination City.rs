/*
 *  Link: https://leetcode.com/problems/destination-city/
 *  Problem: 1436. Destination City
 * */

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        use std::collections::HashMap;
        let mut m: HashMap<String, bool> = HashMap::new();

        for p in paths {
            m.insert(p[0].to_string(), true);
            if m.contains_key(&p[1].to_string()) == false {
                m.insert(p[1].to_string(), false);
            }
        }

        let mut ans: String = String::new();

        for (city, out) in m {
            if out == false {
                ans = city.to_string();
                break;
            }
        }

        return ans;
    }
}

