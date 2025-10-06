use url::Url;

pub enum Position {
    Header,
    Body,
    Footer,
}

pub struct ScoredUrl {
    pub original_url: Url,
    pub url: Url,
    pub score: Option<u32>,
    // pub position: Position,
}

impl ScoredUrl {
    pub fn new(original_url: Url, url: Url) -> Self {
        Self {
            original_url,
            url,
            score: None,
        }
    }

    pub fn get_score(&self) -> u32 {
        if self.score.is_none() {
            return 0;
        } else {
            return self.score.unwrap();
        }
    }

    pub fn calc_score(&self) -> Self {
        let original_host = self.original_url.host_str().unwrap_or("");
        let url_host = self.url.host_str().unwrap_or("");
        let score = if original_host == url_host {
            10
        } else if url_host.ends_with(original_host) {
            5
        } else {
            1
        };
        Self {
            original_url: self.original_url.clone(),
            url: self.url.clone(),
            score: Some(score),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_calc_score() {
        let original_url = Url::parse("https://example.com").unwrap();
        let url_same = Url::parse("https://example.com/page").unwrap();
        let url_subdomain = Url::parse("https://sub.example.com").unwrap();
        let url_different = Url::parse("https://different.com").unwrap();

        let scored_same = ScoredUrl::new(original_url.clone(), url_same).calc_score();
        assert_eq!(scored_same.score, Some(10));

        let scored_subdomain = ScoredUrl::new(original_url.clone(), url_subdomain).calc_score();
        assert_eq!(scored_subdomain.score, Some(5));

        let scored_different = ScoredUrl::new(original_url, url_different).calc_score();
        assert_eq!(scored_different.score, Some(1));
    }
}
