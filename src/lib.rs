//! Simple flag implementation for tokio.

use std::sync::Arc;
use tokio::sync::RwLock;

/// Structure that consists of a `bool` wrapped in `Arc` and `RwLock` to ensure thread safety and protect against race conditions.
/// Since the `RwLock` used is from the `tokio`, it can be used asynchronously.
/// **Important:** Before passing the flag to a new thread clone it.
#[derive(Clone)]
pub struct Flag(pub Arc<RwLock<bool>>);

impl Flag {
    /// Creates a new `Flag`.
    pub fn new(value: bool) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }

    /// Reads the boolean value of the `Flag` (without blocking other threads from reading the value as well).
    pub async fn read(&self) -> bool {
        *self.0.read().await
    }

    /// Write a new boolean value to the `Flag` (blocks read and write operations from all threads until completed).
    pub async fn write(&self, value: bool) {
        *self.0.write().await = value;
    }

    /// Creates a new reference to the flag so it can be used by a new thread.
    pub fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}
