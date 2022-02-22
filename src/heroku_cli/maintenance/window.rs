use crate::heroku_cli::maintenance;
use crate::heroku_cli::maintenance::{HerokuCmd, Status};
use chrono::{offset::TimeZone, DateTime, NaiveDateTime, Utc};
use std::thread;

struct MaintenanceWindow {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl MaintenanceWindow {
    fn new(start: &NaiveDateTime, end: &NaiveDateTime) -> Self {
        let start: DateTime<Utc> = Utc.from_local_datetime(start).unwrap();
        let end: DateTime<Utc> = Utc.from_local_datetime(end).unwrap();

        assert!(
            !(start < Utc::now() || end < Utc::now()),
            "The maintenance window must not be in the past"
        );
        assert!(!(start > end), "Invalid maintenance window");

        MaintenanceWindow { start, end }
    }
}

pub fn execute(start: &NaiveDateTime, end: &NaiveDateTime) {
    let window = MaintenanceWindow::new(start, end);

    sleep_until(&window.start);
    maintenance::execute(&HerokuCmd::MaintenanceMode(Status::On));

    sleep_until(&window.end);
    maintenance::execute(&HerokuCmd::MaintenanceMode(Status::Off));
}

fn sleep_until(date_time: &DateTime<Utc>) {
    let duration = date_time
        .signed_duration_since(Utc::now())
        .to_std()
        .expect("Invalid duration");

    log::debug!("DURATION: {duration:?}");
    thread::sleep(duration);
}
