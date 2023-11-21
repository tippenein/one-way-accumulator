// One-way cryptographic accumulators
// https://link.springer.com/content/pdf/10.1007/3-540-48285-7_24.pdf

#![cfg(test)]
mod tests {
    use crate::PrimeAccumulator;

    use super::Accumulator;
    #[test]
    pub fn test_primes() {
        let mut accumulator: PrimeAccumulator = Accumulator::new(3);
        assert_eq!(accumulator.add(5), 15);
        assert_eq!(accumulator.add(7), 105);
        assert_eq!(accumulator.del(5), 21);
        assert!(accumulator.prove(7));
        assert!(!accumulator.prove(111));
    }
}

trait Accumulator<T> {
    fn new(base: T) -> Self;
    fn add(&mut self, addition: T) -> T;
    fn del(&mut self, deletion: T) -> T;
    fn prove(&self, elem: T) -> bool;
}

struct PrimeAccumulator {
    set: i32,
}

impl Accumulator<i32> for PrimeAccumulator {
    fn new(base: i32) -> Self {
        Self { set: base }
    }
    fn add(&mut self, addition: i32) -> i32 {
        let result = self.set * addition;
        self.set = result;
        return result;
    }

    fn del(&mut self, deletion: i32) -> i32 {
        let result = self.set / deletion;
        self.set = result;
        return result;
    }
    fn prove(&self, elem: i32) -> bool {
        self.set % elem == 0
    }
}

pub fn main() {
    println!("Hello, world!");
}
