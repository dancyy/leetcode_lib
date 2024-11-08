use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    // Loop thru the nums vec, find the complement and compare to our hashmap
    for (current_index, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_index) = map.get(&complement) {
            println!("[{}, {}]", complement_index, current_index);
            return vec![complement_index as i32, current_index as i32];
        }
        // Add the current number and its index to the hashmap
        map.insert(num, current_index);
    }
    vec![] // Return an empty vector if no solution found
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn ex2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn ex3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
