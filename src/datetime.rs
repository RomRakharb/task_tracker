use std::time::SystemTime;

#[derive(Default, Debug)]
pub struct DateTime {
    years: u16,
    months: u8,
    days: u8,
    hours: u8,
    minutes: u8,
    seconds: u8,
}

impl DateTime {
    pub fn now() -> Self {
        const SECONDS_IN_MINUTE: u64 = 60;
        const SECONDS_IN_HOUR: u64 = 60 * SECONDS_IN_MINUTE;
        const SECONDS_IN_DAY: u64 = 24 * SECONDS_IN_HOUR;

        const MONTHS_IN_YEAR: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        const MONTHS_IN_LEAP_YEAR: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        const SECOND_IN_YEAR: u64 = 365 * SECONDS_IN_DAY;
        const SECOND_IN_LEAP_YEAR: u64 = 366 * SECONDS_IN_DAY;

        let mut time_pool: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(time) => time.as_secs(),
            Err(e) => {
                eprintln!("Error at DateTime::now : {e}");
                return Self::default();
            }
        };

        let mut years = 1970;
        let mut months = 1;
        let mut days = 1;
        let mut hours = 0;
        let mut minutes = 0;

        while time_pool > SECOND_IN_YEAR {
            if is_leap_year(years) {
                time_pool -= SECOND_IN_LEAP_YEAR;
            } else {
                time_pool -= SECOND_IN_YEAR;
            }
            years += 1;
        }

        let this_year = if is_leap_year(years) {
            MONTHS_IN_LEAP_YEAR
        } else {
            MONTHS_IN_YEAR
        };

        for month in this_year {
            if time_pool < month * SECONDS_IN_DAY {
                break;
            }
            time_pool -= month * SECONDS_IN_DAY;
            months += 1;
        }

        days += time_pool / SECONDS_IN_DAY;
        time_pool %= SECONDS_IN_DAY;

        hours += time_pool / SECONDS_IN_HOUR;
        time_pool %= SECONDS_IN_HOUR;

        minutes += time_pool / SECONDS_IN_MINUTE;
        time_pool %= SECONDS_IN_MINUTE;

        Self {
            years,
            months: months as u8,
            days: days as u8,
            hours: hours as u8,
            minutes: minutes as u8,
            seconds: time_pool as u8,
        }
    }
}

fn is_leap_year(year: u16) -> bool {
    (year - 1968) % 4 == 0
}
