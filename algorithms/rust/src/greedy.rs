/// Jump to the End
///
/// Given an array where nums[i] is the max jump distance from index i,
/// determine if you can reach the last index starting from index 0.
///
/// # Examples
///
/// ```text
/// Input: nums = [3, 2, 0, 2, 5]
/// Output: true   // 0->3->4 (jump 3, then jump 2)
///
/// Input: nums = [2, 1, 0, 3]
/// Output: false  // stuck at index 2 (can't jump past the 0)
/// ```
///
/// Work backwards: start destination at last index.
/// If position i can reach destination, move destination to i.
/// If destination reaches 0, we can make it!
///
/// Example walkthrough for [3, 2, 0, 2, 5], destination starts at index 4:
///
/// ```text
/// i=3: nums[3]=2, 3+2=5 >= 4 ✓ destination=3
/// i=2: nums[2]=0, 2+0=2 < 3  ✗
/// i=1: nums[1]=2, 1+2=3 >= 3 ✓ destination=1
/// i=0: nums[0]=3, 0+3=3 >= 1 ✓ destination=0
/// destination == 0 -> True!
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(1)
pub fn jump_to_end(nums: &[usize]) -> bool {
    let mut destination = nums.len() - 1;

    for i in (0..nums.len() - 1).rev() {
        if nums[i] + i >= destination {
            destination = i;
        }
    }

    destination == 0
}

/// Jump Game II
///
/// Given array where nums[i] is max jump length from position i,
/// return minimum number of jumps to reach the last index.
/// Guaranteed you can reach the end.
///
/// # Examples
///
/// ```text
/// Input: nums = [2, 3, 1, 1, 4]
/// Output: 2
/// // Jump 1 step (0->1), then 3 steps (1->4)
/// ```
///
/// BFS-like greedy: track the "edge" of current jump.
/// When we reach the edge, we must jump. Pick the farthest we've seen.
///
/// Example walkthrough for [2, 3, 1, 1, 4]:
///
/// ```text
/// i=0: farthest=max(0, 0+2)=2, i==edge(0) -> JUMP! edge=2, jumps=1
/// i=1: farthest=max(2, 1+3)=4, not at edge yet
/// i=2: farthest=max(4, 2+1)=4, i==edge(2) -> JUMP! edge=4, jumps=2
/// edge=4 >= last index, done! Answer: 2
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(1)
pub fn min_jumps(nums: &[usize]) -> usize {
    if nums.len() <= 1 {
        return 0;
    }

    let mut jumps = 0;
    let mut current_end = 0; // edge of current jump
    let mut farthest = 0; // best we have seen

    for i in 0..nums.len() - 1 {
        // update the fathest reachable index of this jump
        farthest = farthest.max(nums[i] + i);

        // reached the edge, must jump
        if i == current_end {
            jumps += 1;
            current_end = farthest;
        }
    }

    jumps
}

/// Gas Stations
///
/// Circular route with gas stations. At each station you gain gas[i] fuel
/// and spend cost[i] to reach the next. Find the starting index to complete
/// the circuit, or -1 if impossible.
///
/// # Examples
///
/// ```text
/// Input: gas = [2, 5, 1, 3], cost = [3, 2, 1, 4]
/// Output: 1
/// // Start at 1: tank=5-2=3 -> +1-1=3 -> +3-4=2 -> +2-3=1 ✓
/// ```
///
/// Two key insights:
///     1. If total gas < total cost -> impossible (return -1)
///     2. If tank goes negative at station i, none of 0..i can be the start.
///        Why? If we couldn't make it FROM any of those stations TO i,
///        starting earlier doesn't help. Reset start to i+1.
///
/// Example walkthrough for gas=[2, 5, 1, 3], cost=[3, 2, 1, 4]:
///
/// ```text
/// total gas=11, total cost=10, 11 >= 10 -> solution exists
/// i=0: tank=0+(2-3)=-1 < 0 -> reset! start=1, tank=0
/// i=1: tank=0+(5-2)=3
/// i=2: tank=3+(1-1)=3
/// i=3: tank=3+(3-4)=2
/// No more resets. start=1 ✓
/// ```
///
/// Proof by contradiction that start is correct:
///     1. sum(gas) >= sum(cost) -> a valid starting point must exist
///     2. We confirmed starting anywhere before start results in a deficit
///     3. We didn't encounter any deficit from start to the last station
///     4. Since a solution exists and all stations before start fail,
///        start must be it. (The remaining circuit from station 0 to
///        start-1 is guaranteed to work because total gas >= total cost,
///        so any deficit before start is offset by surplus after.)
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(1)
pub fn gas_stations(gas: &[i32], cost: &[i32]) -> i32 {
    let sum_gas: i32 = gas.iter().sum();
    let sum_cost: i32 = cost.iter().sum();

    // If the total gas is less than the total cost, completing the
    // circuit is impossible.
    if sum_cost > sum_gas {
        return -1;
    }

    let mut start = 0;
    let mut tank = 0;

    for i in 0..gas.len() {
        let net_cost = gas[i] - cost[i];
        tank += net_cost;
        // If our tank has negative gas, we cannot continue through the
        // circuit from the current start point, nor from any station
        // before or including the current station 'i '
        if tank < 0 {
            // Set the next station as the new start point and reset the
            // tank.
            tank = 0;
            start = (i + 1) as i32;
        }
    }

    // Proof by contradiction that `start` is the answer:
    // 1. sum(gas) >= sum(cost) implies a valid starting point must exist.
    // 2. We confirmed that starting anywhere before `start` results in a deficit.
    // 3. We didn't encounter any deficit from `start` to the last station.
    // 4. Since a solution exists and all stations before `start` fail, `start` must be it.
    //    (The remaining circuit from station 0 to `start-1` is guaranteed to work because
    //    the total gas >= total cost, so any deficit before `start` is offset by surplus after.)
    start
}

/// Candies
///
/// Children sit in a row with ratings. Distribute minimum candies such that:
///     1. Each child gets at least 1
///     2. Higher-rated child gets more candies than their neighbor
///
/// # Examples
///
/// ```text
/// Input: ratings = [4, 3, 2, 4, 5, 1]
/// Output: 12
/// // Candies: [3, 2, 1, 2, 3, 1]
/// ```
///
/// Two-pass approach:
///     1. Left to right: if rating goes UP, give one more than left neighbor
///     2. Right to left: if rating goes UP (looking right), take max of
///        current and right neighbor + 1
///
/// Example walkthrough for [4, 3, 2, 4, 5, 1]:
///
/// ```text
/// Start:       [1, 1, 1, 1, 1, 1]
/// Left pass:   [1, 1, 1, 2, 3, 1]  (only 4>2 and 5>4 go up)
/// Right pass:  [3, 2, 1, 2, 3, 1]  (4>3, 3>2 going right-to-left)
/// Sum: 3+2+1+2+3+1 = 12
/// ```
///
/// # Complexity
///
/// - Time: O(n) - two passes
/// - Space: O(n) - candies array
pub fn candies(ratings: &[u32]) -> u32 {
    let mut candies = vec![1; ratings.len()];

    for i in 1..ratings.len() {
        if ratings[i] > ratings[i - 1] {
            candies[i] = 1 + candies[i - 1];
        }
    }

    for i in (0..ratings.len() - 1).rev() {
        if ratings[i] > ratings[i + 1] {
            candies[i] = candies[i].max(candies[i + 1] + 1);
        }
    }

    candies.iter().sum()
}

/// Best Time to Buy and Sell Stock
///
/// Find max profit from buying on one day and selling on a later day.
///
/// # Examples
///
/// ```text
/// Input: prices = [7, 1, 5, 3, 6, 4]
/// Output: 5
/// // Buy at 1, sell at 6
/// ```
///
/// Track the minimum price seen so far. At each price, check if
/// selling now would beat our best profit.
///
/// Example walkthrough for [7, 1, 5, 3, 6, 4]:
///
/// ```text
/// price=7: min=7, profit=0
/// price=1: min=1, profit=0
/// price=5: min=1, profit=5-1=4
/// price=3: min=1, profit=4
/// price=6: min=1, profit=6-1=5 ★
/// price=4: min=1, profit=5
/// Answer: 5
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(1)
pub fn max_profit(prices: &[i32]) -> i32 {
    let mut min_price = i32::MAX;
    let mut max_profit = 0;

    for &price in prices {
        if price < min_price {
            min_price = price;
        } else if price - min_price > max_profit {
            max_profit = price - min_price;
        }
    }

    max_profit
}

/// Best Time to Buy and Sell Stock II
///
/// Max profit with unlimited transactions (but only hold one share at a time).
///
/// # Examples
///
/// ```text
/// Input: prices = [7, 1, 5, 3, 6, 4]
/// Output: 7
/// // Buy at 1, sell at 5 (profit 4). Buy at 3, sell at 6 (profit 3).
/// ```
///
/// Greedy: capture every upward movement.
/// If price goes up tomorrow, buy today and sell tomorrow.
///
/// Why this works: sum of small gains = one big gain
///     prices [1, 2, 3, 4]:
///     (2-1) + (3-2) + (4-3) = 3 = (4-1)
///     Capturing every +1 step equals the peak-valley difference.
///
/// Example walkthrough for [7, 1, 5, 3, 6, 4]:
///
/// ```text
/// i=1: prices[1]=1 < prices[0]=7 -> skip
/// i=2: prices[2]=5 > prices[1]=1 -> profit += 5-1 = 4
/// i=3: prices[3]=3 < prices[2]=5 -> skip
/// i=4: prices[4]=6 > prices[3]=3 -> profit += 6-3 = 3
/// i=5: prices[5]=4 < prices[4]=6 -> skip
/// Total profit: 4+3 = 7
/// ```
///
/// # Complexity
///
/// - Time: O(n)
/// - Space: O(1)
pub fn max_profit_ii(prices: &[i32]) -> i32 {
    let mut profit = 0;

    for i in 1..prices.len() {
        if prices[i] > prices[i - 1] {
            profit += prices[i] - prices[i - 1];
        }
    }

    profit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jump_to_end_example1() {
        assert_eq!(jump_to_end(&[3, 2, 0, 2, 5]), true);
    }

    #[test]
    fn test_jump_to_end_example2() {
        assert_eq!(jump_to_end(&[2, 1, 0, 3]), false);
    }

    #[test]
    fn test_jump_to_end_single() {
        assert_eq!(jump_to_end(&[0]), true);
    }

    #[test]
    fn test_jump_to_end_two_elements() {
        assert_eq!(jump_to_end(&[1, 0]), true);
        assert_eq!(jump_to_end(&[0, 1]), false);
    }

    #[test]
    fn test_jump_to_end_all_ones() {
        assert_eq!(jump_to_end(&[1, 1, 1, 1]), true);
    }

    #[test]
    fn test_jump_to_end_large_jump() {
        assert_eq!(jump_to_end(&[5, 0, 0, 0, 0]), true);
    }

    #[test]
    fn test_gas_stations_example() {
        assert_eq!(gas_stations(&[2, 5, 1, 3], &[3, 2, 1, 4]), 1);
    }

    #[test]
    fn test_gas_stations_impossible() {
        assert_eq!(gas_stations(&[1, 2, 3], &[3, 2, 2]), -1);
    }

    #[test]
    fn test_gas_stations_start_at_zero() {
        assert_eq!(gas_stations(&[5, 1, 2], &[2, 2, 2]), 0);
    }

    #[test]
    fn test_gas_stations_single() {
        assert_eq!(gas_stations(&[2], &[1]), 0);
        assert_eq!(gas_stations(&[1], &[2]), -1);
    }

    #[test]
    fn test_gas_stations_all_equal() {
        assert_eq!(gas_stations(&[3, 3, 3], &[3, 3, 3]), 0);
    }

    #[test]
    fn test_candies_example1() {
        assert_eq!(candies(&[4, 3, 2, 4, 5, 1]), 12);
    }

    #[test]
    fn test_candies_example2() {
        assert_eq!(candies(&[1, 3, 3]), 4);
    }

    #[test]
    fn test_candies_single() {
        assert_eq!(candies(&[5]), 1);
    }

    #[test]
    fn test_candies_increasing() {
        assert_eq!(candies(&[1, 2, 3, 4]), 10); // [1, 2, 3, 4]
    }

    #[test]
    fn test_candies_decreasing() {
        assert_eq!(candies(&[4, 3, 2, 1]), 10); // [4, 3, 2, 1]
    }

    #[test]
    fn test_candies_all_equal() {
        assert_eq!(candies(&[3, 3, 3, 3]), 4); // [1, 1, 1, 1]
    }

    #[test]
    fn test_candies_valley() {
        assert_eq!(candies(&[5, 2, 1, 2, 5]), 11); // [2, 1, 1, 2, 3] -> wait that's wrong
        // Actually: [3, 2, 1, 2, 3] = 11
    }

    #[test]
    fn test_max_profit_example1() {
        assert_eq!(max_profit(&[7, 1, 5, 3, 6, 4]), 5);
    }

    #[test]
    fn test_max_profit_example2() {
        assert_eq!(max_profit(&[7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_max_profit_single() {
        assert_eq!(max_profit(&[5]), 0);
    }

    #[test]
    fn test_max_profit_two_increasing() {
        assert_eq!(max_profit(&[1, 5]), 4);
    }

    #[test]
    fn test_max_profit_two_decreasing() {
        assert_eq!(max_profit(&[5, 1]), 0);
    }

    #[test]
    fn test_max_profit_best_at_end() {
        assert_eq!(max_profit(&[2, 4, 1, 7]), 6); // Buy at 1, sell at 7
    }

    #[test]
    fn test_max_profit_ii_example1() {
        assert_eq!(max_profit_ii(&[7, 1, 5, 3, 6, 4]), 7);
    }

    #[test]
    fn test_max_profit_ii_example2() {
        assert_eq!(max_profit_ii(&[1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn test_max_profit_ii_example3() {
        assert_eq!(max_profit_ii(&[7, 6, 4, 3, 1]), 0);
    }

    #[test]
    fn test_max_profit_ii_single() {
        assert_eq!(max_profit_ii(&[5]), 0);
    }

    #[test]
    fn test_max_profit_ii_zigzag() {
        assert_eq!(max_profit_ii(&[1, 3, 2, 4]), 4); // (3-1) + (4-2) = 2 + 2 = 4
    }

    #[test]
    fn test_min_jumps_example1() {
        assert_eq!(min_jumps(&[2, 3, 1, 1, 4]), 2);
    }

    #[test]
    fn test_min_jumps_example2() {
        assert_eq!(min_jumps(&[2, 3, 0, 1, 4]), 2);
    }

    #[test]
    fn test_min_jumps_single() {
        assert_eq!(min_jumps(&[0]), 0);
    }

    #[test]
    fn test_min_jumps_two_elements() {
        assert_eq!(min_jumps(&[1, 0]), 1);
    }

    #[test]
    fn test_min_jumps_one_big_jump() {
        assert_eq!(min_jumps(&[5, 0, 0, 0, 0]), 1);
    }

    #[test]
    fn test_min_jumps_all_ones() {
        assert_eq!(min_jumps(&[1, 1, 1, 1]), 3);
    }
}
