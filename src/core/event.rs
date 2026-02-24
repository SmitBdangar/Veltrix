//! Engine-wide publish/subscribe event bus.
//!
//! Any subsystem can emit typed events; any subsystem can subscribe to receive them.
//! Events are collected during a frame and dispatched at the end of the frame.

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Trait implemented by all engine events.
///
/// An event is any `'static + Send + Sync` type (typically a small struct).
pub trait Event: Any + Send + Sync + 'static {}
impl<T: Any + Send + Sync + 'static> Event for T {}

/// A boxed, type-erased event.
type BoxedEvent = Box<dyn Any + Send + Sync>;

/// Callback signature for event handlers.
pub trait EventHandler: Send + Sync {
    /// Called when the subscribed event type is dispatched.
    fn call(&mut self, event: &dyn Any);
}

/// A typed event handler wrapping a closure.
struct HandlerFn<E: Event> {
    f: Box<dyn FnMut(&E) + Send + Sync>,
}

impl<E: Event> EventHandler for HandlerFn<E> {
    fn call(&mut self, event: &dyn Any) {
        if let Some(e) = event.downcast_ref::<E>() {
            (self.f)(e);
        }
    }
}

/// Engine-wide event bus.
///
/// Events are queued with [`emit`](EventBus::emit) and dispatched by calling
/// [`flush`](EventBus::flush) at the end of each frame.
#[derive(Default)]
pub struct EventBus {
    /// Pending events waiting to be dispatched.
    pending: Vec<(TypeId, BoxedEvent)>,
    /// Registered handlers, keyed by event TypeId.
    handlers: HashMap<TypeId, Vec<Box<dyn EventHandler>>>,
}

impl std::fmt::Debug for EventBus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventBus")
            .field("pending_count", &self.pending.len())
            .field("handler_types", &self.handlers.len())
            .finish()
    }
}

impl EventBus {
    /// Create an empty event bus.
    pub fn new() -> Self {
        Self::default()
    }

    /// Queue an event to be dispatched at the end of this frame.
    pub fn emit<E: Event>(&mut self, event: E) {
        self.pending.push((TypeId::of::<E>(), Box::new(event)));
    }

    /// Subscribe a closure to events of type `E`.
    ///
    /// The closure is called once for every `E` event dispatched via [`flush`].
    pub fn subscribe<E: Event>(&mut self, handler: impl FnMut(&E) + Send + Sync + 'static) {
        let tid = TypeId::of::<E>();
        self.handlers
            .entry(tid)
            .or_default()
            .push(Box::new(HandlerFn { f: Box::new(handler) }));
    }

    /// Dispatch all queued events to registered handlers, then clear the queue.
    pub fn flush(&mut self) {
        // Drain pending into a local vec to avoid holding &mut self while calling handlers.
        let events: Vec<(TypeId, BoxedEvent)> = std::mem::take(&mut self.pending);
        for (tid, event) in &events {
            if let Some(handlers) = self.handlers.get_mut(tid) {
                for h in handlers.iter_mut() {
                    h.call(event.as_ref());
                }
            }
        }
    }

    /// Returns the number of events waiting to be dispatched.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[derive(Debug)]
    struct DamageEvent {
        amount: u32,
    }

    #[test]
    fn emit_and_flush_calls_handler() {
        let mut bus = EventBus::new();
        let received = Arc::new(Mutex::new(0u32));
        let r = Arc::clone(&received);

        bus.subscribe(move |ev: &DamageEvent| {
            *r.lock().unwrap() += ev.amount;
        });

        bus.emit(DamageEvent { amount: 10 });
        bus.emit(DamageEvent { amount: 5 });
        assert_eq!(bus.pending_count(), 2);

        bus.flush();
        assert_eq!(bus.pending_count(), 0);
        assert_eq!(*received.lock().unwrap(), 15);
    }

    #[test]
    fn flush_with_no_handlers_does_not_panic() {
        let mut bus = EventBus::new();
        bus.emit(42u32);
        bus.flush(); // Should not panic.
        assert_eq!(bus.pending_count(), 0);
    }
}
