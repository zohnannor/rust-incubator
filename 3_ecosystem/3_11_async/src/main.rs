use std::path::PathBuf;

use clap::Parser;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

fn cpus() -> &'static str {
    Box::leak(num_cpus::get().to_string().into_boxed_str())
}

#[derive(clap::Parser)]
struct Args {
    /// Maximum amount of threads to spawn
    #[clap(short, long, value_parser, default_value = cpus())]
    max_threads: usize,

    /// File to read links from
    file: PathBuf,
}

fn main() {
    let Args { max_threads, file } = Args::parse();

    dbg!(&max_threads, &file);

    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(max_threads)
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let client = reqwest::Client::new();

        let file = tokio::fs::File::open(file).await?;
        let file = BufReader::new(file);
        let mut lines = file.lines();

        let mut downloads = vec![];

        while let Ok(Some(url)) = lines.next_line().await {
            let h = tokio::spawn(download_url(client.clone(), url));
            downloads.push(h);
        }

        for h in downloads {
            h.await?.unwrap();
        }

        Ok::<_, Box<dyn std::error::Error>>(())
    })
    .unwrap();
}

async fn download_url(
    client: reqwest::Client,
    url: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = reqwest::Url::parse(&url)?;
    let html = client.get(url.clone()).send().await?.text().await?;

    let path: String = url.path_segments().unwrap().collect::<Vec<_>>().join("_");
    dbg!(&path);
    let mut file = tokio::fs::File::create(path).await?;
    file.write_all(html.as_bytes()).await?;

    Ok(())
}
