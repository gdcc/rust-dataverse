use std::sync::{Arc, Mutex};

pub type CallbackFunInner = Box<dyn FnMut(u64) + Send>;

// Define a wrapper struct
pub struct CallbackFun {
    inner: Arc<Mutex<Box<CallbackFunInner>>>,
}

// Implement `Clone` for the wrapper struct
impl Clone for CallbackFun {
    fn clone(&self) -> Self {
        CallbackFun {
            inner: Arc::clone(&self.inner),
        }
    }
}

// Implement other methods if necessary
impl CallbackFun {
    pub fn new(f: CallbackFunInner) -> Self {
        CallbackFun {
            inner: Arc::new(Mutex::new(Box::new(f))),
        }
    }

    pub fn call(&self, arg: u64) {
        let mut f = self.inner.lock().unwrap();
        f(arg);
    }

    pub fn wrap<F>(closure: F) -> Self
    where
        F: FnMut(u64) + Send + 'static,
    {
        CallbackFun::new(Box::new(closure))
    }
}