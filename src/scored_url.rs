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
        let mut score = self.calc_score_from_domain();
        score += self.calc_score_from_path();
        Self {
            original_url: self.original_url.clone(),
            url: self.url.clone(),
            score: Some(score),
        }
    }

    fn calc_score_from_domain(&self) -> u32 {
        let original_host = self.original_url.host_str().unwrap_or("");
        let url_host = self.url.host_str().unwrap_or("");
        if original_host == url_host {
            10
        } else if url_host.ends_with(original_host) {
            5
        } else {
            1
        }
    }

    const PATH_SCORE_UNIT: u32 = 2;
    fn calc_score_from_path(&self) -> u32 {
        let original_pathes = self
            .original_url
            .path_segments()
            .unwrap()
            .collect::<Vec<_>>();
        let url_pathes = self.url.path_segments();
        if url_pathes.is_none() {
            return 0;
        }
        let url_pathes = url_pathes.unwrap().collect::<Vec<_>>();
        let loop_count = url_pathes.len().min(original_pathes.len());
        let mut c = 0;
        for i in 0..loop_count {
            let o_seg = original_pathes[i];
            let u_seg = url_pathes[i];
            if o_seg != u_seg {
                return c * Self::PATH_SCORE_UNIT;
            }
            c += 1;
        }
        c * Self::PATH_SCORE_UNIT
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_calc_score() {
        let original_url = Url::parse("https://example.com/a").unwrap();
        let url_same = Url::parse("https://example.com/page").unwrap();
        let url_subdomain = Url::parse("https://sub.example.com").unwrap();
        let url_different = Url::parse("https://different.com").unwrap();
        let same_path = Url::parse("https://different.com/a").unwrap();

        let scored_same = ScoredUrl::new(original_url.clone(), url_same).calc_score();
        assert_eq!(scored_same.score, Some(10));

        let scored_subdomain = ScoredUrl::new(original_url.clone(), url_subdomain).calc_score();
        assert_eq!(scored_subdomain.score, Some(5));

        let scored_different = ScoredUrl::new(original_url.clone(), url_different).calc_score();
        assert_eq!(scored_different.score, Some(1));

        let scored_different = ScoredUrl::new(original_url, same_path).calc_score();
        assert_eq!(scored_different.score, Some(1 + 2));
    }
}
