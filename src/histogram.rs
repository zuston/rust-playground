#[cfg(test)]
mod test {
    use hdrhistogram::Histogram;

    #[test]
    fn test_histogram() {
        let mut histogram = Histogram::<u64>::new(3).unwrap();
        histogram.record(1).unwrap();
        histogram.record(2).unwrap();
        histogram.record(3).unwrap();

        let p99 = histogram.value_at_quantile(0.99);
        let p50 = histogram.value_at_quantile(0.50);
        println!("p99: {}", p99);
        println!("p50: {}", p50);

        // histogram.clear();

        histogram.record(10).unwrap();
        histogram.record(20).unwrap();
        histogram.record(30).unwrap();

        let p99 = histogram.value_at_quantile(0.99);
        let p50 = histogram.value_at_quantile(0.50);
        println!("p99: {}", p99);
        println!("p50: {}", p50);
    }
}
