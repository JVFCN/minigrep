use colored::*;
use minigrep::*;
use std::env;
use std::process::exit;
use std::time::Instant;
use structopt::StructOpt;

fn main() {
    let args = Config::from_args();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("程序错误:{err}");
        exit(1);
    });

    println!("搜索内容:{}", config.query.purple());
    println!("文件路径:{}", config.file_path.cyan());

    let start = Instant::now();
    if let Err(e) = run(config) {
        println!("程序错误:{e}");
        exit(1)
    }
    let use_time = start.elapsed();
    println!("搜索用时:{:?}", use_time);
}
