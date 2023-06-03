pub fn find_signal_strengths(path: &str, targets: Vec<i32>) -> usize {
    1+2
}

pub fn s2(path: &str) -> usize {
    2+3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_larger_sample() {
        let result = find_signal_strengths("sample_day_ten.txt", vec![20, 60, 100, 140, 180, 220]);
        assert_eq!(result, 13140);
    }
}
