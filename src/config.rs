use core::fmt;
use std::env;

pub struct Config {
    pub pattern: String,
    pub filenames: Vec<String>,
    pub case_sensitive: bool,
    pub regex: bool,
    pub piped: bool,
    pub piped_text: String,
    pub count: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let mut pattern = args[args.len() - 2].clone();
        let mut filenames = vec![];
        let mut case_sensitive = true;
        let mut regex = false;
        let mut count = false;

        if env::var("CASE_INSENSITIVE").is_ok_and(|x| x.parse::<u8>() == Ok(1)) {
            case_sensitive = false;
        }

        for i in 1..args.len() {
            if args[i] == "--no-ignore-case" {
                case_sensitive = true;
            } else if args[i] == "-e" {
                regex = true;

                if args.len() == i + 1 {
                    return Err("Please provide the regex pattern after the -e flag.");
                }

                pattern = args[i + 1].clone();

                if args.len() <= i + 2 {
                    return Err(
                        "Couldn't parse file name(s). Should be present after flags and pattern.",
                    );
                }

                for s in args.split_at(i + 2).1.iter() {
                    filenames.push(s.clone());
                }

                break;
            } else if args[i] == "--ignore-case" || args[i] == "-i" {
                case_sensitive = false;
            } else if args[i] == "-c" || args[i] == "--count" {
                count = true;
            } else {
                pattern = args[i].clone();

                for s in args.split_at(i + 1).1.iter() {
                    filenames.push(s.clone());
                }

                break;
            }
        }

        Ok(Config {
            pattern,
            filenames,
            case_sensitive,
            regex,
            piped: false,
            piped_text: String::new(),
            count,
        })
    }

    pub fn new_from_pipe(args: &[String], input: &str) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let mut pattern = args[args.len() - 1].clone();
        let mut case_sensitive = true;
        let mut regex = false;
        let mut count = false;

        if env::var("CASE_INSENSITIVE").is_ok_and(|x| x.parse::<u8>() == Ok(1)) {
            case_sensitive = false;
        }

        for (i, arg) in args.iter().enumerate() {
            if arg == "--no-ignore-case" {
                case_sensitive = true;
            } else if arg == "-e" {
                regex = true;

                if args.len() == i + 1 {
                    return Err("a regex pattern should directly follow the -e flag");
                }

                pattern = args[i + 1].clone();
            } else if arg == "--ignore-case" || arg == "-i" {
                case_sensitive = false;
            } else if arg == "-c" || arg == "--count" {
                count = true;
            }
        }

        Ok(Config {
            pattern,
            filenames: vec![],
            case_sensitive,
            regex,
            piped: true,
            piped_text: input.to_string(),
            count,
        })
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.piped {
            write!(
                f,
                "Searching for `{}` in piped text `{}`; {} case sensitive.",
                self.pattern,
                self.piped_text.replace("\n", " ").trim_end(),
                if self.case_sensitive { "is" } else { "is not" }
            )
        } else {
            write!(
                f,
                "Searching for `{}` in `{:?}`; {} case sensitive.",
                self.pattern,
                self.filenames,
                if self.case_sensitive { "is" } else { "is not" }
            )
        }
    }
}
