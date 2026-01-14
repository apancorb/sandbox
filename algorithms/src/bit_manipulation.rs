/// Hamming Weights of Integers
///
/// The Hamming weight of a number is the number of set bits (1-bits) in its binary
/// representation. Given a positive integer n, return an array where the ith element
/// is the Hamming weight of integer i for all integers from 0 to n.
///
/// # Example
///
/// ```text
/// Input: n = 7
///
/// Output: [0, 1, 1, 2, 1, 2, 2, 3]
///
/// Explanation:
/// 0 = 000 → 0 ones
/// 1 = 001 → 1 one
/// 2 = 010 → 1 one
/// 3 = 011 → 2 ones
/// 4 = 100 → 1 one
/// 5 = 101 → 2 ones
/// 6 = 110 → 2 ones
/// 7 = 111 → 3 ones
/// ```
pub fn hamming_weights(n: usize) -> Vec<u32> {
    fn count_set_bits(mut x: usize) -> u32 {
        let mut count = 0;

        while x > 0 {
            count += x & 1;
            x >>= 1;
        }

        count as u32
    }

    let mut res = Vec::new();
    for i in 0..=n {
        res.push(count_set_bits(i));
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming_weights_example() {
        assert_eq!(hamming_weights(7), vec![0, 1, 1, 2, 1, 2, 2, 3]);
    }

    #[test]
    fn test_hamming_weights_zero() {
        assert_eq!(hamming_weights(0), vec![0]);
    }

    #[test]
    fn test_hamming_weights_one() {
        assert_eq!(hamming_weights(1), vec![0, 1]);
    }

    #[test]
    fn test_hamming_weights_two() {
        assert_eq!(hamming_weights(2), vec![0, 1, 1]);
    }

    #[test]
    fn test_hamming_weights_fifteen() {
        // 15 = 1111 → 4 ones
        assert_eq!(
            hamming_weights(15),
            vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4]
        );
    }
}
