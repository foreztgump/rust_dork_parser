#[derive(Default, Debug)]
pub struct Configuration {
    /// Print page visited on standart output
    pub verbose: bool,
    /// Polite crawling delay in milli seconds
    pub delay: u64,
    /// How many request can be run simultaneously
    pub concurrency: usize,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            delay: 250,
            concurrency: 4,
            ..Default::default()
        }
    }
}