use nanorand::{ChaCha, Rng};
use once_cell::sync::Lazy;
use std::sync::{Mutex, PoisonError};

/// Cryptographically-secure Pseudo-Random Number Generator
pub static CSPRNG: Lazy<Mutex<ChaCha<20>>> = Lazy::new(|| Mutex::new(ChaCha::new()));

#[allow(clippy::module_name_repetitions)]
#[must_use]
pub fn random_positive_nonzero_i64() -> i64 {
    let mut rng_guard = CSPRNG.lock().unwrap_or_else(PoisonError::into_inner);
    rng_guard.generate_range(1_i64..i64::MAX)
}
