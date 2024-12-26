use anyhow::Result;
use bytes::Bytes;
use clap::{App, Arg};
use positioned_io::ReadAt;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::time::Instant;

fn main() -> Result<()> {
    let cli = App::new("io_read_bench")
        .arg(Arg::with_name("data_path").takes_value(true))
        .arg(Arg::with_name("data_bytes").takes_value(true))
        .arg(Arg::with_name("read_batch_bytes").takes_value(true))
        .arg(Arg::with_name("bench_type").takes_value(true))
        .get_matches();

    let data_path = cli.value_of("data_path").unwrap();
    let total_data_bytes: usize = cli.value_of("data_bytes").unwrap().parse().unwrap();
    let batch_read_bytes: usize = cli.value_of("read_batch_bytes").unwrap().parse().unwrap();

    let data_bytes = Bytes::from(vec![0; total_data_bytes]);
    let file_path = format!("{}/{}", &data_path, "1.data");
    if !std::path::Path::new(&file_path).exists() {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&file_path)?;
        file.write_all(&*data_bytes)?;
        file.flush()?;

        println!("file: {} created", &file_path);
    }

    let timer = Instant::now();
    let bench_type = cli.value_of("bench_type").unwrap();
    if bench_type == "buf_read" {
        read_with_buf(file_path, batch_read_bytes, total_data_bytes)?;
    } else if bench_type == "read" {
        read(file_path, batch_read_bytes, total_data_bytes)?;
    } else if bench_type == "buf_read_of_relative_seek" {
        read(file_path, batch_read_bytes, total_data_bytes)?;
    } else if bench_type == "pread" {
        pread(file_path, batch_read_bytes, total_data_bytes)?;
    }
    println!(
        "[{}] cost: {} (ms)",
        bench_type,
        timer.elapsed().as_millis()
    );

    Ok(())
}

fn pread(file_path: String, batch_read_bytes: usize, total_data_bytes: usize) -> Result<()> {
    let loop_cnt = total_data_bytes / batch_read_bytes + 1;
    let mut offset = 0;
    let mut buffer = vec![0; batch_read_bytes];
    let mut total_size = 0;
    for _ in 0..loop_cnt {
        buffer.clear();
        let mut reader = File::open(&file_path)?;
        let mut buffer = vec![0; batch_read_bytes];
        let bytes_read = reader.read_at(offset, &mut buffer)?;
        let size = bytes_read;
        offset += buffer.len() as u64;
        // println!("read #{} - {}", offset, size);
        total_size += size;
    }

    // println!("total_size: {}", total_size);

    Ok(())
}

fn read(file_path: String, batch_read_bytes: usize, total_data_bytes: usize) -> Result<()> {
    let loop_cnt = total_data_bytes / batch_read_bytes + 1;
    let mut offset = 0;
    let mut buffer = vec![0; batch_read_bytes];
    let mut total_size = 0;
    for _ in 0..loop_cnt {
        buffer.clear();
        let mut reader = File::open(&file_path)?;
        reader.seek(SeekFrom::Start(offset as u64))?;

        let mut buffer = vec![0; batch_read_bytes];
        reader.read(&mut buffer)?;
        let size = buffer.len();
        offset += buffer.len();

        // println!("read #{} - {}", offset, size);

        total_size += size;
    }

    // println!("total_size: {}", total_size);

    Ok(())
}

fn read_with_buf(
    file_path: String,
    batch_read_bytes: usize,
    total_data_bytes: usize,
) -> Result<()> {
    let loop_cnt = total_data_bytes / batch_read_bytes + 1;
    let mut offset = 0;
    let mut buffer = vec![0; batch_read_bytes];
    let mut total_size = 0;
    for _ in 0..loop_cnt {
        buffer.clear();
        let file = File::open(&file_path)?;
        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(offset as u64))?;

        let mut buffer = vec![0; batch_read_bytes];
        reader.read(&mut buffer)?;
        let size = buffer.len();
        offset += buffer.len();

        // println!("read #{} - {}", offset, size);

        total_size += size;
    }

    // println!("total_size: {}", total_size);

    Ok(())
}

fn read_with_buf_of_relative_seek(
    file_path: String,
    batch_read_bytes: usize,
    total_data_bytes: usize,
) -> Result<()> {
    let loop_cnt = total_data_bytes / batch_read_bytes + 1;
    let mut offset = 0;
    let mut buffer = vec![0; batch_read_bytes];
    let mut total_size = 0;
    for _ in 0..loop_cnt {
        buffer.clear();
        let file = File::open(&file_path)?;
        let mut reader = BufReader::new(file);
        reader.seek_relative(offset)?;

        let mut buffer = vec![0; batch_read_bytes];
        reader.read(&mut buffer)?;
        let size = buffer.len();
        offset += buffer.len() as i64;

        // println!("read #{} - {}", offset, size);

        total_size += size;
    }

    // println!("total_size: {}", total_size);

    Ok(())
}
