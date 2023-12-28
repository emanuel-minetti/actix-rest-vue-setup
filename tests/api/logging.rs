use crate::helpers::spawn_app;
use chrono::{Local, NaiveDate};
use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::Read;

#[tokio::test]
async fn writing_to_logfile_works() {
    // Arrange
    let test_app = spawn_app();
    let path_string = test_app.settings.log_settings().path() + ".log";
    let path = std::path::Path::new(&path_string);
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(path)
        .expect("Failed to open logfile for writing");
    file.set_len(0).expect("Clearing logfile failed");
    log::info!("Logfile cleared in test 'writing_to_logfile_works'");
    let client = reqwest::Client::new();
    // Act
    let now_local_dt = Local::now();
    let _ = client
        .get(format!("{}/logtest", &test_app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    let mut file = File::open(path).expect("Failed to open logfile for reading");
    let mut file_buffer = String::new();
    let _ = file.read_to_string(&mut file_buffer);
    let log_line_re =
        Regex::new(r#"(?m)^(?P<date_time>[\d\- :]+): \[(?P<log_level>\w+)].+"GET /logtest.+$"#)
            .unwrap();
    let line_caps = log_line_re
        .captures(file_buffer.as_str())
        .expect("No log entry from acting");
    let date_time_re = Regex::new(
        r"(?P<year>\d+)-(?P<month>\d+)-(?P<day>\d+) (?P<hour>\d+):(?P<minute>\d+):(?P<second>\d+)",
    )
    .unwrap();
    let dt_caps = date_time_re
        .captures(&line_caps["date_time"])
        .expect("Failed to parse date_time entry");
    let year: i32 = dt_caps["year"].parse().expect("Failed to parse year");
    let month: u32 = dt_caps["month"].parse().expect("Failed to parse month");
    let day: u32 = dt_caps["day"].parse().expect("Failed to parse day");
    let hour: u32 = dt_caps["hour"].parse().expect("Failed to parse hour");
    let minute: u32 = dt_caps["minute"].parse().expect("Failed to parse minute");
    let second: u32 = dt_caps["second"].parse().expect("Failed to parse second");
    let logged_local_dt = NaiveDate::from_ymd_opt(year, month, day)
        .unwrap()
        .and_hms_opt(hour, minute, second)
        .unwrap()
        .and_local_timezone(Local)
        .unwrap();
    let dt_diff_in_secs = now_local_dt
        .signed_duration_since(logged_local_dt)
        .abs()
        .num_seconds();
    assert_eq!(&line_caps["log_level"], "INFO");
    assert!(dt_diff_in_secs <= 2);
}
