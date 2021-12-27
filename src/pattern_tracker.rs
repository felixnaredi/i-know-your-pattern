use std::collections::HashMap;

use bitvec::prelude::*;
use rand::Rng;

type Counter = [usize; 2];

const NGRAM_SIZE: usize = 5;

/// A `PatternTracker` is able to predict the next input from a sequence of previous input.
#[derive(Debug)]
pub struct PatternTracker
{
    buffer: BitVec,
    pattern: HashMap<usize, Counter>,
}

impl PatternTracker
{
    /// Creates a new `PatternTracker`.
    pub fn new() -> PatternTracker
    {
        PatternTracker {
            buffer: BitVec::new(),
            pattern: HashMap::new(),
        }
    }

    /// Push input.
    pub fn push(&mut self, input: bool)
    {
        if let Some(key) = self.last_ngram(NGRAM_SIZE) {
            let counter = if let Some(counter) = self.pattern.get_mut(&key) {
                counter
            } else {
                self.pattern.insert(key.clone(), [0, 0]);
                self.pattern.get_mut(&key).unwrap()
            };

            if input {
                counter[1] += 1;
            } else {
                counter[0] += 1;
            }
        }

        self.buffer.push(input);
    }

    fn predict(&self, key: &usize) -> bool
    {
        match self.pattern.get(key) {
            Some(counter) if counter[1] > counter[0] => true,
            Some(counter) if counter[0] > counter[1] => false,
            _ => rand::thread_rng().gen(),
        }
    }

    /// Predicts next input if there are enough pushed data.
    pub fn predict_next(&self) -> Option<bool>
    {
        self.last_ngram(NGRAM_SIZE).map(|key| self.predict(&key))
    }

    fn last_ngram(&self, n: usize) -> Option<usize>
    {
        if self.buffer.len() >= n {
            let (_, bits) = self
                .buffer
                .as_bitslice()
                .split_at(self.buffer.len() - NGRAM_SIZE);

            let mut key = 0;
            key.view_bits_mut::<Lsb0>()[..bits.len()].clone_from_bitslice(bits);

            Some(key)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests
{
    use rand::Rng;

    use super::*;

    #[test]
    fn predict_homogeneous_input()
    {
        let mut pattern_tracker = PatternTracker::new();

        for _ in 0..NGRAM_SIZE + 1 {
            pattern_tracker.push(true);
        }

        assert_eq!(pattern_tracker.predict_next().unwrap(), true);

        for _ in 0..NGRAM_SIZE + 1 {
            pattern_tracker.push(false);
        }

        assert_eq!(pattern_tracker.predict_next().unwrap(), false);
    }

    #[test]
    fn predict_fully_predictable_pattern()
    {
        let mut rng = rand::thread_rng();
        let mut pattern_tracker = PatternTracker::new();
        let n = 1 << NGRAM_SIZE;

        // Initialize maping from a 2^NGRAM_SIZE bit sequence to next input.
        let pattern: HashMap<usize, bool> = (0..n).map(|key| (key, rng.gen())).collect();

        println!("{:?}", pattern);

        let mut key = 1;

        // Push inputs to the pattern tracker so it will be able to make qualified predictions.
        for _ in 0..n {
            let input = pattern[&key];

            pattern_tracker.push(input);
            key = ((n - 1) & (key << 1)) | if input { 1 } else { 0 };

            println!("key: {}, input: {}", key, input);
        }

        println!("Pattern tracker loaded - begin assertions");

        // Continue to push inputs but also require every prediction the pattern tracker makes to be
        // correct.
        for _ in 0..32 {
            let input = pattern[&key];

            let prediction = pattern_tracker.predict_next().unwrap();
            assert_eq!(prediction, input);

            pattern_tracker.push(input);
            key = ((n - 1) & (key << 1)) | if input { 1 } else { 0 };

            println!("key: {}, input: {}", key, input);
        }
    }
}
