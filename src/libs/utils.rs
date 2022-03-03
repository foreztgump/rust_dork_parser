use std::fs::File;
use std::{env, io};
use std::path::{Path, PathBuf};
use std::net::{ToSocketAddrs, TcpStream};
use std::io::{BufRead, BufReader};
use rand::{seq::SliceRandom, distributions::Alphanumeric, thread_rng};


use super::rand::Rng;


pub fn inner_main() -> io::Result<PathBuf> {
    let exe = env::current_exe()?;
    let dir = exe.parent().expect("Executable must be in some directory");
    let dir = dir.join("resources");
    Ok(dir)
}

#[derive(Clone, Debug)]
pub struct FileHandler {
    public_path: String,
}

impl FileHandler {
    pub fn read_file(file_name: &str) -> Self {
        let mut dir = inner_main().expect("Couldn't find path");
        dir.push(file_name);
        Self {
            public_path: dir.display().to_string(),
        }
    }

    pub fn get_path(&self) -> String {
        self.public_path.clone()
    }
}

#[derive(Debug, Clone)]
pub struct AgentInfo {
    user_agent: String,
    proxy: String,
    google_api: Vec<String>,
}

impl AgentInfo {
    pub fn new() -> Self{
        let user_agent_list: Vec<String> = user_agents();
        let proxy = proxy_cru_gen();
        //let proxy_list: Vec<String> = proxy_list();
        let google_api_list: Vec<String> = google_api();
        let google_api = random_google_api(&google_api_list);
        //let proxy = random_proxy(&proxy_list);
        let user_agent = random_user_agent(&user_agent_list);

        Self {
            user_agent,
            proxy,
            google_api,
        }

    }

    #[allow(dead_code)]
    pub fn get_proxy(&self) -> String {
        self.proxy.clone()
    }

    pub fn get_user_agent(&self) -> String {
        self.user_agent.clone()
    }

    pub fn get_google_api(&self) -> Vec<String> {
        self.google_api.clone()
    }

}

pub fn between<'value>(value: &'value str, a: &str, b: &str) -> &'value str {
    // Find the two strings.
    if let Some(pos_a) = value.find(a) {
        if let Some(pos_b) = value.rfind(b) {
            // Return the part in between the 2 strings.
            let adjusted_pos_a = &pos_a + a.len();
            if adjusted_pos_a < pos_b {
                return &value[adjusted_pos_a..pos_b];
            }
        }
    }
    return "";
}

pub fn before<'value>(value: &'value str, a: &str) -> &'value str {
    // Find the string and return the part before.
    if let Some(pos_a) = value.find(a) {
        return &value[0..pos_a];
    }
    return "";
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .collect()
}

pub fn random_user_agent(user_agent: &[String]) -> String {
    user_agent.choose(&mut thread_rng()).unwrap().to_string()
}

pub fn random_google_api(google_api: &[String]) -> Vec<String> {
    let string = google_api.choose(&mut thread_rng()).unwrap().to_string();
    let vec_google_api:Vec<String> = string.split(":").map(|s| s.to_string()).collect();
    vec_google_api
}

pub fn check_socks_proxy<A: ToSocketAddrs>(address: A) -> bool{
    TcpStream::connect(address).is_ok()
}

pub fn proxy_cru_gen() -> String {
    let rand_user = rand_string_gen();
    let rand_pass = rand_string_gen();
    let proxy = format!("socks5://{}:{}@127.0.0.1:9051", rand_user, rand_pass);
    proxy
}

pub fn rand_string_gen() -> String {
    let rand_user: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
    rand_user
}

pub fn user_agents() -> Vec<String> {
    vec!["Mozilla/5.0 (Windows NT 6.1; WOW64; rv:77.0) Gecko/20190101 Firefox/77.0".to_string(),
         "Mozilla/5.0 (Windows NT 10.0; WOW64; rv:77.0) Gecko/20100101 Firefox/77.0".to_string(),
         "Mozilla/5.0 (X11; Linux ppc64le; rv:75.0) Gecko/20100101 Firefox/75.0".to_string(),
         "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:39.0) Gecko/20100101 Firefox/75.0".to_string(),
         "Mozilla/5.0 (Macintosh; U; Intel Mac OS X 10.10; rv:75.0) Gecko/20100101 Firefox/75.0".to_string(),
         "Mozilla/5.0 (X11; Linux; rv:74.0) Gecko/20100101 Firefox/74.0".to_string(),
         "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.13; rv:61.0) Gecko/20100101 Firefox/73.0".to_string(),
         "Mozilla/5.0 (X11; OpenBSD i386; rv:72.0) Gecko/20100101 Firefox/72.0".to_string(),
         "Mozilla/5.0 (Windows NT 6.3; WOW64; rv:71.0) Gecko/20100101 Firefox/71.0".to_string(),
         "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:70.0) Gecko/20191022 Firefox/70.0".to_string(),
         "Mozilla/5.0 (Windows NT 6.1; WOW64; rv:70.0) Gecko/20190101 Firefox/70.0".to_string(),
         "Mozilla/5.0 (Windows; U; Windows NT 9.1; en-US; rv:12.9.1.11) Gecko/20100821 Firefox/70".to_string(),
         "Mozilla/5.0 (Windows NT 10.0; WOW64; rv:69.2.1) Gecko/20100101 Firefox/69.2".to_string(),
         "Mozilla/5.0 (Windows NT 6.1; rv:68.7) Gecko/20100101 Firefox/68.7".to_string(),
         "Mozilla/5.0 (X11; Linux i686; rv:64.0) Gecko/20100101 Firefox/64.0".to_string(),
         "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.102 Safari/537.36 Edge/18.19582".to_string(),
         "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/70.0.3538.102 Safari/537.36 Edge/18.19577".to_string(),
         "Mozilla/5.0 (X11) AppleWebKit/62.41 (KHTML, like Gecko) Edge/17.10859 Safari/452.6".to_string(),
         "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML like Gecko) Chrome/51.0.2704.79 Safari/537.36 Edge/14.14931".to_string(),
         "Chrome (AppleWebKit/537.1; Chrome50.0; Windows NT 6.3) AppleWebKit/537.36 (KHTML like Gecko) Chrome/51.0.2704.79 Safari/537.36 Edge/14.14393".to_string(),
         "Mozilla/5.0 (Windows NT 6.2; WOW64) AppleWebKit/537.36 (KHTML like Gecko) Chrome/46.0.2486.0 Safari/537.36 Edge/13.9200".to_string(),
         "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML like Gecko) Chrome/46.0.2486.0 Safari/537.36 Edge/13.10586".to_string(),
         "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/42.0.2311.135 Safari/537.36 Edge/12.246".to_string(),
         "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.93 Safari/537.36".to_string(),
         "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.93 Safari/537.36".to_string(),
         "Mozilla/5.0 (Windows NT 10.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.93 Safari/537.36".to_string(),
         "Mozilla/5.0 (Macintosh; Intel Mac OS X 11_3_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.93 Safari/537.36".to_string(),
         "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.93 Safari/537.36".to_string(),
    ]
}

pub fn blacklist_sites() -> Vec<String> {
    vec!["www.youtube.com".to_string(),
         "www.facebook.com".to_string(),
         "www.pinterest.com".to_string(),
         "www.google.com".to_string(),
         "www.twitter.com".to_string(),
         "www.instagram.com".to_string(),
         "chrome.google.com".to_string(),
         "www.quora.com".to_string(),
         "play.google.com".to_string(),
         "github.com".to_string(),
         "www.amazon.com".to_string(),
         "itunes.apple.com".to_string(),
         "support.google.com".to_string(),
         "twitter.com".to_string(),
         "music.youtube.com".to_string(),
    ]
}

//Need to provide google_api
pub fn google_api() -> Vec<String> {
    vec!["8c3b7407385cfae28:AIzaSyBi2H3Y8ABsmg6assEubh4kf7q_e-b-Fs3".to_string(),
         "ec0a23b5c6b3395d3:AIzaSyCHxBTJARS77DLgUpT4gB9P43Mj6A2HCRE".to_string(),
    ]
}