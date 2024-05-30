#[derive(Debug)]
pub struct HighScores<'a>(&'a [u32]);

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        return HighScores(scores);
    }

    pub fn scores(&self) -> &[u32] {
        return &self.0;
    }

    pub fn latest(&self) -> Option<u32> {
        return self
            .scores()
            .last()
            .copied();
    }

    pub fn personal_best(&self) -> Option<u32> {
        return self
            .scores()
            .iter()
            .copied()
            .max();
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.scores().to_vec();
        scores.sort_unstable();

        return scores
            .iter()
            .rev()
            .take(3)
            .copied()
            .collect();
    }
}
