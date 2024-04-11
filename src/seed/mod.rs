use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    rngs::SmallRng,
    Rng, SeedableRng,
};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Seed(u64, SmallRng);

impl Default for Seed {
    fn default() -> Self {
        Self::new()
    }
}

impl Seed {
    /// Returns a new seed.
    ///
    /// This seed is _NOT_ cryprographically secure as it is intended to be reproducible.
    /// It uses the [`UNIX_EPOCH`] const from stdlib.
    pub fn new() -> Seed {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let rng = SmallRng::seed_from_u64(seed);
        Seed(seed, rng)
    }

    /// Return the seed used to generate the RNG.
    pub fn seed(&self) -> u64 {
        self.0
    }

    /// Generate a random u32 number using the seed.
    pub fn next_u32(&mut self) -> u32 {
        self.1.gen()
    }

    /// Generate a random u64 number using the seed.
    pub fn next_u64(&mut self) -> u64 {
        self.1.gen()
    }

    /// Generate a random T from a given range.
    pub fn gen_range<T: SampleUniform>(
        &mut self,
        range: impl SampleRange<T>,
    ) -> impl SampleUniform {
        self.1.gen_range(range)
    }
}

impl std::ops::Deref for Seed {
    type Target = SmallRng;
    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl std::ops::DerefMut for Seed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}
