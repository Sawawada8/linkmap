use anyhow::Error;
use clap::Parser;
use url::Url;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    // #[arg(short, long)]
    url: Url,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    depth: u8,
}

pub fn exec() -> Result<(), Error> {
    let arg = Args::parse();

    searcher::search(&arg.url, arg.depth as usize);

    Ok(())
}

mod searcher {
    use std::collections::{HashSet, VecDeque};

    use reqwest::blocking::Client;
    use url::Url;

    pub fn search(url: &Url, count: usize) -> Vec<Url> {
        let client = Client::new();
        let mut c = 1;
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back(url.clone());
        while q.len() > 0 && c < count {
            if let Some(url) = q.pop_front() {
                println!("----------------------------------");
                println!("start!, c: {}, url: {}", c, url);
                println!("----------------------------------");
                let res = client.get(url.clone()).send().unwrap();
                let status = res.status();
                match res.status() {
                    reqwest::StatusCode::OK => {}
                    reqwest::StatusCode::FORBIDDEN => {
                        println!("403 Forbidden: {}", url);
                    }
                    _ => {
                        println!("Error: {}, url: {}", status, url);
                    }
                }
                let body = res.text().unwrap();
                let ankers = extraction_anker(&body, &url);
                ankers.iter().for_each(|url| {
                    if visited.contains(url) {
                        return;
                    } else {
                        visited.insert(url.clone());
                        q.push_back(url.clone());
                    }
                });
                c += 1;
            } else {
                panic!("invalid q");
            }
        }
        visited.into_iter().collect::<Vec<_>>()
    }

    pub fn extraction_anker(html: &str, base: &Url) -> Vec<Url> {
        let doc = scraper::Html::parse_document(html);
        let ankers = doc
            .select(&scraper::Selector::parse("a").unwrap())
            .collect::<Vec<_>>();
        println!("Found {} links", ankers.len());
        println!("list: {:?}", ankers);

        let urls = ankers
            .iter()
            .filter_map(|a| a.value().attr("href"))
            .map(|s| {
                Url::parse(s).unwrap_or_else(|err| {
                    println!("Failed to parse URL: {}, err: {}", s, err);
                    // visited なのでスキップされる
                    Url::parse(base.as_str()).unwrap()
                })
            })
            .collect::<Vec<_>>();

        for url in &urls {
            println!("- {}", url.as_str());
        }
        urls
    }
}
