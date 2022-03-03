use crate::libs::utils::{AgentInfo, blacklist_sites, between, before};

use ureq::Error;
use ::url::{Url};
use serde_json::Value;
use std::time::Duration;
use std::collections::HashSet;
use serde::{Deserialize, Serialize};

use super::rand::{thread_rng, Rng};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QueryRequest {
    query: String,
    retried: i32,
    response_error: String,
    transport_error: String,
    parse_json_error: String,
    result_urls: HashSet<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response {
    //query: String,
    //number_of_results: String,
    //searchInformation: SearchInformation,
    //items: Vec<Items>,
    //origin: String,
    //results: Vec<Results>,
    results: Vec<unescapedUrl>,
    cursor: estimatedResultCount,
}

// #[allow(non_snake_case)]
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct SearchInformation {
//     totalResults: String,
// }
//
// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Items {
//     link: String,
// }
//
// impl Items {
//     pub fn get_url(&self) -> String {
//         self.link.clone()
//     }
// }

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct estimatedResultCount {
    estimatedResultCount: String,
}

#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct unescapedUrl {
    unescapedUrl: String,
}
impl unescapedUrl {
    fn get_url(&self) -> String {
        self.unescapedUrl.clone()
    }
}

// #[derive(Serialize, Debug, Deserialize, Clone)]
// pub struct Results {
//     url: String,
// }
//
// impl Results {
//     pub fn get_url(&self) -> String {
//         self.url.clone()
//     }
// }

#[allow(non_camel_case_types)]
#[derive(Serialize, Debug, Deserialize)]
pub struct url {
    url: String,
}

impl QueryRequest {
    #[allow(non_snake_case)]
    #[allow(unused_assignments)]
    pub fn new(query_dork: String) -> Self {
        let retries = 4;
        let mut counter = 1;
        let mut v_links = vec![String::new()];
        let mut query_result = vec![String::new()];
        let mut call_break = true & false | false;
        let mut transport_error_happened = String::new();
        let mut response_error_happened = String::new();
        let mut parse_error_happened = String::new();
        let mut page_to_parse = 6;
        let mut current_page = 1;
        let mut new_ip = String::new();
        let mut current_ip = String::new();
        let mut cse_token = String::new();
        let mut cse_libv = String::new();
        let mut change_ip = true & false | false;
        let mut got_new_ip = true & false | false;
        let mut got_token = true & false | false;


        while counter < retries {
            //println!("counter: {}", counter);

            //get useragent_info to build agent for ureq
            let agent_info = AgentInfo::new();
            let proxy = agent_info.get_proxy();
            let user_agent_use = agent_info.get_user_agent();
            let google_api_string: Vec<String> = agent_info.get_google_api();

            //println!("{}", proxy);

            //set proxy and build agent for ureq
            let proxy = ureq::Proxy::new(proxy.as_str()).unwrap();
            let agent = ureq::AgentBuilder::new()
                .proxy(proxy)
                .user_agent(user_agent_use.as_str())
                .timeout(Duration::from_secs(30))
                .build();

            for x in 1..page_to_parse {
                //println!("request page # {}", x);

                //-------------------------------Change IP------------------------------------------
                //Check IP and get new ip from proxy
                if change_ip {
                    match agent.get("http://ip-api.com/json/").call() {
                        Ok(response) => {
                            match response.into_json() {
                                Ok(res) => {
                                    let v: Value = res;
                                    //let v: Value = serde_json::from_str(&*res).expect("can't parse");
                                    new_ip = v["query"].to_string();
                                    got_token = false;
                                    got_new_ip = true;
                                    change_ip = false;
                                    //println!("new ip : {}", new_ip);
                                },
                                Err(_e) => {
                                    //println!("p");
                                    parse_error_happened = "Failed to read JSON".to_string();
                                    change_ip = true;
                                    break;
                                },
                            }
                        },
                        Err(Error::Status(_code, _response)) => {
                            //println!("r");
                            change_ip = true; break;},
                        Err(Error::Transport(_transport)) => {
                            //println!("t");
                            change_ip = true; break;}
                    }
                }

                //Check if IP the same
                if got_new_ip && new_ip == current_ip {
                    got_new_ip = false;
                    change_ip = true;
                    break;
                } else if !change_ip {
                    current_ip = new_ip.clone();
                    got_new_ip = false;
                }
                //----------------------------------------------------------------------------------

                //-------------------------------Get Token------------------------------------------
                if !got_token {
                    let get_token_link = format!("https://cse.google.com/cse.js?hpg=1&cx={}",
                                                 google_api_string[0]);
                    match agent.get(&*get_token_link).call() {
                        Ok(response) => {
                            match response.into_string() {
                                Ok(res) => {
                                    let cse_token_string = between(&*res, "\"cse_token\": \"", "\",");
                                    let real_token = before(cse_token_string, "\"");
                                    let cselibVersion = between(&*res, "cselibVersion\": \"", "\",");
                                    cse_token = real_token.parse().unwrap();
                                    cse_libv = cselibVersion.parse().unwrap();
                                    got_token = true;
                                },
                                Err(_e) => {
                                    //println!("p");
                                    parse_error_happened = "Failed to read token from string".to_string();
                                    break;
                                },
                            }
                        },
                        Err(Error::Status(_code, _response)) => {
                            //println!("r");
                            change_ip = true;
                            break;},
                        Err(Error::Transport(_transport)) => {
                            //println!("t");
                            change_ip = true;
                            break;}
                    }
                }
                //----------------------------------------------------------------------------------

                //stop thread when parse all pages
                if x+1 == page_to_parse  {
                    //println!("Call Break");
                    call_break = true;
                    break;
                }

                //make link to for each dork
                let mut rng = thread_rng();
                let api_number = rng.gen_range(1000..9999);
                //Get current page
                if x == 2 {
                    current_page = 21;
                } else if x == 3 {
                    current_page = 41;
                } else if x == 4 {
                    current_page = 61;
                } else if x == 5 {
                    current_page = 81;
                }
                //println!("{} {}", x, current_page);
                let request = format!("https://cse.google.com/cse/element/v1?rsz=filtered_cse&num=20&start={}&hl=en&source=gcsc&gss=.com&cselibv={}&cx={}&q={}&safe=off&cse_tok={}&sort=&exp=csqr,cc&callback=google.search.cse.api{}"
                                      , current_page, cse_libv, google_api_string[0], query_dork, cse_token, api_number);

                //println!("{}", request);

                //Send GET request for dork query
                match agent.get(&*request).call() {
                    //Check if get any response
                    Ok(response) => {
                        match response.into_string(){
                            Ok(response) => {
                                //println!("{}", response);
                                //turn json response to struct variable
                                //let v: Response = response;
                                //let v: Response = serde_json::from_str(&*response).expect("Can't parse line");
                                //println!("results count : {:?}", v.results.len());

                                //Check if there's any results from search
                                if response.contains("estimatedResultCount") {
                                    let to_json_string = between(&*response, "(", ");" );
                                    let v =  serde_json::from_str(to_json_string);
                                    match v {
                                        Ok(v) => {
                                            let v: Response = v;
                                            //parse total_result
                                            let total_result = v.cursor.estimatedResultCount
                                                .parse::<i64>()
                                                .expect("Can't parse total value");
                                            if  total_result < 60 {
                                                //check how many page to parse
                                                if total_result < 20 {
                                                    page_to_parse = 1;
                                                } else if total_result < 40 {
                                                    page_to_parse = 2;
                                                } else if total_result < 60 {
                                                    page_to_parse = 3;
                                                }
                                                page_to_parse = 4;
                                            }

                                            //Parse links from response
                                            for elem in v.results.iter() {
                                                query_result.push((&*elem.get_url()).parse().unwrap());
                                                let link = Url::parse(&*elem.get_url()).unwrap();
                                                let query = link.query();
                                                //println!("{:?}", elem);
                                                //check if hostname is blacklist
                                                match link.host() {
                                                    Some(t) => {
                                                        let hostname = t.to_string();
                                                        if !blacklist_sites().contains(&hostname) {
                                                            //check if link has query string
                                                            match query {
                                                                Some(_t) => {
                                                                    //println!("{:?}", query);
                                                                    //push url to vector to collect
                                                                    v_links.push(String::from(link));
                                                                },
                                                                None => {continue}
                                                            }
                                                        }
                                                    }
                                                    None => {continue}
                                                }
                                            }
                                        },
                                        Err(e) => {
                                            counter += 1;
                                            //println!("e_g_json");
                                            got_token = false;
                                            parse_error_happened = e.to_string();
                                            break;},
                                    };
                                } else {
                                    //if no results from the first search then stop
                                    //println!("No links from search");
                                    call_break = true;
                                    break;
                                }
                            },
                            Err(_e) => {
                                counter += 1;
                                got_token = false;
                                //println!("e_g_string");
                                parse_error_happened = "Failed to read string from query".to_string();
                                break;
                            },
                        }
                    },
                    //Break if there's any error and restart
                    Err(Error::Status(code, _response)) => {
                        counter += 1;
                        got_token = false;
                        //println!("{}", code);
                        response_error_happened = code.to_string();
                        change_ip = true;
                        break;
                    },
                    Err(Error::Transport(transport)) => {
                        counter += 1;
                        got_token = false;
                        //println!("{}", transport);
                        transport_error_happened = transport.to_string();
                        change_ip = true;
                        break;
                    }
                }
            }

            //proxy_use_result = proxy;
            //url_use_result = request;
            if call_break {
                break;
            }
        }
        //print result links (test!)
        //println!("results query link count : {:?}", v_links.len());

        //push links to hashset
        let result_urls_hash: HashSet<String> = v_links.into_iter().collect();


        Self {
            query: query_dork,
            retried: counter,
            parse_json_error: parse_error_happened,
            response_error: response_error_happened,
            transport_error: transport_error_happened,
            result_urls: result_urls_hash,
        }
    }

    pub fn get_query(&self) -> String { self.query.clone() }
    pub fn get_result_urls(&self) -> HashSet<String> { self.result_urls.clone() }
    pub fn get_retried(&self) -> i32 { self.retried.clone() }
    pub fn get_parse_json_error(&self) -> String { self.parse_json_error.clone() }
    pub fn get_response_error(&self) -> String { self.response_error.clone() }
    pub fn get_transport_error(&self) -> String { self.transport_error.clone() }
}