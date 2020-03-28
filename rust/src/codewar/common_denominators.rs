/// https://www.codewars.com/kata/54d7660d2daf68c619000d95/train/rust
#[allow(dead_code)]
pub fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let divisors: Vec<i64> = l.iter().map(|(_, d)| *d).collect();
    let lcm_divisors = divisors.iter().fold(1, |acc, &d| lcm(acc, d));

    let before_simplified: Vec<(i64, i64)> = l
        .into_iter()
        .map(|(num, denom)| (num * lcm_divisors / denom, lcm_divisors))
        .collect();
    let common_factor = before_simplified
        .iter()
        .map(|(num, denom)| gcd(*num, *denom))
        .fold(0, |acc, g| gcd(acc, g));
    before_simplified
        .into_iter()
        .map(|(num, denom)| (num / common_factor, denom / common_factor))
        .collect()
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    let mut remainder;
    let mut tmp_a = a;
    let mut tmp_b = b;

    while {
        remainder = tmp_a % tmp_b;
        tmp_a = tmp_b;
        tmp_b = remainder;
        remainder != 0
    } {}
    tmp_a
}

/// best solution
//fn gcd(a: i64, b: i64) -> i64 { if b == 0 { a } else { gcd(b, a % b)} }
//fn lcm(a: i64, b: i64) -> i64 { a / gcd(a, b) * b }
//fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
//    let d = l.iter().fold(1, |acc, &(num, den)| lcm(acc, den/gcd(num, den)));
//    l.iter().map(|&(num, den)| (num*d/den, d)).collect()
//}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
        assert_eq!(convert_fracts(l), exp)
    }

    #[test]
    fn basics_convert_fracts() {
        testing(
            vec![(69, 130), (87, 1310), (3, 4)],
            vec![(18078, 34060), (2262, 34060), (25545, 34060)],
        );
        testing(
            vec![(690, 1300), (87, 1310), (30, 40)],
            vec![(18078, 34060), (2262, 34060), (25545, 34060)],
        );
    }
}
