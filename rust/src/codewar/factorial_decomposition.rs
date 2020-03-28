//! https://www.codewars.com/kata/5a045fee46d843effa000070/train/rust
#[allow(dead_code)]
fn decomp(n: i32) -> String {
    if n == 1 {
        "1".to_string()
    } else {
        let r = PrimeGenerator::new(2, n as u64)
            .map(|prime| {
                let max_k = (n as f32).log10() / (prime as f32).log10();
                let max_k = max_k as u32;
                let prime_power = (1..max_k + 1).fold(0, |acc, k| {
                    let prime = prime as i32;
                    let dp = n / prime.pow(k);
                    acc + dp
                });
                (prime, prime_power)
            })
            .filter(|(_, pp)| *pp > 0)
            .fold(String::new(), |mut init, (p, pp)| {
                let s = match pp {
                    1 => format!("{} * ", p),
                    _ => format!("{}^{} * ", p, pp),
                };
                init.push_str(&s);
                init
            });
        r[0..r.len() - 3].to_string()
    }
}

struct PrimeGenerator {
    end: u64,
    n: u64,
}

impl PrimeGenerator {
    fn new(start: u64, end: u64) -> Self {
        Self { end, n: start }
    }

    fn is_prime(&self, n: u64) -> bool {
        match n {
            2 | 3 => true,
            _ if n % 2 == 0 || n % 3 == 0 => false,
            _ => {
                let mut result: bool = true;
                let (mut i, mut w) = (5, 2);
                while i * i <= n {
                    if n % i == 0 {
                        result = false;
                        break;
                    } else {
                        i += w;
                        w = 6 - w;
                    }
                }
                result
            }
        }
    }
}

impl Iterator for PrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next: Option<Self::Item> = None;
        while self.n <= self.end {
            if self.is_prime(self.n) {
                next = Some(self.n);
                self.n += 1;
                break;
            } else {
                self.n += 1;
            }
        }
        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i32, exp: &str) -> () {
        println!("n:{:?}", n);
        let ans = decomp(n);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp.to_string());
        println!("{}", ans == exp.to_string());
        assert_eq!(ans, exp.to_string());
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(17, "2^15 * 3^6 * 5^3 * 7^2 * 11 * 13 * 17");
        dotest(5, "2^3 * 3 * 5");
        dotest(22, "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19");
        dotest(14, "2^11 * 3^5 * 5^2 * 7^2 * 11 * 13");
        dotest(25, "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23");
    }
}
