use nanorand::{ChaCha, Rng};
use once_cell::sync::Lazy;
use std::sync::Mutex;

/// Cryptographically-secure Pseudo-Random Number Generator
pub static CSPRNG: Lazy<Mutex<ChaCha<20>>> = Lazy::new(|| Mutex::new(ChaCha::new()));

pub fn random_u64() -> u64 {
    let mut rng_guard = CSPRNG.lock().unwrap_or_else(|e| e.into_inner());
    rng_guard.generate_range(0_u64..u64::MAX)
}
