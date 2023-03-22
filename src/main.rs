use std::cmp::max;
use std::io::{self, BufRead};

use regex::Regex;

struct HttpLogLine {
    host: String,
    client_identity: String,
    user_id: String,
    date_time: String,
    http_method: String,
    requested_url: String,
    http_protocol_version: String,
    http_status_code: i32,
    response_body_size: i32,
    referrer_url: String,
    agent: String,
    code: i32,
    field_lengths: Vec<usize>,
}

impl HttpLogLine {
    fn to_line(&self, column_sizes: &[usize]) -> String {
        let line = format!(
            "{:<w0$} {:<w1$} {:<w2$} [{:<w3$}] \"{} {} {:<w6$}\" {:<w7$} {:<w8$} \"{}\" \"{}\" {:<w11$}",
            as_column(&self.host, column_sizes[0]),
            as_column(&self.client_identity, column_sizes[1]),
            as_column(&self.user_id, column_sizes[2]),
            as_column(&self.date_time, column_sizes[3]),
            as_column(&self.http_method, column_sizes[4]),
            as_column(&self.requested_url, column_sizes[5]),
            self.http_protocol_version.to_string(),
            self.http_status_code.to_string(),
            self.response_body_size.to_string(),
            self.referrer_url.to_string(),
            self.agent.to_string(),
            self.code.to_string(),
            w0 = column_sizes[0],
            w1 = column_sizes[1],
            w2 = column_sizes[2],
            w3 = column_sizes[3],
            w6 = column_sizes[6],
            w7 = column_sizes[7],
            w8 = column_sizes[8],
            w11 = column_sizes[11],
        );
        line
    }
}

fn main() {
    let mut max_field_lengths: Vec<usize> = vec![];

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        match from_line(&line) {
            Some(log_line) => {
                new_max_field_lengths(&mut max_field_lengths, &log_line.field_lengths);
                let print_line = log_line.to_line(&max_field_lengths);

                if (400..500).contains(&log_line.http_status_code) {
                    println!("{}", warn(&print_line));
                } else if (500..600).contains(&log_line.http_status_code) {
                    println!("{}", error(&print_line));
                } else {
                    println!("{}", ok(&print_line));
                }
            }
            None => {
                println!("{}", line.trim());
            }
        }
    }
}

fn as_column(field: &str, column_size: usize) -> String {
    format!("{field:<width$}", field = field, width = column_size)
}


fn from_line(line: &str) -> Option<HttpLogLine> {
    let pattern = r#"^(\S+) (\S+) (\S+) \[([\w:/]+\s[+\-]\d{4})\] "(\S+) (\S+) (\S+)" (\d{3}) (\S+) "(\S+)" "([^"]+)" (\d+)"#;
    let re = Regex::new(pattern).unwrap();
    if let Some(captures) = re.captures(line) {
        let field_lengths = captures
            .iter()
            .skip(1)
            .map(|capture| capture.unwrap().as_str().len())
            .collect::<Vec<_>>();
        Some(HttpLogLine {
            host: captures[1].to_owned(),
            client_identity: captures[2].to_owned(),
            user_id: captures[3].to_owned(),
            date_time: captures[4].to_owned(),
            http_method: captures[5].to_owned(),
            requested_url: captures[6].to_owned(),
            http_protocol_version: captures[7].to_owned(),
            http_status_code: captures[8].parse().unwrap(),
            response_body_size: captures[9].parse().unwrap(),
            referrer_url: captures[10].to_owned(),
            agent: captures[11].to_owned(),
            code: captures[12].parse().unwrap(),
            field_lengths: field_lengths.to_owned(),
        })
    } else {
        None
    }
}

fn ok(string: &str) -> String {
    format!("\x1B[92m{}\x1B[0m", string)
}

fn warn(string: &str) -> String {
    format!("\x1B[93m{}\x1B[0m", string)
}

fn error(string: &str) -> String {
    format!("\x1B[91m{}\x1B[0m", string)
}

fn new_max_field_lengths(max_field_lengths: &mut Vec<usize>, field_lengths: &[usize]) {
    if max_field_lengths.len() < field_lengths.len() {
        *max_field_lengths = field_lengths.to_vec();
    } else {
        for (max_len, len) in max_field_lengths.iter_mut().zip(field_lengths) {
            *max_len = max(*max_len, *len);
        }
    }
}

