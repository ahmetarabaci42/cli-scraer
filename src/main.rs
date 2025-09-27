use clap::Parser;
use reqwest::Client;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Parser, Debug)]
#[command(name = "cli-scraper", about = "Simple async HTML scraper in Rust")]
struct Args {
    /// Target URL to scrape
    url: String,
    /// Output format: json | lines
    #[arg(long, default_value = "json")]
    format: String,
}

#[derive(Serialize)]
struct PageData {
    titles: Vec<String>,
    links: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let client = Client::builder().build()?;
    let html = client.get(&args.url).send().await?.text().await?;

    let doc = Html::parse_document(&html);
    let sel_titles = Selector::parse("h1, h2, h3, h4, h5, h6").unwrap();
    let sel_links = Selector::parse("a[href]").unwrap();

    let titles = doc
        .select(&sel_titles)
        .map(|n| n.text().collect::<String>().trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let links = doc
        .select(&sel_links)
        .filter_map(|n| n.value().attr("href"))
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let data = PageData { titles, links };

    match args.format.as_str() {
        "json" => println!("{}", serde_json::to_string_pretty(&data)?),
        "lines" => {
            for t in &data.titles {
                println!("[TITLE] {t}");
            }
            for l in &data.links {
                println!("[LINK] {l}");
            }
        }
        _ => eprintln!("Unknown format: {}", args.format),
    }

    Ok(())
}
