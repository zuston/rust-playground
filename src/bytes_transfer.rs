extern crate libc;

use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use bytes::{BufMut, Bytes, BytesMut};

fn main() {
    let bytes = create_1gb_bytes();

    match get_resident_memory_size() {
        Ok(rss) => println!("Resident memory size: {} bytes", rss),
        Err(e) => eprintln!("Failed to get resident memory size: {}", e),
    }

    let mut cloned = BytesMut::with_capacity(1 << 30);

    // this will make real memory copy
    // cloned.extend_from_slice(&*bytes);

    // this will zero copy of putting
    cloned.put(bytes);
    let c = cloned.freeze();

    println!("----");
    for _ in 0..10 {
        sleep(Duration::from_secs(10));
        match get_resident_memory_size() {
            Ok(rss) => println!("Resident memory size: {} bytes", rss),
            Err(e) => eprintln!("Failed to get resident memory size: {}", e),
        }
    }

    println!("size: {}", c.len());
}

fn create_1gb_bytes() -> Bytes {
    let size = 1 << 30; // 1GB = 2^30 bytes

    let mut buffer = BytesMut::with_capacity(size);

    buffer.resize(size, 0);

    buffer.freeze()
}


fn get_resident_memory_size() -> Result<usize, String> {
    // 获取当前进程的 PID
    let pid = std::process::id();

    // 使用 `ps` 命令来获取进程信息
    let output = Command::new("ps")
        .args(&["-o", "rss=", "-p", &pid.to_string()])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!(
            "ps command failed with status: {}",
            output.status
        ));
    }

    // 将输出转换为字符串并解析为 usize
    let rss_str = std::str::from_utf8(&output.stdout).map_err(|e| e.to_string())?;
    let rss = rss_str.trim().parse::<usize>().map_err(|e| e.to_string())?;

    Ok(rss * 1024) // `ps` 返回的值通常是以 KB 为单位，转换为字节
}