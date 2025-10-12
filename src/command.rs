use std::collections::{HashSet, VecDeque};

use reqwest::blocking::Client;
use url::Url;

use anyhow::Error;
use clap::Parser;

use crate::scored_url::ScoredUrl;

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
    let original_url = arg.url.clone();

    let mut urls = Searcher::new(original_url.clone())
        .search(arg.depth as usize)
        .into_iter()
        .map(|url| {
            let scored_url = ScoredUrl::new(original_url.clone(), url).calc_score();
            scored_url
        })
        .collect::<Vec<_>>();
    urls.sort_by(|a, b| b.get_score().cmp(&a.get_score()));
    println!("----------------------------------");
    println!("result: {} urls", urls.len());
    println!("|----------------------------------|");
    for s_url in &urls {
        println!("| score={}, {}", s_url.get_score(), s_url.url.as_str());
    }
    println!("|----------------------------------|");

    Ok(())
}

struct Searcher {
    base_url: Url,
    visited: HashSet<Url>,
}

impl Searcher {
    pub fn new(base_url: Url) -> Self {
        Self {
            base_url,
            visited: HashSet::new(),
        }
    }

    pub fn search(&mut self, count: usize) -> Vec<Url> {
        let client = Client::new();
        let mut c = 0;
        let mut q = VecDeque::new();
        q.push_back(self.base_url.clone());
        while q.len() > 0 && c < count {
            if let Some(url) = q.pop_front() {
                println!("----------------------------------");
                println!("start!, c: {}, url: {}", c, url);
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
                let ankers = self.extraction_anker(&body);
                ankers.iter().for_each(|url| {
                    if self.visited.contains(url) {
                        return;
                    } else {
                        self.visited.insert(url.clone());
                        q.push_back(url.clone());
                    }
                });
                c += 1;
            } else {
                panic!("invalid q");
            }
        }
        self.visited.clone().into_iter().collect::<Vec<_>>()
    }

    fn extraction_anker(&self, html: &str) -> Vec<Url> {
        let doc = scraper::Html::parse_document(html);
        let ankers = doc
            .select(&scraper::Selector::parse("a").unwrap())
            .collect::<Vec<_>>();
        println!("Found {} links", ankers.len());
        // println!("list: {:?}", ankers);

        ankers
            .iter()
            .filter_map(|a| a.value().attr("href"))
            .map(|s| {
                Url::parse(s).unwrap_or_else(|err| {
                    // println!("Failed to parse URL: {}, err: {}", s, err);
                    let new_base = self.generate_url_from_path(s);
                    // println!("new base URL: {}", new_base);
                    new_base
                })
            })
            .collect::<Vec<_>>()
    }

    fn generate_url_from_path(&self, path: &str) -> Url {
        let scheme = self.base_url.scheme();
        let host = self.base_url.host_str().unwrap_or("");
        let new_base = format!("{}://{}{}", scheme, host, path);
        Url::parse(new_base.as_str()).unwrap_or_else(|err| {
            // println!("Failed to parse new URL: {}, err: {}", new_base, err);
            // 元のURLを返す、visited なのでスキップされる
            Url::parse(self.base_url.as_str()).unwrap()
        })
    }
}
