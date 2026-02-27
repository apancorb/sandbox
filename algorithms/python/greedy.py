"""
Greedy Pattern

A collection of algorithm problems using greedy strategies.

Greedy = at each step, make the locally optimal choice.
Works when local optimum leads to global optimum.
"""


def jump_to_end(nums: list[int]) -> bool:
    """
    Jump to the End

    Given an array where nums[i] is the max jump distance from index i,
    determine if you can reach the last index starting from index 0.

    Example:
        >>> jump_to_end([3, 2, 0, 2, 5])
        True   # 0→3→4 (jump 3, then jump 2)
        >>> jump_to_end([2, 1, 0, 3])
        False  # stuck at index 2 (can't jump past the 0)

    Work backwards: start destination at last index.
    If position i can reach destination, move destination to i.
    If destination reaches 0, we can make it!

    Example [3, 2, 0, 2, 5], destination starts at index 4:
        i=3: nums[3]=2, 3+2=5 >= 4 ✓ destination=3
        i=2: nums[2]=0, 2+0=2 < 3  ✗
        i=1: nums[1]=2, 1+2=3 >= 3 ✓ destination=1
        i=0: nums[0]=3, 0+3=3 >= 1 ✓ destination=0
        destination == 0 → True!

    Time Complexity: O(n)
    Space Complexity: O(1)
    """
    destination = len(nums) - 1

    for i in range(len(nums) - 2, -1, -1):
        if nums[i] + i >= destination:
            destination = i

    return destination == 0


# -----------------------------------------------------------------------------
# Tests for jump_to_end
# -----------------------------------------------------------------------------

def test_jump_to_end_example1():
    assert jump_to_end([3, 2, 0, 2, 5]) == True


def test_jump_to_end_example2():
    assert jump_to_end([2, 1, 0, 3]) == False


def test_jump_to_end_single():
    assert jump_to_end([0]) == True


def test_jump_to_end_two_elements():
    assert jump_to_end([1, 0]) == True
    assert jump_to_end([0, 1]) == False


def test_jump_to_end_all_ones():
    assert jump_to_end([1, 1, 1, 1]) == True


def test_jump_to_end_large_jump():
    assert jump_to_end([5, 0, 0, 0, 0]) == True


def min_jumps(nums: list[int]) -> int:
    """
    Jump Game II

    Given array where nums[i] is max jump length from position i,
    return minimum number of jumps to reach the last index.
    Guaranteed you can reach the end.

    Example:
        >>> min_jumps([2, 3, 1, 1, 4])
        2
        # Jump 1 step (0→1), then 3 steps (1→4)

    BFS-like greedy: track the "edge" of current jump.
    When we reach the edge, we must jump. Pick the farthest we've seen.

    Example [2, 3, 1, 1, 4]:
        i=0: farthest=max(0, 0+2)=2, i==edge(0) → JUMP! edge=2, jumps=1
        i=1: farthest=max(2, 1+3)=4, not at edge yet
        i=2: farthest=max(4, 2+1)=4, i==edge(2) → JUMP! edge=4, jumps=2
        edge=4 >= last index, done! Answer: 2

    Time Complexity: O(n)
    Space Complexity: O(1)
    """
    if len(nums) <= 1:
        return 0

    jumps = 0
    current_end = 0   # edge of current jump
    farthest = 0      # best we've seen so far

    for i in range(len(nums) - 1):
        farthest = max(farthest, nums[i] + i)

        # Reached the edge, must jump
        if i == current_end:
            jumps += 1
            current_end = farthest

    return jumps


# -----------------------------------------------------------------------------
# Tests for min_jumps
# -----------------------------------------------------------------------------

def test_min_jumps_example1():
    assert min_jumps([2, 3, 1, 1, 4]) == 2


def test_min_jumps_example2():
    assert min_jumps([2, 3, 0, 1, 4]) == 2


def test_min_jumps_single():
    assert min_jumps([0]) == 0


def test_min_jumps_two_elements():
    assert min_jumps([1, 0]) == 1


def test_min_jumps_one_big_jump():
    assert min_jumps([5, 0, 0, 0, 0]) == 1


def test_min_jumps_all_ones():
    assert min_jumps([1, 1, 1, 1]) == 3


def gas_stations(gas: list[int], cost: list[int]) -> int:
    """
    Gas Stations

    Circular route with gas stations. At each station you gain gas[i] fuel
    and spend cost[i] to reach the next. Find the starting index to complete
    the circuit, or -1 if impossible.

    Example:
        >>> gas_stations([2, 5, 1, 3], [3, 2, 1, 4])
        1
        # Start at 1: tank=5-2=3 → +1-1=3 → +3-4=2 → +2-3=1 ✓

    Two key insights:
        1. If total gas < total cost → impossible (return -1)
        2. If tank goes negative at station i, none of 0..i can be the start.
           Why? If we couldn't make it FROM any of those stations TO i,
           starting earlier doesn't help. Reset start to i+1.

    Example gas=[2, 5, 1, 3], cost=[3, 2, 1, 4]:
        total gas=11, total cost=10, 11 >= 10 → solution exists
        i=0: tank=0+(2-3)=-1 < 0 → reset! start=1, tank=0
        i=1: tank=0+(5-2)=3
        i=2: tank=3+(1-1)=3
        i=3: tank=3+(3-4)=2
        No more resets. start=1 ✓

    Time Complexity: O(n)
    Space Complexity: O(1)

    Proof by contradiction that start is correct:
        1. sum(gas) >= sum(cost) → a valid starting point must exist
        2. We confirmed starting anywhere before start results in a deficit
        3. We didn't encounter any deficit from start to the last station
        4. Since a solution exists and all stations before start fail,
           start must be it. (The remaining circuit from station 0 to
           start-1 is guaranteed to work because total gas >= total cost,
           so any deficit before start is offset by surplus after.)
    """
    # If total gas < total cost, impossible
    if sum(gas) < sum(cost):
        return -1

    start = 0
    tank = 0

    for i in range(len(gas)):
        tank += gas[i] - cost[i]
        # Tank went negative → can't start from anywhere 0..i
        if tank < 0:
            tank = 0
            start = i + 1

    return start


# -----------------------------------------------------------------------------
# Tests for gas_stations
# -----------------------------------------------------------------------------

def test_gas_stations_example():
    assert gas_stations([2, 5, 1, 3], [3, 2, 1, 4]) == 1


def test_gas_stations_impossible():
    assert gas_stations([1, 2, 3], [3, 2, 2]) == -1


def test_gas_stations_start_at_zero():
    assert gas_stations([5, 1, 2], [2, 2, 2]) == 0


def test_gas_stations_single():
    assert gas_stations([2], [1]) == 0
    assert gas_stations([1], [2]) == -1


def test_gas_stations_all_equal():
    assert gas_stations([3, 3, 3], [3, 3, 3]) == 0


def candies(ratings: list[int]) -> int:
    """
    Candies

    Children sit in a row with ratings. Distribute minimum candies such that:
        1. Each child gets at least 1
        2. Higher-rated child gets more candies than their neighbor

    Example:
        >>> candies([4, 3, 2, 4, 5, 1])
        12
        # Candies: [3, 2, 1, 2, 3, 1]

    Two-pass approach:
        1. Left to right: if rating goes UP, give one more than left neighbor
        2. Right to left: if rating goes UP (looking right), take max of
           current and right neighbor + 1

    Example [4, 3, 2, 4, 5, 1]:
        Start:       [1, 1, 1, 1, 1, 1]
        Left pass:   [1, 1, 1, 2, 3, 1]  (only 4>2 and 5>4 go up)
        Right pass:  [3, 2, 1, 2, 3, 1]  (4>3, 3>2 going right-to-left)
        Sum: 3+2+1+2+3+1 = 12

    Time Complexity: O(n) - two passes
    Space Complexity: O(n) - candies array
    """
    n = len(ratings)
    candy = [1] * n

    # Left pass: if rating higher than left neighbor, give more
    for i in range(1, n):
        if ratings[i] > ratings[i - 1]:
            candy[i] = candy[i - 1] + 1

    # Right pass: if rating higher than right neighbor, take max
    for i in range(n - 2, -1, -1):
        if ratings[i] > ratings[i + 1]:
            candy[i] = max(candy[i], candy[i + 1] + 1)

    return sum(candy)


# -----------------------------------------------------------------------------
# Tests for candies
# -----------------------------------------------------------------------------

def test_candies_example1():
    assert candies([4, 3, 2, 4, 5, 1]) == 12


def test_candies_example2():
    assert candies([1, 3, 3]) == 4


def test_candies_single():
    assert candies([5]) == 1


def test_candies_increasing():
    assert candies([1, 2, 3, 4]) == 10  # [1, 2, 3, 4]


def test_candies_decreasing():
    assert candies([4, 3, 2, 1]) == 10  # [4, 3, 2, 1]


def test_candies_all_equal():
    assert candies([3, 3, 3, 3]) == 4  # [1, 1, 1, 1]


def test_candies_valley():
    assert candies([5, 2, 1, 2, 5]) == 11  # [3, 2, 1, 2, 3]


def max_profit(prices: list[int]) -> int:
    """
    Best Time to Buy and Sell Stock

    Find max profit from buying on one day and selling on a later day.

    Example:
        >>> max_profit([7, 1, 5, 3, 6, 4])
        5
        # Buy at 1, sell at 6

    Track the minimum price seen so far. At each price, check if
    selling now would beat our best profit.

    Example [7, 1, 5, 3, 6, 4]:
        price=7: min=7, profit=0
        price=1: min=1, profit=0
        price=5: min=1, profit=5-1=4
        price=3: min=1, profit=4
        price=6: min=1, profit=6-1=5 ★
        price=4: min=1, profit=5
        Answer: 5

    Time Complexity: O(n)
    Space Complexity: O(1)
    """
    min_price = float('inf')
    best = 0

    for price in prices:
        if price < min_price:
            min_price = price
        elif price - min_price > best:
            best = price - min_price

    return best


# -----------------------------------------------------------------------------
# Tests for max_profit
# -----------------------------------------------------------------------------

def test_max_profit_example1():
    assert max_profit([7, 1, 5, 3, 6, 4]) == 5


def test_max_profit_example2():
    assert max_profit([7, 6, 4, 3, 1]) == 0


def test_max_profit_single():
    assert max_profit([5]) == 0


def test_max_profit_two_increasing():
    assert max_profit([1, 5]) == 4


def test_max_profit_two_decreasing():
    assert max_profit([5, 1]) == 0


def test_max_profit_best_at_end():
    assert max_profit([2, 4, 1, 7]) == 6  # Buy at 1, sell at 7


def max_profit_ii(prices: list[int]) -> int:
    """
    Best Time to Buy and Sell Stock II

    Max profit with unlimited transactions (but only hold one share at a time).

    Example:
        >>> max_profit_ii([7, 1, 5, 3, 6, 4])
        7
        # Buy at 1, sell at 5 (profit 4). Buy at 3, sell at 6 (profit 3).

    Greedy: capture every upward movement.
    If price goes up tomorrow, buy today and sell tomorrow.

    Example [7, 1, 5, 3, 6, 4]:
        i=1: prices[1]=1 < prices[0]=7 → skip
        i=2: prices[2]=5 > prices[1]=1 → profit += 5-1 = 4
        i=3: prices[3]=3 < prices[2]=5 → skip
        i=4: prices[4]=6 > prices[3]=3 → profit += 6-3 = 3
        i=5: prices[5]=4 < prices[4]=6 → skip
        Total profit: 4+3 = 7

    Time Complexity: O(n)
    Space Complexity: O(1)

    Why this works: sum of small gains = one big gain
        prices [1, 2, 3, 4]:
        (2-1) + (3-2) + (4-3) = 3 = (4-1)
        Capturing every +1 step equals the peak-valley difference.
    """
    profit = 0

    for i in range(1, len(prices)):
        if prices[i] > prices[i - 1]:
            profit += prices[i] - prices[i - 1]

    return profit


# -----------------------------------------------------------------------------
# Tests for max_profit_ii
# -----------------------------------------------------------------------------

def test_max_profit_ii_example1():
    assert max_profit_ii([7, 1, 5, 3, 6, 4]) == 7


def test_max_profit_ii_example2():
    assert max_profit_ii([1, 2, 3, 4, 5]) == 4


def test_max_profit_ii_example3():
    assert max_profit_ii([7, 6, 4, 3, 1]) == 0


def test_max_profit_ii_single():
    assert max_profit_ii([5]) == 0


def test_max_profit_ii_zigzag():
    assert max_profit_ii([1, 3, 2, 4]) == 4  # (3-1) + (4-2) = 4


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
