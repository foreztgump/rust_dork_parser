use chrono::prelude::*;
use rayon::ThreadPoolBuilder;
use std::io::{Write, stdout};
use std::collections::HashSet;
use std::{sync, thread, time, fs};
use crossterm::{QueueableCommand, cursor};
use indicatif::{ProgressBar, ProgressStyle};


use crate::libs::request::QueryRequest;
use crate::libs::configuration::Configuration;
use crate::libs::utils::{inner_main, lines_from_file};
use crate::libs::utils::{FileHandler, check_socks_proxy};
use crossterm::terminal::{Clear, ClearType};


#[derive(Debug)]
pub struct Dork {
    pub configuration: Configuration,
    output_path: String,
    path: String,
}

impl Dork {
    pub fn new(path: String) -> Self {
        let proxy_addy = check_socks_proxy("127.0.0.1:9051");
        match proxy_addy {
            true => println!("Tor is running."),
            false => panic!("Tor is not running. Please start Tor and restart the program again.")
        }
        let local: DateTime<Local> = Local::now();
        let output_folder = inner_main().expect("Couldn't find path");
        let output_path = format!("{}/filter_links_{}.txt", output_folder.to_string_lossy() ,local.format("%Y_%m_%d_%H_%M_%S").to_string());
        let dork_path = FileHandler::read_file(path.as_str());
        //println!("{}", output_path);
        //println!("{}", dork_path.get_path());
        Self {
            configuration: Configuration::new(),
            path: dork_path.get_path(),
            output_path,
        }
    }

    #[allow(unused_must_use)]
    pub fn analyzer(&mut self) {
        let delay = time::Duration::from_millis(self.configuration.delay);
        let pool = ThreadPoolBuilder::new()
            .num_threads(self.configuration.concurrency)
            .build()
            .expect("Failed building thread pool.");

        println!("Starting with {} threads...", self.configuration.concurrency);
        let (tx, rx) = sync::mpsc::channel();
        let dorks: Vec<String> = lines_from_file(&self.path).expect("Could not read line from file");
        let dork_count = dorks.len();

        println!("Found : {} dorks", dork_count);
        //make progress bar
        let pb = ProgressBar::new(dork_count as u64);
        pb.set_style(ProgressStyle::default_bar().template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] (Dorks : {pos}/{len})",
        ));

        //create stdout for crossterm
        let mut stdout = stdout();

        for d in dorks.into_iter() {
            let dork_query: String = d.to_string();
            if self.configuration.verbose {
                //println!("Fetch query: {}", dork_query);
            }

            let tx = tx.clone();
            pool.spawn(move || {
                tx.send(QueryRequest::new(dork_query)).unwrap();
                //println!(" {:?}", thread::current().id());
                thread::sleep(delay);
            });
        };

        drop(tx);

        //let mut dork_done_count = 0;
        //let mut filter_links_count = 0;
        let mut result_hash = HashSet::new();
        let mut status_update = String::new();
        result_hash.clear();
        rx.into_iter().for_each(|query|{
            if self.configuration.verbose {
                println!("Parsed: {}", query.get_query());
                println!("Requested for : {} times", query.get_retried());
            }

            if !query.get_result_urls().is_empty() {
                for link in query.get_result_urls().iter() {
                    //filter_links_count += 1;
                    let link_to_hash = link.clone();
                    //check if its a new link
                    if !result_hash.contains(&*link_to_hash) {
                        //write link to output
                        let mut file = fs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .append(true)
                            .open(&self.output_path)
                            .unwrap();
                        write!(file, "{}\n", link_to_hash).unwrap();
                        //save link to hashset
                        result_hash.insert(link_to_hash);
                    }
                }
            }

            if query.get_response_error().is_empty()
                && query.get_transport_error().is_empty()
                && query.get_parse_json_error().is_empty() {
                status_update = format!("Total filter links : {} - - - Successfully Parsed",
                                        result_hash.len().to_string());
            } else {
                status_update = format!("Total filter links : {} - - - Encounter some errors : \
                parse error: {} response: {}, transport: {}",
                                        result_hash.len().to_string(),
                                        query.get_parse_json_error(),
                                        query.get_response_error(),
                                        query.get_transport_error());
            }
            // dork_done_count += 1;
            // println!("Dork Done: {}/{} Filter Links: {}", dork_done_count,
            //          dork_count as u64, filter_links_count);
            //add counter progress bar
            pb.inc(1);
            stdout.queue(cursor::SavePosition).expect("Can't save cursor position");
            stdout.queue(Clear(ClearType::CurrentLine));
            stdout.flush().expect("Can't flush terminal");
            stdout.write(format!("{}", status_update).as_ref()).expect("Can't write to terminal");
            stdout.queue(cursor::RestorePosition).expect("Can't restore cursor position");
            stdout.flush().expect("Can't flush terminal");
            //println!("Filtered Links : {:?}", result_hash);
            //println!("Filtered Links : {}", result_hash.len());
        });

        //clear progress bar
        //pb.finish();
        println!("\nDone");
    }
}

