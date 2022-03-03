use crate::libs::jobs::Dork;

mod libs;

fn main() {
    let mut dork: Dork = Dork::new("dorks.txt".to_string());
    dork.configuration.verbose = false;
    dork.configuration.delay = 2000;
    dork.configuration.concurrency = 4;
    dork.analyzer();
}