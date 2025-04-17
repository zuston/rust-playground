
#[cfg(test)]
mod test {
    trait Handler {
        fn handle(&self);
    }

    struct ConcreteHandler;

    impl Handler for ConcreteHandler {
        fn handle(&self) {
            println!("Handling request in ConcreteHandler");
        }
    }

    trait Layer {
        fn wrap(&self, handler: Box<dyn Handler>) -> Box<dyn Handler>;
    }

    struct LoggingLayer;

    impl Layer for LoggingLayer {
        fn wrap(&self, handler: Box<dyn Handler>) -> Box<dyn Handler> {
            Box::new(LoggingMiddleware { handler })
        }
    }

    struct LoggingMiddleware {
        handler: Box<dyn Handler>,
    }

    impl Handler for LoggingMiddleware {
        fn handle(&self) {
            println!("Logging before handling");
            self.handler.handle();
            println!("Logging after handling");
        }
    }

    struct MetricLayer;

    impl Layer for MetricLayer {
        fn wrap(&self, handler: Box<dyn Handler>) -> Box<dyn Handler> {
            Box::new(MetricMiddleware { handler })
        }
    }

    struct MetricMiddleware {
        handler: Box<dyn Handler>,
    }

    impl Handler for MetricMiddleware {
        fn handle(&self) {
            println!("Metrics before handling");
            self.handler.handle();
            println!("Metrics after handling");
        }
    }


    struct OperatorBuilder {
        handler: Box<dyn Handler>,
    }

    impl OperatorBuilder {
        fn new(handler: Box<dyn Handler>) -> Self {
            OperatorBuilder { handler }
        }

        fn layer<L: Layer>(self, layer: L) -> OperatorBuilder {
            OperatorBuilder {
                handler: layer.wrap(self.handler),
            }
        }

        fn build(self) -> Box<dyn Handler> {
            self.handler
        }
    }

    #[test]
    fn test_layer_builder() {
        let handler = ConcreteHandler;
        let logging_layer = LoggingLayer;
        let metric_layer = MetricLayer;

        let operator = OperatorBuilder::new(Box::new(handler))
            .layer(logging_layer)
            .layer(metric_layer)
            .build();
        operator.handle();
    }
}