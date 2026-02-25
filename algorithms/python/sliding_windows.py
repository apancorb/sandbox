"""
Sliding Window Pattern

A collection of algorithm problems using sliding windows.

Two types:
- Fixed size window: maintain window of size k, slide one step at a time
- Variable size window: expand/shrink based on condition (e.g., unique chars)
"""


def count_anagrams(s: str, t: str) -> int:
    """
    Substring Anagrams

    Count the number of substrings in s that are anagrams of t.

    Example:
        >>> count_anagrams("caabab", "aba")
        2
        # Anagrams at index 1 ("aab") and index 2 ("aba")

    Time Complexity: O(n) - single pass with fixed window
    Space Complexity: O(1) - fixed size arrays (26 letters)

    Fixed window of size len(t). Compare character frequencies.

    Example walkthrough for s="caabab", t="aba":
        expected freq for "aba": {a:2, b:1}

        i=0: window="c"     → not full yet (need size 3)
        i=1: window="ca"    → not full yet
        i=2: window="caa"   → full! freq={c:1,a:2} ≠ expected, remove 'c'
        i=3: window="aab"   → freq={a:2,b:1} = expected ✓ count=1, remove 'a'
        i=4: window="aba"   → freq={a:2,b:1} = expected ✓ count=2, remove 'a'
        i=5: window="bab"   → freq={a:1,b:2} ≠ expected

        Answer: 2
    """
    if len(t) > len(s) or len(t) == 0:
        return 0

    # Count character frequencies in t (what we're looking for)
    expected = [0] * 26
    for c in t:
        expected[ord(c) - ord('a')] += 1

    # Current window's character frequencies
    window = [0] * 26
    count = 0

    for i, c in enumerate(s):
        # 1. EXPAND: add right character to window
        window[ord(c) - ord('a')] += 1

        # 2. CHECK: once window reaches target size
        if i >= len(t) - 1:
            # Is this window an anagram? (same char frequencies)
            if window == expected:
                count += 1

            # 3. SHRINK: remove leftmost character to slide window
            left_char = s[i - len(t) + 1]
            window[ord(left_char) - ord('a')] -= 1

    return count


# -----------------------------------------------------------------------------
# Tests for count_anagrams
# -----------------------------------------------------------------------------

def test_count_anagrams_example():
    assert count_anagrams("caabab", "aba") == 2


def test_count_anagrams_no_match():
    assert count_anagrams("abcdef", "xyz") == 0


def test_count_anagrams_all_same_char():
    assert count_anagrams("aaaa", "aa") == 3


def test_count_anagrams_exact_match():
    assert count_anagrams("abc", "abc") == 1


def test_count_anagrams_single_char():
    assert count_anagrams("ababa", "a") == 3


def test_count_anagrams_t_longer_than_s():
    assert count_anagrams("ab", "abc") == 0


def test_count_anagrams_empty_s():
    assert count_anagrams("", "abc") == 0


def test_count_anagrams_empty_t():
    assert count_anagrams("abc", "") == 0


def test_count_anagrams_multiple_matches():
    # "ab", "ba", "ab" are all anagrams of "ab"
    assert count_anagrams("abab", "ab") == 3


def test_count_anagrams_overlapping():
    assert count_anagrams("cbaebabacd", "abc") == 2


def longest_unique_substring(s: str) -> int:
    """
    Longest Substring With Unique Characters

    Find the length of the longest substring with all unique characters.

    Example:
        >>> longest_unique_substring("abcba")
        3
        # "abc" or "cba" are the longest with unique chars

    Time Complexity: O(n) - each char visited at most twice
    Space Complexity: O(min(n, 26)) - hashmap of char positions

    Variable window: expand right, shrink left when duplicate found.
    Track last seen position of each character.

    Example walkthrough for s="abcba":
        i=0 'a': last_seen={}, no dup → window [0,0]="a", len=1, last_seen={a:0}
        i=1 'b': no dup → window [0,1]="ab", len=2, last_seen={a:0,b:1}
        i=2 'c': no dup → window [0,2]="abc", len=3 ★, last_seen={a:0,b:1,c:2}
        i=3 'b': dup! b was at 1, jump left to 2 → window [2,3]="cb", len=2
        i=4 'a': a was at 0, but 0 < left(2) so NOT in window, no jump
                 → window [2,4]="cba", len=3

        Answer: 3
    """
    last_seen = {}  # char -> last index where we saw it
    max_len = 0
    left = 0

    for right, char in enumerate(s):
        # If char is duplicate AND it's inside our current window
        if char in last_seen and last_seen[char] >= left:
            # Jump left pointer past the previous occurrence
            # (no need to shrink one-by-one, we can jump directly)
            left = last_seen[char] + 1

        # Current window is [left, right], all unique
        max_len = max(max_len, right - left + 1)

        # Record/update where we saw this char
        last_seen[char] = right

    return max_len


# -----------------------------------------------------------------------------
# Tests for longest_unique_substring
# -----------------------------------------------------------------------------

def test_longest_unique_substring_example():
    assert longest_unique_substring("abcba") == 3


def test_longest_unique_substring_all_unique():
    assert longest_unique_substring("abcdef") == 6


def test_longest_unique_substring_all_same():
    assert longest_unique_substring("aaaa") == 1


def test_longest_unique_substring_empty():
    assert longest_unique_substring("") == 0


def test_longest_unique_substring_single_char():
    assert longest_unique_substring("a") == 1


def test_longest_unique_substring_two_chars():
    assert longest_unique_substring("ab") == 2


def test_longest_unique_substring_repeating_pattern():
    assert longest_unique_substring("abcabcbb") == 3


def test_longest_unique_substring_end_longest():
    assert longest_unique_substring("aabcdef") == 6


def test_longest_unique_substring_middle_longest():
    assert longest_unique_substring("aaabcdefa") == 6


def longest_uniform_substring(s: str, k: int) -> int:
    """
    Longest Uniform Substring After Replacements

    Find the longest substring where all characters are the same,
    if you can replace up to k characters.

    Example:
        >>> longest_uniform_substring("aabcdcca", 2)
        5
        # Replace 'b' and 'd' with 'c' to get "ccccc"

    Time Complexity: O(n) - single pass
    Space Complexity: O(26) - frequency map

    Key insight: window is valid if (window_size - most_frequent_char) <= k
    That's the number of chars we need to replace.

    Example walkthrough for s="aabcdcca", k=2:
        Keep the most frequent char, replace the rest.
        Valid if: window_size - max_freq <= k

        i=0 'a': freq={a:1}, max_freq=1, window="a", replace=1-1=0 ≤2 ✓
        i=1 'a': freq={a:2}, max_freq=2, window="aa", replace=2-2=0 ≤2 ✓
        i=2 'b': freq={a:2,b:1}, max_freq=2, window="aab", replace=3-2=1 ≤2 ✓
        i=3 'c': freq={a:2,b:1,c:1}, max_freq=2, replace=4-2=2 ≤2 ✓
        i=4 'd': freq={a:2,b:1,c:1,d:1}, max_freq=2, replace=5-2=3 >2 ✗
                 shrink! remove 'a', left=1, replace=4-1=3 >2 ✗
                 shrink! remove 'a', left=2, replace=3-1=2 ≤2 ✓
        i=5 'c': freq={b:1,c:2,d:1}, max_freq=2, replace=4-2=2 ≤2 ✓
        i=6 'c': freq={b:1,c:3,d:1}, max_freq=3, replace=5-3=2 ≤2 ✓ len=5 ★
        i=7 'a': freq={a:1,b:1,c:3,d:1}, max_freq=3, replace=6-3=3 >2 ✗
                 shrink! ...

        Answer: 5 (make "ccccc" by replacing b,d with c)
    """
    if not s:
        return 0

    freq = {}       # char -> count in current window
    max_freq = 0    # highest frequency of any single char in window
    left = 0
    max_len = 0

    for right, char in enumerate(s):
        # 1. EXPAND: add right char to window
        freq[char] = freq.get(char, 0) + 1
        max_freq = max(max_freq, freq[char])

        # 2. SHRINK if invalid: need more than k replacements
        # replacements = window_size - max_freq (replace everything except most common)
        while (right - left + 1) - max_freq > k:
            freq[s[left]] -= 1
            left += 1

        # 3. UPDATE: window [left, right] is valid, track max
        max_len = max(max_len, right - left + 1)

    return max_len


# -----------------------------------------------------------------------------
# Tests for longest_uniform_substring
# -----------------------------------------------------------------------------

def test_longest_uniform_substring_example():
    assert longest_uniform_substring("aabcdcca", 2) == 5


def test_longest_uniform_substring_no_replacements():
    assert longest_uniform_substring("aaabbb", 0) == 3


def test_longest_uniform_substring_all_same():
    assert longest_uniform_substring("aaaa", 2) == 4


def test_longest_uniform_substring_all_different():
    assert longest_uniform_substring("abcd", 2) == 3


def test_longest_uniform_substring_k_equals_len():
    assert longest_uniform_substring("abcd", 4) == 4


def test_longest_uniform_substring_empty():
    assert longest_uniform_substring("", 2) == 0


def test_longest_uniform_substring_single_char():
    assert longest_uniform_substring("a", 2) == 1


def test_longest_uniform_substring_two_chars():
    assert longest_uniform_substring("ab", 1) == 2


def test_longest_uniform_substring_alternating():
    assert longest_uniform_substring("ababab", 2) == 5


if __name__ == "__main__":
    import pytest
    pytest.main([__file__, "-v"])
