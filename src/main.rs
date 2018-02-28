extern crate reqwest;

use std::env;

const API: &str = "https://www.gitignore.io/api/";
const LIST: &str = "list";

const LINE_LENGTH: usize = 80;
const PAD: &str = "  ";

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let _cmd = &args[0];
    let types = &args[1..];

    if types.len() == 0 || &types[0] == "list" {
        println!("Available gitignore types:");
        get_types();
        return;
    }

    get_gitignore(types);
}

/// Prints a .gitignore config for the given types.
fn get_gitignore(types: &[String]) {
    let mut url = API.to_owned();
    url.push_str(&types.join(","));

    let result = reqwest::get(&url);
    if result.is_err() {
        println!("Unable to get url {}", url);
        return;
    }

    let _ = std::io::copy(&mut result.unwrap(), &mut std::io::stdout());
}

/// Prints a list of the available gitignore types.
fn get_types() {
    let mut url = API.to_owned();
    url.push_str(LIST);

    let result = reqwest::get(&url);
    if result.is_err() {
        println!("Unable to get url {}", url);
        return;
    }

    let mut response = result.unwrap();
    let s = response.text().unwrap();

    let list: Vec<&str> = s.split(&[',', '\n'][..]).collect();

    let mut ret = String::new();
    {
        let pad_len = PAD.len();

        let mut l = 0;
        for t in list {
            if l > 0 {
                ret.push_str(PAD);
                l += pad_len;
            }

            if l + t.len() > LINE_LENGTH {
                ret.push('\n');
                l = 0;
            }

            ret.push_str(t);
            l += t.len();
        }
    }
    println!("{}", ret);
}
