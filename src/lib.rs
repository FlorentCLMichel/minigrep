//! # minigrep 
//!
//! A simple implementation of a grep-like command
//! 
//! ## Aim of this small project
//!
//! The main aim of this project is to learn a bit of Rust. 
//! I do not aim at making an efficient implementation of the `grep` command—they already exist!
//!
//! ## Use
//!
//!     minigrep query filename [style]
//! 
//! * `query` (String): the string to search for
//! * `filename` (String): the name of the file to search in
//! * `style` (Integer, optional): the style the query is to be printed with
//! 
//! ## Behaviour
//! 
//! Print all lines in the file `filename` containing the string `query`. 
//! 
//! If the environment variable `CASE_INSENSITIVE` is set, the search is performed in a
//! case-insensitive way.
//!
//! ## Example 
//!
//!     minigrep you poem.txt 1

use std::fs;
use std::env;
use std::error::Error;

mod style;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    // read the file
    let contents = read_file(config.filename)?;

    // select the lines that contain the query
    let lines_with_query = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // print the result
    if config.style > 0 {
        for n_line in lines_with_query {
            println!("{}", format(&contents[n_line], &config.query, config.style));
        }
    } else {
        for n_line in lines_with_query {
            println!("{}", &contents[n_line]);
        }
    }

    Ok(())
}


#[derive(Debug, PartialEq)]
pub struct Config {
    query: String,
    filename: String,
    style: u8,
    case_sensitive: bool,
}


impl Config {
    /// Create a new Config from an array of arguments
    /// 
    /// # Argument
    ///
    /// `args`: array of `String` with at least 3 elements
    ///
    /// # Errors
    ///
    /// * `Not enough arguments` if the number of arguments is smaller than 2
    ///
    /// # Warnings
    ///
    /// * `Too many arguments` if the number of arguments is larger than 3
    ///
    /// # Values
    ///
    /// * `query` and `filemane` are given by the first two arguments.
    /// * If there is a third argument, it is conerted to a `u8` and set to `style`. If not, 
    /// `style` takes the value 0.
    /// * `case_sensitive` is set to `true` if the environment variable `CASE_INSENSITIVE` is not 
    /// set and to `false` if it is set.
    pub fn new(mut args: env::Args) -> Result<Config, String> {
    
        // read the arguments
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err(style::add_fg("Missing the first argument (query)".to_string(), 
                                             255, 0, 0))
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err(style::add_fg("Missing the second argument (filename)".to_string(), 
                                             255, 0, 0))
        };
        let style: u8 = match args.next() {
            Some(arg) => {
                let mut s = 0;
                match arg.parse::<u8>() {
                    Ok(x) => s = x,
                    Err(_) => {
                        eprintln!("{}",
                            style::add_fg("WARNING: Could not parse the third argument (style) as a u8"
                                          .to_string(), 
                                          255, 255, 0));
                    }
                };
                match args.next() {
                    Some(_) => eprintln!("{}",
                                style::add_fg(
                                    "WARNING: Too many arguments; the 4th one and up will be discarded"
                                    .to_string(), 
                                    255, 255, 0)),
                    None => ()
                };
                s
            },
            None => 0
        };

        // set the case_sensitive value
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, style, case_sensitive })
    }
}


/// Read the content of a file as a vector of strings, each element being a line in the file
///
/// # Errors
///
/// * `Could not open the file` if the file can not be opened
fn read_file(filename: String) -> Result<Vec<String>, String> {
    let content = match fs::read_to_string(&filename) {
        Ok(f) => f,
        Err(_) => {
            let err_message = style::add_fg(
                format!("Could not open the file {}", &filename), 
                255, 0, 0
            );
            return Err(err_message);
        }
    };
    let lines = content.split("\n").collect::<Vec<&str>>();
    let mut res = Vec::<String>::new();
    for line in lines {
        res.push(line.to_string());
    }
    Ok(res)
}

/// Format a string to highlight each occurrence of a word
///
/// # Examples
///
/// ```
/// use minigrep::format; 
///
/// let line = "This is a fine sentence!";
/// let word = "fine";
/// let style = 2;
/// 
/// let formatted_line = format(line, word, style);
///
/// assert_eq!("This is a \x1b[2;1mfine\x1b[0m sentence!".to_string(), 
///            formatted_line)
/// ```
pub fn format(line: &str, word: &str, style: u8) -> String {
    let line_s = &line.to_string();
    let word_format =  style::add_style(word.to_string(), style);
    str::replace(line_s, &word, &word_format)
}


/// Select the indices of the strings containing the query
fn search(query: &String, contents: &Vec<String>) -> Vec<usize>{
    let mut res = Vec::<usize>::new();
    for i in 0..contents.len() {
        if contents[i].contains(query) {
            res.push(i);
        }
    }
    res
}


/// Select the indices of the strings containing the query, case-insensitive
fn search_case_insensitive(query: &String, contents: &Vec<String>) -> Vec<usize>{
    let mut res = Vec::<usize>::new();
    let query = query.to_lowercase();
    for i in 0..contents.len() {
        if contents[i].to_lowercase().contains(&query) {
            res.push(i);
        }
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_1() {
        let sentence = "I love blue cheese!";
        let word = "love";
        let style: u8 = 1;
        let sentence_highlighted = format(sentence, word, 1);
        let expected_result = "I \x1b[1;1mlove\x1b[0m blue cheese!".to_string();
        assert_eq!(expected_result, sentence_highlighted);
    }

    #[test]
    fn search_1() {
        let query = "duct".to_string();
        let contents = vec!["Rust:".to_string(),
                            "safe, fast, productive.".to_string(), 
                            "Pick three.".to_string(),
                            "‘Ductape’ is a typo".to_string()];
        assert_eq!(vec![1], search(&query, &contents));
    }

    #[test]
    fn search_2() {
        let query = "duct".to_string();
        let contents = vec!["Rust:".to_string(),
                            "safe, fast, productive.".to_string(), 
                            "Pick three.".to_string(),
                            "‘Ductape’ is a typo".to_string()];
        assert_eq!(vec![1,3], search_case_insensitive(&query, &contents));
    }
}
