use floem::reactive::{
    create_effect, create_rw_signal, RwSignal, SignalGet, SignalSet,
};

/// A signal listener that receives 'events' from the outside and runs the callback.  
/// This is implemented using effects and normal rw signals. This should be used when it doesn't  
/// make sense to think of it as 'storing' a value, like an `RwSignal` would typically be used for.
///  
/// Copied/Cloned listeners refer to the same listener.
#[derive(Debug)]
pub struct Listener<T: 'static> {
    val: RwSignal<Option<T>>,
}

impl<T: Clone + 'static> Listener<T> {
    pub fn new(on_val: impl Fn(T) + 'static) -> Listener<T> {
        let val = create_rw_signal(None);

        let listener = Listener { val };
        listener.listen(on_val);

        listener
    }

    /// Construct a listener when you can't yet give it a callback.  
    /// Call `listen` to set a callback.
    pub fn new_empty() -> Listener<T> {
        let val = create_rw_signal(None);
        Listener { val }
    }

    /// Listen for values sent to this listener.  
    pub fn listen(self, on_val: impl Fn(T) + 'static) {
        let val = self.val;

        create_effect(move |_| {
            // TODO(minor): Signals could have a `take` method to avoid cloning.
            if let Some(cmd) = val.get() {
                on_val(cmd);
            }
        });
    }

    /// Send a value to the listener.
    pub fn send(&self, v: T) {
        self.val.set(Some(v));
    }
}

impl<T: 'static> Copy for Listener<T> {}

impl<T: 'static> Clone for Listener<T> {
    fn clone(&self) -> Self {
        Listener { val: self.val }
    }
}
