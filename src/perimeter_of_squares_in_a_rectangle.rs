//! https://www.codewars.com/kata/perimeter-of-squares-in-a-rectangle/train/rust
fn perimeter(n: u64) -> u64 {
    // init (1) + n numbers
    (0..n).fold(State { prev_2: 0, prev_1: 1, sum: 1 }, |acc, _| {
        let next_fib_number = acc.prev_1 + acc.prev_2;
        State {
            prev_2: acc.prev_1,
            prev_1: next_fib_number,
            sum: acc.sum + next_fib_number,
        }
    }).sum * 4

    // 1st attempt
//    4 * match n {
//        0...1 => n,
//        _ => {
//            let mut sum = 1;
//            let mut prev = (0, 1);
//            for i in 2..(n as usize + 2) {
//                let next = prev.0 + prev.1;
//                sum += next;
//                prev = (prev.1, next);
//            }
//            sum
//        }
//    }
}

struct State {
    prev_2: u64,
    prev_1: u64,
    sum: u64,
}

#[cfg(test)]
mod tests {
    use crate::perimeter_of_squares_in_a_rectangle::perimeter;

    fn dotest(n: u64, exp: u64) -> () {
        assert_eq!(perimeter(n), exp)
    }

    #[test]
    fn basics_perimeter() {
        dotest(0, 4);
        dotest(1, 8);
        dotest(5, 80);
        dotest(7, 216);
        dotest(20, 114624);
        dotest(30, 14098308);
    }
}
