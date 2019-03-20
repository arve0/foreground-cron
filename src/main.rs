// limit error_chain recursion
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
mod errors {
    error_chain! {
        foreign_links {
            Io(::std::io::Error);
        }

        errors {
            ParseCrontabError(msg: String) {
                display("Crontab parsing failed: {}", msg)
            }
        }
    }
}
use crate::errors::*;

extern crate job_scheduler;

use std::fs;
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;
mod utils;
use crate::utils::parse_crontab_line;

quick_main!(run);

fn run() -> Result<()> {
    let crontab = fs::read_to_string("crontab").chain_err(|| "Unable to open crontab file.")?;

    let mut scheduler = JobScheduler::new();

    for line in crontab.lines() {
        if is_empty_or_commented(&line) {
            continue;
        }

        let (schedule, mut cmd) = parse_crontab_line(&line)?;
        let job = Job::new(schedule, move || {
            match cmd.status() {
                Ok(exit_code) => {
                    println!("{}", exit_code);
                }
                Err(msg) => {
                    println!("{}", msg);
                }
            };
        });

        scheduler.add(job);
    }

    loop {
        scheduler.tick();
        std::thread::sleep(Duration::from_millis(500));
    }

    Ok(())
}

fn is_empty_or_commented(line: &str) -> bool {
    line.len() == 0 || line.chars().next() == Some('#')
}