struct Solution

impl Solution {
  pub fn is_valid(s: String) -> bool {
     
  }
}

fn is_open(c: char) -> bool {
  match c {
    '(' | '[' | '{' => true,
    _ => false
  }
}

fn is_pair(open: char, close: char) -> bool {
  match (open, close) {
    ('(', ')') | ('[', ']') | ('{', '}') => true,
    _ => false
  }
}


#[cfg(test)]
mod tests {


    #[test]
    fn it_works() {
    }
}
