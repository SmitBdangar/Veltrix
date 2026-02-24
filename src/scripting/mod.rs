//! Scripting: Behavior trait, coroutines, and generic state machine.

pub mod behavior;
pub mod coroutine;
pub mod state_machine;

pub use behavior::Behavior;
pub use state_machine::StateMachine;
