#[cfg(test)]
mod test {
    use std::sync::atomic::AtomicU64;
    use std::sync::atomic::Ordering::SeqCst;
    use std::sync::Arc;

    struct Manager {
        size: AtomicU64,
    }

    struct Event {
        is_valid_func: Box<dyn Fn() -> bool>,
    }

    #[test]
    fn test_closure() {
        let manager = Manager {
            size: Default::default(),
        };
        let manager_ref = Arc::new(manager);

        impl Event {
            fn is_valid(&self) -> bool {
                (self.is_valid_func)()
            }
        }

        let closure = {
            let cloned = manager_ref.clone();
            move || -> bool { cloned.size.load(SeqCst) == 0u64 }
        };
        let event = Event {
            is_valid_func: Box::new(closure),
        };
        assert_eq!(true, event.is_valid());
    }
}
