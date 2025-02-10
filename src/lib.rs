use std::sync::{Arc, RwLock, Weak};

/// Global Weak Reference type used for autobuilding Arc.
/// Once initialized first upgrade will construct the value.
pub struct GlobalWeak<T> {
    wrapped: RwLock<Weak<T>>,
    builder: fn() -> T,
}

impl<T> GlobalWeak<T> {
    /// Create new weak reference that will be built later.
    pub const fn new(builder: fn() -> T) -> GlobalWeak<T> {
        GlobalWeak {
            wrapped: RwLock::new(Weak::new()),
            builder
        }
    }

    /// create Arc value, or get new strong reference if state is available
    pub fn upgrade(&self) -> Option<Arc<T>>
    {
        let rguard = self.wrapped.read().ok()?;
        if let Some(strongref) = rguard.upgrade() {
            Some(strongref)
        } else {
            std::mem::drop(rguard);
            let mut wguard = self.wrapped.write().ok()?;
            Some(wguard.upgrade().unwrap_or_else(||{
                let arc = Arc::new((self.builder)());
                *wguard = Arc::downgrade(&arc);
                arc
            }))
        }
    }
}

/// Convert the value back to the weak reference
impl<T> From<GlobalWeak<T>> for Weak<T> {
    fn from(value: GlobalWeak<T>) -> Self {
        value.wrapped.into_inner().unwrap()
    }
}

/// Convert the value back to the weak reference
/// This variant of trait creates new weak reference to the same object
impl<T> From<&GlobalWeak<T>> for Weak<T> {
    fn from(value: &GlobalWeak<T>) -> Self {
        let rguard = value.wrapped.read().unwrap();
        rguard.clone()
    }
}



#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    struct Complex {
        a: String
    }

    static TESTSTATIC1: GlobalWeak<Complex> = GlobalWeak::new(|| Complex { a: "klop".to_string() });

    #[test]
    fn test_constructs() {

        assert_eq!(TESTSTATIC1.upgrade().unwrap().a, "klop");
    }
    #[test]
    fn test_converts_weak() {
        let _arc = TESTSTATIC1.upgrade().unwrap();
        let weak: Weak<Complex> = (&TESTSTATIC1).into();
        assert_eq!(weak.strong_count(), 1);
        assert!(weak.weak_count() > 1);
        let _arc2 = weak.upgrade().unwrap();
        assert_eq!(weak.strong_count(), 2);
        let _arc3 = (&TESTSTATIC1).upgrade().unwrap();
        assert_eq!(weak.strong_count(), 3);
    }
}
