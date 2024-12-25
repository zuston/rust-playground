#[cfg(test)]
mod test {
    use std::thread::sleep;
    use std::time::Duration;

    #[tokio::test]
    async fn test_await_timeout() {
        let future = tokio::spawn(async {
            // sleep(Duration::from_millis(100000));
            tokio::time::sleep(Duration::from_secs(10)).await;
        });
        match tokio::time::timeout(Duration::from_secs(5), future).await {
            Err(e) => panic!("timeout: {:?}", e),
            Ok(result) => result.unwrap(),
        }
    }
}