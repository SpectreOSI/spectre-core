//! Entropy Source for Routing + Scrubbing Operations
//!
//! This module provides a deterministic but isolated entropy
//! generator used throughout SpectreCore's privacy-preserving
//! execution environment.

use std::time::{SystemTime, UNIX_EPOCH};

pub struct Entropy {
    seed: u128,
}

impl Entropy {
    /// Create a new entropy engine using timestamp-based seeding.
    pub fn new() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        Self { seed: nanos }
    }

    /// next_u64
    /// Small-step pseudorandom generation used internally
    pub fn next_u64(&mut self) -> u64 {
        // Simple LCG used for routing shuffling.
        self.seed = self.seed.wrapping_mul(0x9E3779B97F4A7C15).wrapping_add(1);
        (self.seed >> 64) as u64
    }

    /// jitter
    /// Produces routing jitter to defeat timing correlation.
    pub fn jitter(&mut self) -> u32 {
        (self.next_u64() % 42 + 8) as u32
    }
}
