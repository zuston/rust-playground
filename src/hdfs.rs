use bytes::Bytes;
use clap::{App, Arg};
use hdfs_native::{Client, WriteOptions};
use url::Url;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = App::new("hdfs_write")
        .arg(Arg::with_name("file_path").takes_value(true))
        .arg(Arg::with_name("size").takes_value(true))
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

    let size: usize = cli.value_of("size").unwrap().parse().unwrap();
    println!("Gotten size: {}", &size);

    let bytes = Bytes::from(vec![0; size]);

    let client = Client::new(url_header.as_str())?;

    let file_path = root_path;
    if let Ok(status) = client.get_file_info(file_path).await {
        match client.delete(file_path, false).await {
            Ok(_) => println!("deleted."),
            _ => panic!("delete failure."),
        }
    }

    client
        .create(file_path, WriteOptions::default())
        .await?
        .close()
        .await?;

    println!("created");

    let mut file_writer = client.append(file_path).await?;
    file_writer.write(bytes).await?;
    file_writer.close().await?;

    println!("finished");

    Ok(())
}
