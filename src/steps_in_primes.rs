 fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
     let mut prime_gen = PrimeGenerator::new(m, n);
     let mut read_primes: Vec<u64> = vec![prime_gen.next().unwrap()];
     let mut result: Option<(u64, u64)> = None;
     'outer: for p in prime_gen {
         for read_p in &read_primes {
             println!("debug={},{}", read_p, p);
             let gap = p - read_p;
             if gap % (g as u64) == 0 {
                 result = Some((*read_p, p));
                 break 'outer;
             }
         }
         read_primes.push(p);
     }
     // prime_gen
     //     .iter()
     //     .enumerate()
     //     .flat_map(|(i, p)| {
     //         let rhs = prime_gen.iter().skip(i + 1).copied();
     //         let lhs = iter::repeat(*p).take(rhs.len());
     //         lhs.zip(rhs)
     //     })
     //     .find(|(a, b)| (b - a) % (g as u64) == 0)
     result
 }


struct PrimeGenerator {
    end: u64,
    n: u64,
}

impl PrimeGenerator {
    fn new(start: u64, end: u64) -> Self {
        Self {
            end,
            n: start,
        }
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

fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    assert_eq!(step(g, m, n), exp)
}

#[test]
fn basics_step() {
    testing(2,100,110, Some((101, 103)));
    testing(4,100,110, Some((103, 107)));
    testing(8,30000,100000, Some((30089, 30097)));
    testing(11,30000,100000, None);
    testing(2,10000000,11000000, Some((10000139, 10000141)));
}