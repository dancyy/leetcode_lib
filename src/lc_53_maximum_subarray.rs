pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut max_sub = nums[0];
    let mut current_sub = 0;

    for n in nums {
        if current_sub < 0 {
            current_sub = 0
        }
        current_sub += n;
        max_sub = max_sub.max(current_sub);
    }
    println!("{max_sub}");
    max_sub
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }
}
