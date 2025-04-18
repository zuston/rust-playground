use anyhow::Result;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use clap::{App, Arg};
use hdrs::ClientBuilder;
use url::Url;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[allow(non_upper_case_globals)]
#[export_name = "malloc_conf"]
pub static malloc_conf: &[u8] = b"prof:true,prof_active:true,lg_prof_sample:19\0";

#[tokio::main]
async fn main() -> Result<()> {
    let mut v = vec![];
    for i in 0..1000000 {
        v.push(i);
    }

    let cli = App::new("hdfs_with_memory_profile")
        .arg(Arg::with_name("file_path").takes_value(true))
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
    let fs = ClientBuilder::new(url_header.as_str()).connect()?;
    println!("Created hdfs client");
    drop(fs);

    let app = axum::Router::new().route("/debug/pprof/heap", axum::routing::get(handle_get_heap));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

pub async fn handle_get_heap() -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut prof_ctl = jemalloc_pprof::PROF_CTL.as_ref().unwrap().lock().await;
    require_profiling_activated(&prof_ctl)?;
    let pprof = prof_ctl
        .dump_pprof()
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok(pprof)
}

/// Checks whether jemalloc profiling is activated an returns an error response if not.
fn require_profiling_activated(
    prof_ctl: &jemalloc_pprof::JemallocProfCtl,
) -> Result<(), (axum::http::StatusCode, String)> {
    if prof_ctl.activated() {
        Ok(())
    } else {
        Err((
            axum::http::StatusCode::FORBIDDEN,
            "heap profiling not activated".into(),
        ))
    }
}
