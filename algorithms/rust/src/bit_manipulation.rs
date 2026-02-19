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

/// Lonely Integer
///
/// Given an integer array where each number occurs twice except for one of them,
/// find the unique number.
///
/// # Example
///
/// ```text
/// Input: nums = [1, 3, 3, 2, 1]
///
/// Output: 2
/// ```
///
/// # Constraints
///
/// - nums contains at least one element.
pub fn lonely_integer(nums: &[i32]) -> i32 {
    let mut res = 0;

    // XOR each element of the array so that duplicate values will
    // cancel each other out (x ^ x == e).
    for &num in nums {
        res ^= num;
    }

    // 'res' will store the lonely integer because it would not have
    // been canceled out by any duplicate.
    res
}

/// Swap Odd and Even Bits
///
/// Given an unsigned 32-bit integer n, return an integer where all of n's even bits
/// are swapped with their adjacent odd bits.
///
/// # Example 1
///
/// ```text
/// 1 0 1 0 0 1  (41)
/// ↓ ↑ ↓ ↑ ↓ ↑
/// 0 1 0 1 1 0  (22)
///
/// Input: n = 41
/// Output: 22
/// ```
///
/// # Example 2
///
/// ```text
/// 0 1 0 1 1 1  (23)
/// ↓ ↑ ↓ ↑ ↓ ↑
/// 1 0 1 0 1 1  (43)
///
/// Input: n = 23
/// Output: 43
/// ```
pub fn swap_odd_even_bits(n: u32) -> u32 {
    const EVEN_MASK: u32 = 0x55555555;
    const ODD_MASK: u32 = 0xAAAAAAAA;

    let even_bits = EVEN_MASK & n;
    let odd_bits = ODD_MASK & n;

    // Shift the even bits to the left, the odd bits to the right, and
    // merge these shifted values together.
    (even_bits << 1) | (odd_bits >> 1)
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

    #[test]
    fn test_lonely_integer_example() {
        assert_eq!(lonely_integer(&[1, 3, 3, 2, 1]), 2);
    }

    #[test]
    fn test_lonely_integer_single() {
        assert_eq!(lonely_integer(&[42]), 42);
    }

    #[test]
    fn test_lonely_integer_at_start() {
        assert_eq!(lonely_integer(&[5, 1, 1, 2, 2]), 5);
    }

    #[test]
    fn test_lonely_integer_at_end() {
        assert_eq!(lonely_integer(&[1, 1, 2, 2, 7]), 7);
    }

    #[test]
    fn test_lonely_integer_negative() {
        assert_eq!(lonely_integer(&[-1, 2, 2, -1, -3]), -3);
    }

    #[test]
    fn test_swap_odd_even_bits_example1() {
        assert_eq!(swap_odd_even_bits(41), 22);
    }

    #[test]
    fn test_swap_odd_even_bits_example2() {
        assert_eq!(swap_odd_even_bits(23), 43);
    }

    #[test]
    fn test_swap_odd_even_bits_zero() {
        assert_eq!(swap_odd_even_bits(0), 0);
    }

    #[test]
    fn test_swap_odd_even_bits_all_ones() {
        // 0b1111 stays 0b1111
        assert_eq!(swap_odd_even_bits(0b1111), 0b1111);
    }

    #[test]
    fn test_swap_odd_even_bits_alternating() {
        // 0b10101010 becomes 0b01010101
        assert_eq!(swap_odd_even_bits(0b10101010), 0b01010101);
    }
}
