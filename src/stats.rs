#[derive(Debug, Default)]
pub struct Stats {
    pub words: usize,
    pub typos: usize,
}

impl Stats {
    pub fn new() -> Self {
        Self::default()
    }
}
