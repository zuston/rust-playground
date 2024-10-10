use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::sleep;
use std::time::{Duration, Instant};
use bytes::Bytes;
use clap::{App, Arg};
use tokio::io::AsyncWriteExt;

fn main() {
    let cli = App::new("io_bench")
        .arg(
            Arg::with_name("data_path")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("epoch")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("batch_bytes")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("io_bench_type")
                .takes_value(true),
        )
        .get_matches();

    let data_path = cli.value_of("data_path").unwrap();
    let epoch: usize = cli.value_of("epoch").unwrap().parse().unwrap();
    let batch_bytes: usize = cli.value_of("batch_bytes").unwrap().parse().unwrap();
    let batch_bytes = Bytes::from(vec![0; batch_bytes]);

    let io_bench_type = cli.value_of("io_bench_type").unwrap();
    if io_bench_type == "std_thread_buffer_io" {
        println!("io bench: [std_thread_buffer_io]");
        let _ = std_thread_buffer_io(epoch, batch_bytes, data_path.to_string());
        return;
    }

    if io_bench_type == "tokio_async_buffer_io" {
        println!("io bench: [tokio_async_buffer_io]");
        let _ = tokio_async_buffer_io(epoch, batch_bytes, data_path.to_string());
        return;
    }

    println!("Unknown IO type");
}

fn std_thread_buffer_io(epoch: usize, batch_bytes: Bytes, data_dir: String) -> anyhow::Result<()> {
    let cores = std::thread::available_parallelism().unwrap();

    fn creat_file(file_path: &str) {
        if !std::path::Path::new(file_path).exists() {
            println!("creating file: {}", file_path);
            let _ = std::fs::File::create(file_path);
        }
    }

    let now = Instant::now();
    let mut handles = vec![];
    for idx in 0..usize::from(cores) {
        let data_dir = data_dir.to_string();
        let batch_bytes = batch_bytes.clone();

        let handle = std::thread::spawn(move || {
            let file_path = format!("{}/{}.std_thread_buffer_io.file", &data_dir, idx);
            creat_file(&file_path);
            let file = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path).unwrap();
            let mut buf_write = std::io::BufWriter::new(file);
            for _ in 0..epoch {
                let _ = buf_write.write_all(&batch_bytes).unwrap();
            }
            let _ = buf_write.flush().unwrap();
            println!("[{:?}] finished", idx);
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
    println!("std_thread_buffer_io: {}(ms)", now.elapsed().as_millis());
    Ok(())
}

fn tokio_async_buffer_io(epoch: usize, batch_bytes: Bytes, data_dir: String) -> anyhow::Result<()> {
    let cores = std::thread::available_parallelism().unwrap();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(usize::from(cores))
        .enable_all().build()?;

    async fn create_file(data_path: &str) -> anyhow::Result<()> {
        println!("creating file: {:?}", data_path);
        let file_path = data_path;
        match tokio::fs::metadata(file_path).await {
            Ok(_) => {
            }
            Err(e) if e.kind() == tokio::io::ErrorKind::NotFound => {
                let _ = tokio::fs::File::create(file_path).await?;
            }
            Err(e) => {

            }
        }
        Ok(())
    }

    let now = Instant::now();
    let counter = Arc::new(AtomicUsize::new(0));
    for idx in 0..usize::from(cores) {
        let data_dir = data_dir.to_string();
        let batch_bytes = batch_bytes.clone();
        let counter = counter.clone();
        runtime.spawn(async move {
            let file_path = format!("{}/{}.tokio_async_buffer_io.file", &data_dir, idx);
            let _ = create_file(&file_path).await;
            let file = tokio::fs::OpenOptions::new()
                .append(true)
                .open(file_path)
                .await.unwrap();
            let mut buf_write = tokio::io::BufWriter::new(file);
            for _ in 0..epoch {
                if let Err(e) = buf_write.write_all(&batch_bytes).await {
                    println!("Errors on writing data. err: {:#?}", e);
                }
            }
            let _ = buf_write.flush().await;
            counter.fetch_add(1, Ordering::SeqCst);
            println!("[{:?}] finished", idx);
        });
    }

    loop {
        if counter.load(Ordering::SeqCst) == cores.get() {
            break;
        }
        sleep(Duration::from_millis(100));
    }

    println!("tokio_async_buffer_io: {}(ms)", now.elapsed().as_millis());
    Ok(())
}