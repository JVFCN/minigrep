use std::error::Error;
use std::{env, fs};
use colored::Colorize;
use std::collections::BTreeMap;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "MiniGrep", about = "模仿Linux的Grep")]
pub struct Config {
    #[structopt(short, long, help = "输入要查询的字符串")]
    pub query: String,
    #[structopt(short, long = "file", help = "输入一个文件路径")]
    pub file_path: String,
    #[structopt(short, long, help = "是否输出行信息")]
    pub output: bool
}

impl Config {
    pub fn build(args_: &Config) -> Result<Config, &'static str> {
        Ok(
            Config {
                query: args_.query.clone(),
                file_path: args_.file_path.clone(),
                output: args_.output.clone()
            })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let binding = contents.to_string();
    let result_s = search(&config.query.to_string()[..], &binding[..]);
    if config.output {
        for (num, line) in &result_s {
            println!("{}\t{}", "[".to_owned() + &num.to_string() + "]", highlight_keywords(&line.trim(), &config.query));
        }
    }
    println!("共{}个结果", result_s.len());
    Ok(())
}

fn highlight_keywords(line: &str, keywords: &str) -> String {
    let colored_keywords = keywords
        .split_whitespace()
        .map(|keyword| keyword.yellow().underline().to_string())
        .collect::<Vec<_>>()
        .join(" ");

    line.replace(keywords, &colored_keywords)
}

pub fn search<'a>(query: &str, contents: &'a str) -> BTreeMap<u32, &'a str> {
    // let mut results  = Vec::new();
    let mut results = BTreeMap::new();
    let mut line_num: u32 = 0;
    for line in contents.lines() {
        line_num += 1;
        if line.contains(query) {
            results.insert(line_num, line);
        }
    }
    results
}
