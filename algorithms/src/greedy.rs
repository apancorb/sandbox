/// Jump to the End
///
/// You are given an integer array in which you're originally positioned at index 0. Each number
/// in the array represents the maximum jump distance from the current index. Determine if
/// it's possible to reach the end of the array.
///
/// # Example 1
///
/// ```text
/// Input: nums = [3, 2, 0, 2, 5]
///
/// Output: true
/// ```
///
/// # Example 2
///
/// ```text
/// Input: nums = [2, 1, 0, 3]
///
/// Output: false
/// ```
///
/// # Constraints
///
/// - There is at least one element in nums.
/// - All integers in nums are non-negative integers.
pub fn jump_to_end(nums: &[usize]) -> bool {
    let mut destination = nums.len() - 1;

    for i in (0..nums.len() - 1).rev() {
        if nums[i] + i >= destination {
            destination = i;
        }
    }

    destination == 0
}

/// Gas Stations
///
/// There's a circular route which contains gas stations. At each station, you can fill your car
/// with a certain amount of gas, and moving from that station to the next one consumes some
/// fuel.
///
/// Find the index of the gas station you would need to start at, in order to complete the circuit
/// without running out of gas. Assume your car starts with an empty tank. If it's not possible
/// to complete the circuit, return -1. If it's possible, assume only one solution exists.
///
/// # Example
///
/// ```text
/// Input: gas = [2, 5, 1, 3], cost = [3, 2, 1, 4]
///
/// Output: 1
///
/// Explanation:
/// Start at station 1: gain 5 gas (tank = 5), costs 2 gas to go to station 2 (tank = 3).
/// At station 2: gain 1 gas (tank = 4), costs 1 gas to go to station 3 (tank = 3).
/// At station 3: gain 3 gas (tank = 6), costs 4 gas to go to station 0 (tank = 2).
/// At station 0: gain 2 gas (tank = 4), costs 3 gas to go to station 1 (tank = 1).
/// We started and finished the circuit at station 1 without running out of gas.
/// ```
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
}
