

fn sum_u32(arr: &[u32]) -> Option<u32> {
    arr.iter().try_fold(0u32, |acc, &x| acc.checked_add(x))
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_test() {
        let arr: &[u32] = &[1,2,3,4];
        assert_eq!(sum_u32(arr), Some(10));

        let arr: &[u32] = &[1,2,3,4, u32::MAX];
        assert_eq!(sum_u32(arr), None);
    }
}