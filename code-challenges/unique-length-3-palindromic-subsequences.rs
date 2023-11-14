/*Given a string s, return the number of unique palindromes of length three that are a subsequence of s.
Note that even if there are multiple ways to obtain the same subsequence, it is still only counted once.
A palindrome is a string that reads the same forwards and backwards.
A subsequence of a string is a new string generated from the original string with some characters (can be none) deleted without changing the relative order of the remaining characters.
For example, "ace" is a subsequence of "abcde".
 
Example 1:

Input: s = "aabca"
Output: 3
Explanation: The 3 palindromic subsequences of length 3 are:
- "aba" (subsequence of "aabca")
- "aaa" (subsequence of "aabca")
- "aca" (subsequence of "aabca")

Example 2:

Input: s = "adc"
Output: 0
Explanation: There are no palindromic subsequences of length 3 in "adc".
Example 3:

Input: s = "bbcbaba"
Output: 4
Explanation: The 4 palindromic subsequences of length 3 are:
- "bbb" (subsequence of "bbcbaba")
- "bcb" (subsequence of "bbcbaba")
- "bab" (subsequence of "bbcbaba")
- "aba" (subsequence of "bbcbaba")
 
Constraints:

3 <= s.length <= 105
s consists of only lowercase English letters.
*/

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        // Create a HashMap to store tested 3-length strings and their palindrome status
        let mut tested_strings = HashMap::new();
        let mut palindromes_count = 0;

        // Example of using the is_palindrome function and updating the HashMap
        let test_string = &s[0..3]; // Adjust this based on your specific problem
        if !tested_strings.contains_key(test_string) {
            let is_palindrome = Self::is_palindrome(test_string);
            tested_strings.insert(test_string.to_string(), is_palindrome);

            if is_palindrome {
                palindromes_count += 1
            }
        }

        palindromes_count 
    }

    // Helper function to check if a 3-length string is a palindrome
    fn is_palindrome(s: &str) -> bool {
        // Check if the string is equal to its reverse
        s == s.chars().rev().collect::<String>()
    }
}