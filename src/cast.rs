#[cfg(test)]
mod test {
    use std::any::Any;

    // refer to: https://bennett.dev/rust/downcast-trait-object/

    #[test]
    fn test_cast() {
        struct A;
        struct B;

        trait Car: Any {
            fn as_any(&self) -> &dyn Any;
        }
        impl Car for A {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
        impl Car for B {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        let a = A {};
        let b = B {};

        let a_car: Box<dyn Car> = Box::new(a);
        let b_car: Box<dyn Car> = Box::new(b);

        let a_car = a_car
            .as_any()
            .downcast_ref::<A>()
            .expect("wrong type of dyn any");
    }
}
