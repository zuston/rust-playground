use std::io;
use std::io::Write;
use bytes::Bytes;
use clap::{App, Arg};
use hdrs::ClientBuilder;
use url::Url;

fn main() -> anyhow::Result<()> {
    let cli = App::new("hdfs_write")
        .arg(Arg::with_name("file_path").takes_value(true))
        .arg(Arg::with_name("batch_size").takes_value(true))
        .arg(Arg::with_name("batch").takes_value(true))
        .get_matches();

    let file_path = cli.value_of("file_path").unwrap();
    println!("Gotten file path: {}", &file_path);
    let url = Url::parse(file_path)?;
    let url_header = format!("{}://{}", url.scheme(), url.host().unwrap());
    let root_path = url.path();
    println!(
        "Creating hdfs client, header: {}, path: {}",
        &url_header, root_path
    );

    let batch_size: usize = cli.value_of("batch_size").unwrap().parse().unwrap();
    println!("Gotten batch_size: {}", &batch_size);

    let batch: usize = cli.value_of("batch").unwrap().parse().unwrap();
    println!("Gotten batch: {}", batch);

    let bytes = Bytes::from(vec![0; batch_size]);

    let fs =
        ClientBuilder::new(url_header.as_str())
            .connect()?;

    match fs.metadata(root_path) {
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                println!("There is no such file: {}", root_path);
                let mut file = fs.open_file().write(true).create(true).open(root_path)?;
                file.flush()?;
                println!("Created the file: {}", root_path);
            }
        },
        _ => {
            fs.remove_file(root_path)?;
            println!("Removed the file: {}", root_path);
        }
    }

    for idx in 0..batch {
        let mut f = fs.open_file().write(true).append(true).open(root_path)?;
        f.write_all(&bytes.clone())?;
        f.flush()?;
    }

    println!("Finished file: {}", root_path);

    Ok(())
}