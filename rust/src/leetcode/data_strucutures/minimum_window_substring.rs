use std::{collections::HashMap, ops::Range};

#[allow(dead_code)]
struct Solution {}

#[allow(dead_code)]
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        Self::solution_min_window(s, t)
    }

    pub fn solution_min_window(s: String, t: String) -> String {
        let mut t_count_by_char = count_of_chars(t);
        let filtered_s: Vec<(usize, char)> = s
            .char_indices()
            .filter(|(_, c)| t_count_by_char.contains_key(c))
            .collect();

        let mut tuple_start = 0;
        let mut best_window: Option<Range<usize>> = None;

        let mut char_fulfilled = 0;
        for (i, ch) in &filtered_s {
            let count = t_count_by_char.get_mut(ch).unwrap();
            *count -= 1;
            if *count == 0 {
                char_fulfilled += 1;
                while char_fulfilled == t_count_by_char.len() {
                    let end = *i;
                    let (start, char_at_start) = filtered_s[tuple_start];
                    let is_shorter_window = match &best_window {
                        Some(w) => end - start + 1 < w.len(),
                        None => true,
                    };
                    if is_shorter_window {
                        best_window = Some(start..end + 1);
                    }
                    // can move start
                    tuple_start += 1;
                    // revert
                    let count = t_count_by_char.get_mut(&char_at_start).unwrap();
                    *count += 1;
                    if *count > 0 {
                        char_fulfilled -= 1;
                    }
                }
            }
        }
        best_window.map_or("", |w| &s[w]).to_string()
    }

    fn my_min_window(s: String, t: String) -> String {
        let s_chars: Vec<char> = s.chars().collect();
        let mut remaining_count_to_be_fulfilled_by_char = count_of_chars(t);
        let mut window: Range<usize> = 0..0;
        let mut min_window_fulfilled: Option<Range<usize>> = None;
        let mut ever_fulfilled_all = false;
        for (i, ch) in s.char_indices() {
            window.end = i + 1; // +1 for exclusive
            let mut is_duplicatedly_counted = false;
            let mut is_just_fulfilled_this_count = false;
            // checking manifest, checked, checked!
            if let Some(count) = remaining_count_to_be_fulfilled_by_char.get_mut(&ch) {
                *count -= 1;
                is_duplicatedly_counted = *count < 0;
                is_just_fulfilled_this_count = *count == 0;
            }
            let mut starting_char = s_chars[window.start];
            // found starting char is duplicatedly counted => can shrink window
            // or it is just an irrelevant char
            let mut should_move_start = ch == starting_char && is_duplicatedly_counted
                || !remaining_count_to_be_fulfilled_by_char.contains_key(&starting_char);
            let is_shifted = should_move_start;
            while should_move_start {
                window.start += 1;
                if let Some(count_of_starting_char) =
                    remaining_count_to_be_fulfilled_by_char.get_mut(&starting_char)
                {
                    *count_of_starting_char += 1; // revert the over-counted
                }
                // How about next starting char we've just moved?
                should_move_start = match window.start {
                    start if start >= i => false,
                    _ => {
                        starting_char = s_chars[window.start];
                        if let Some(count_of_starting_char) =
                            remaining_count_to_be_fulfilled_by_char.get_mut(&starting_char)
                        {
                            *count_of_starting_char < 0 // this new starting char was duplicatedly counted also
                        } else {
                            true
                        }
                    }
                };
            }
            if is_just_fulfilled_this_count || is_shifted {
                if !ever_fulfilled_all {
                    ever_fulfilled_all =
                        is_all_char_fulfilled(&remaining_count_to_be_fulfilled_by_char);
                }
                if ever_fulfilled_all {
                    let found_shorter_window = match &min_window_fulfilled {
                        Some(w) => window.len() < w.len(),
                        None => true,
                    };
                    if found_shorter_window {
                        min_window_fulfilled = Some(window.clone());
                    }
                }
            }
        }
        min_window_fulfilled.map_or("", |w| &s[w]).to_string()
    }
}

fn is_all_char_fulfilled(t_count_by_char: &HashMap<char, i32>) -> bool {
    t_count_by_char.values().all(|&c| c <= 0)
}

fn count_of_chars(t: String) -> HashMap<char, i32> {
    t.chars().fold(HashMap::new(), |mut count_by_char, ch| {
        count_by_char
            .entry(ch)
            .and_modify(|count| *count += 1)
            .or_insert(1);
        count_by_char
    })
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_() {
        for (expected, (s, t)) in vec![
            ("BANC", ("ADOBECODEBANC", "ABC")),
            ("ABC1234BBA", ("OABC1234BBAP", "ABCA")),
            ("", ("a", "b")),
            ("usviimasafkxqhrwilaczecdvkeathipbfzjtbd", ("chusviimasafkxqhrwilaczecdvkeathipbfzjtbdvgpszwlxezxgydlbaxgbsplhssjdlkghrwbssnpzonhmssptsxukmfugxdjknqrgotyiiohwdrkvlzqdxmolti"
            ,"sslddavu")),
        ] {
            assert_eq!(
                expected.to_string(),
                Solution::min_window(s.to_string(), t.to_string())
            );
        }
    }
}
