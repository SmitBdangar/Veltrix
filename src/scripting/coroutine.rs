//! Yielding coroutines for delayed execution and sequences.

/// Represents the status of a coroutine execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoroutineStatus {
    /// The coroutine is still running and waiting for time to pass.
    Waiting,
    /// The coroutine has finished executing.
    Finished,
}

/// A rudimentary coroutine manager.
///
/// In Rust, true coroutines (generators) are unstable without async/await.
/// Here we use a closure-based approach or explicit state machines.
pub struct Coroutine {
    /// Time remaining to wait.
    pub timer: f32,
    /// Function to execute when timer reaches 0. If it returns `Waiting`,
    /// it reschedules itself.
    pub task: Box<dyn FnMut() -> CoroutineStatus + Send + Sync>,
}

impl Coroutine {
    /// Create a new coroutine that waits `delay` seconds, then executes.
    pub fn new(delay: f32, task: impl FnMut() -> CoroutineStatus + Send + Sync + 'static) -> Self {
        Self {
            timer: delay,
            task: Box::new(task),
        }
    }

    /// Advance the coroutine timer. Returns the new status.
    pub fn update(&mut self, dt: f32) -> CoroutineStatus {
        if self.timer > 0.0 {
            self.timer -= dt;
            CoroutineStatus::Waiting
        } else {
            (self.task)()
        }
    }
}
