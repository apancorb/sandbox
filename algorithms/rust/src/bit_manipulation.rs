/// Hamming Weights of Integers
///
/// Return array where element i = number of 1-bits in binary of i,
/// for all integers 0 to n.
///
/// # Examples
///
/// ```text
/// Input: hamming_weights(7)
/// Output: [0, 1, 1, 2, 1, 2, 2, 3]
///
/// 0=000→0, 1=001→1, 2=010→1, 3=011→2,
/// 4=100→1, 5=101→2, 6=110→2, 7=111→3
/// ```
///
/// For each number, count set bits by checking last bit (& 1)
/// and shifting right (>> 1) until zero.
///
/// Example count_bits(5):
///
/// ```text
/// 5 = 101
/// 101 & 1 = 1, count=1, shift → 10
/// 10 & 1 = 0, count=1, shift → 1
/// 1 & 1 = 1, count=2, shift → 0
/// Done! 2 bits set
/// ```
///
/// # Complexity
///
/// - Time: O(n * bits) where bits ≤ 32
/// - Space: O(n)
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
/// Every number appears twice except one. Find it.
///
/// # Examples
///
/// ```text
/// Input: nums = [1, 3, 3, 2, 1]
/// Output: 2
/// ```
///
/// XOR trick: a ^ a = 0 and a ^ 0 = a.
/// XOR all numbers together → duplicates cancel out, unique survives.
///
/// Example [1, 3, 3, 2, 1]:
///
/// ```text
/// 0 ^ 1 = 1
/// 1 ^ 3 = 2  (binary: 01 ^ 11 = 10)
/// 2 ^ 3 = 1  (binary: 10 ^ 11 = 01)  ← 3s cancelled!
/// 1 ^ 2 = 3  (binary: 01 ^ 10 = 11)
/// 3 ^ 1 = 2  (binary: 11 ^ 01 = 10)  ← 1s cancelled!
/// Answer: 2
/// ```
///
/// # Complexity
///
/// - Time: O(n) — single pass XOR
/// - Space: O(1) — constant extra storage
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
/// Swap every pair of adjacent bits in a 32-bit integer.
///
/// # Examples
///
/// ```text
/// 41 = 1 0 1 0 0 1
///      ↓ ↑ ↓ ↑ ↓ ↑
/// 22 = 0 1 0 1 1 0
///
/// Input: n = 41
/// Output: 22
/// ```
///
/// Use bitmasks to isolate even and odd bits separately, then shift
/// them into each other's positions and merge with OR. This swaps
/// every adjacent pair in one shot without any loops.
///
/// Masks (32-bit):
///
/// ```text
/// EVEN_MASK = 0x55555555 = 0101 0101 ... (selects even bits)
/// ODD_MASK  = 0xAAAAAAAA = 1010 1010 ... (selects odd bits)
/// ```
///
/// Steps:
///
/// ```text
/// 1. Extract even bits: n & EVEN_MASK
/// 2. Extract odd bits:  n & ODD_MASK
/// 3. Shift even bits left by 1 (move to odd positions)
/// 4. Shift odd bits right by 1 (move to even positions)
/// 5. Merge with OR
/// ```
///
/// Example walkthrough for n=41 (101001):
///
/// ```text
/// even bits: 101001 & 010101 = 000001 → shift left  → 000010
/// odd bits:  101001 & 101010 = 101000 → shift right → 010100
/// merge: 000010 | 010100 = 010110 = 22 ✓
/// ```
///
/// # Complexity
///
/// - Time: O(1) — fixed bitwise operations
/// - Space: O(1) — constant extra storage
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
