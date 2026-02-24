//! Finite State Machine for AI and game logic.

use std::collections::HashMap;

/// Trait representing a generic state in a state machine.
pub trait State<T> {
    /// Called when the state is entered.
    fn on_enter(&mut self, context: &mut T) {
        let _ = context;
    }
    
    /// Called every frame. Returns an optional string to transition to a new state.
    fn on_update(&mut self, context: &mut T, dt: f32) -> Option<String> {
        let _ = (context, dt);
        None
    }
    
    /// Called when the state is exited.
    fn on_exit(&mut self, context: &mut T) {
        let _ = context;
    }
}

/// A generic finite state machine (FSM) manager.
pub struct StateMachine<T> {
    states: HashMap<String, Box<dyn State<T> + Send + Sync>>,
    current_state: Option<String>,
}

impl<T> Default for StateMachine<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> StateMachine<T> {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            current_state: None,
        }
    }

    /// Add a state with a string identifier.
    pub fn add_state(&mut self, name: &str, state: impl State<T> + Send + Sync + 'static) {
        self.states.insert(name.to_string(), Box::new(state));
    }

    /// Set the initial state.
    pub fn start(&mut self, name: &str, context: &mut T) {
        self.current_state = Some(name.to_string());
        if let Some(state) = self.states.get_mut(name) {
            state.on_enter(context);
        }
    }

    /// Update the current state, handling transitions automatically.
    pub fn update(&mut self, context: &mut T, dt: f32) {
        let mut next_state_name = None;

        if let Some(ref current_name) = self.current_state {
            if let Some(state) = self.states.get_mut(current_name) {
                next_state_name = state.on_update(context, dt);
            }
        }

        if let Some(new_name) = next_state_name {
            self.transition_to(&new_name, context);
        }
    }

    fn transition_to(&mut self, new_name: &str, context: &mut T) {
        // Exit old
        if let Some(ref current_name) = self.current_state {
            if let Some(state) = self.states.get_mut(current_name) {
                state.on_exit(context);
            }
        }

        // Enter new
        self.current_state = Some(new_name.to_string());
        if let Some(state) = self.states.get_mut(new_name) {
            state.on_enter(context);
        }
    }
}
