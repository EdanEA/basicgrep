pub mod config;

use config::Config;
use regex::Regex;
use std::process;
use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.piped {
        let results = if config.regex {
            regex_search(&config.pattern, &config.piped_text)
        } else if config.case_sensitive {
            search(&config.pattern, &config.piped_text)
        } else {
            search_case_insensitive(&config.pattern, &config.piped_text)
        };

        if !config.count {
            for line in results {
                println!("{}", line);
            }
        } else {
            println!("{}", results.len());
        }
    }

    for (i, filename) in config.filenames.iter().enumerate() {
        let contents = fs::read_to_string(filename.clone())?;

        let results = if config.regex {
            regex_search(&config.pattern, &contents)
        } else if config.case_sensitive {
            search(&config.pattern, &contents)
        } else {
            search_case_insensitive(&config.pattern, &contents)
        };

        if config.filenames.len() > 1 {
            println!("{}:", filename);
        }

        if !config.count {
            for line in results {
                println!("{}", line);
            }
        } else {
            println!("{}", results.len());
        }

        if i != config.filenames.len() - 1 {
            println!();
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut output = Vec::new();

    for line in contents.lines() {
        if line.contains(query) && !line.is_empty() {
            output.push(line);
        }
    }

    output
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut output = Vec::new();
    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) && !line.is_empty() {
            output.push(line);
        }
    }

    output
}

pub fn regex_search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let re = Regex::new(pattern).unwrap_or_else(|err| {
        println!("Your Regex pattern is not valid or this is an error.");
        eprintln!("{}", err);
        process::exit(1);
    });

    let mut out: Vec<&str> = vec![];

    for line in contents.lines() {
        let capt = re.captures_iter(line);

        capt.for_each(|v| {
            for val in v.iter() {
                match val {
                    Some(m) => {
                        if !m.is_empty() {
                            out.push(m.as_str());
                        }
                    }
                    None => {}
                }
            }
        });
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn start_token_test() {
        let pattern = "^[a,e,i,o,u].*";
        let contents = "\
actually, I dunno
wowzers! thanks, bro
entitled, affirmatively;
amerliorate your own issues
not unless..
";

        assert_eq!(
            vec![
                "actually, I dunno",
                "entitled, affirmatively;",
                "amerliorate your own issues",
            ],
            regex_search(pattern, contents)
        );
    }

    #[test]
    fn numerical_set_test() {
        let pattern = ".*[0-9]{3}-?[0-9]{3}-?[0-9]{4}";
        let content = "\
+1 123-456-7890
+12 (123) 456-7890
1234567890
123 456 7890
(123) 456 7890
(123) 456-7890
123-456-7890
0000000000 1112223344
";

        assert_eq!(
            vec![
                "+1 123-456-7890",
                "1234567890",
                "123-456-7890",
                "0000000000 1112223344"
            ],
            regex_search(pattern, content)
        );
    }
}
