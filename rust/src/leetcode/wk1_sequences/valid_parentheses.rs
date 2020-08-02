// https://leetcode.com/problems/valid-parentheses/

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut opens: Vec<char> = Vec::new();
        for c in s.chars() {
            if is_open(c) {
                opens.push(c);
            } else if opens.last().map_or(false, |&o| is_pair(o, c)) {
                opens.pop();
            } else {
                return false;
            }
        }
        opens.is_empty()
    }
}

fn is_open(c: char) -> bool {
    match c {
        '(' | '[' | '{' => true,
        _ => false,
    }
}

fn is_pair(open: char, close: char) -> bool {
    match (open, close) {
        ('(', ')') | ('[', ']') | ('{', '}') => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        for (ans, s) in vec![
            (true, "[]"),
            (false, "]["),
            (false, "["),
            (true, "()[]{}"),
            (false, "{]"),
            (false, "([)]"),
            (true, "{[]}"),
        ] {
            assert_eq!(ans, Solution::is_valid(s.to_string()));
        }
    }
}
