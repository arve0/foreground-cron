extern crate job_scheduler;

use crate::errors::*;
use job_scheduler::Schedule;
use std::process::Command;

#[cfg(test)]
mod parse_crontab_tests {
    use super::*;

    #[test]
    fn parses_crontab_line() {
        parse_crontab_line("* * * * * cmd").unwrap();
    }
}

pub fn parse_crontab_line(line: &str) -> Result<(Schedule, Command)> {
    let divided: Vec<_> = line.split_whitespace().collect();

    if divided.len() >= 6 {
        // keep original crontab pattern, add seconds first and years last
        let time = format!("0 {} *", divided[0..5].join(" "));
        let schedule = time.parse()
            .chain_err(|| ErrorKind::ParseCrontabError(format!("Unable to parse schedule: {}", time)))?;

        let mut command = Command::new("sh");

        let args = vec![String::from("-c"), divided[5..].join(" ")];
        command.args(args.iter());

        Ok((schedule, command))
    } else {
        Err(Error::from(format!("Expected to find 6 arguments in line '{}', found {}.", line, divided.len())))
    }
}