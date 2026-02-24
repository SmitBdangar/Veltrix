//! Stack-based scene manager.

use super::scene::Scene;

/// Manages a stack of active scenes.
///
/// Top of the stack is the currently active scene.
#[derive(Default)]
pub struct SceneManager {
    stack: Vec<Scene>,
}

impl SceneManager {
    /// Create a new scene manager.
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Push a new scene onto the stack, making it active.
    pub fn push(&mut self, scene: Scene) {
        self.stack.push(scene);
    }

    /// Pop the active scene off the stack, resuming the one beneath it.
    pub fn pop(&mut self) -> Option<Scene> {
        self.stack.pop()
    }

    /// Clear the stack and push a new scene (effectively switching scenes completely).
    pub fn switch(&mut self, scene: Scene) {
        self.stack.clear();
        self.stack.push(scene);
    }

    /// Returns a mutable reference to the currently active scene (top of stack).
    pub fn active_mut(&mut self) -> Option<&mut Scene> {
        self.stack.last_mut()
    }

    /// Returns a reference to the active scene.
    pub fn active(&self) -> Option<&Scene> {
        self.stack.last()
    }

    /// Check if there are any active scenes.
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_operations() {
        let mut sm = SceneManager::new();
        assert!(sm.is_empty());

        sm.push(Scene::new());
        assert!(!sm.is_empty());

        let mut s2 = Scene::new();
        s2.set_modal(true);
        sm.push(s2);

        assert!(sm.active().unwrap().is_modal);

        sm.pop();
        assert!(!sm.active().unwrap().is_modal);

        sm.switch(Scene::new());
        assert!(!sm.active().unwrap().is_modal);
    }
}
