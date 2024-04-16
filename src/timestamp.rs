use cosmwasm_std::Timestamp;

const SECONDS_IN_MINUTE: u64 = 60;
const SECONDS_IN_HOUR: u64 = 3600;
const SECONDS_IN_DAY: u64 = 86400;
const SECONDS_IN_YEAR: u64 = 31536000;
const SECONDS_IN_FOUR_YEARS_WITH_LEAP_YEAR: u64 = 126230400;
const SECONDS_BETWEEN_JAN_1_1972_AND_DEC_31_1999: u64 = 883612800;
const SECONDS_IN_100_YEARS: u64 = 3155673600;
const SECONDS_IN_400_YEARS: u64 = 12622780800;

pub fn convert_iso_string_to_timestamp(iso_string: &String) -> Timestamp {
    let (year, month, day, hour, minute, second, decimal) = convert_date_time_string_to_ymdhms(iso_string);
    convert_ymdhms_to_timestamp(year, month, day, hour, minute, second, decimal)
}

// YYYY-MM-DDTHH:MM:SS.ss~Z => (year, month, day, hour, minute, second, decimal)
fn convert_date_time_string_to_ymdhms(iso_string: &String) -> (u64, u64, u64, u64, u64, u64, u64) {
    let mut split = iso_string.split("T");
    let iso_date = split.next().unwrap();
    let iso_time = split.next().unwrap();

    let (year, month, day) = convert_date_string_to_ymd(iso_date);
    let (hour, minute, second, decimal) = split_time_int(iso_time);

    (year, month, day, hour, minute, second, decimal)
}

fn convert_date_string_to_ymd(iso_date: &str) -> (u64, u64, u64) {
    let mut split = iso_date.split("-");
    let year = split.next().unwrap();
    let month = split.next().unwrap();
    let day = split.next().unwrap();

    (year.parse().unwrap(), month.parse().unwrap(), day.parse().unwrap())
}

fn split_time_int(iso_time: &str) -> (u64, u64, u64, u64) {
    let mut split = iso_time.split(":");
    let hour = split.next().unwrap();
    let minute = split.next().unwrap();
    let remain = split.next().unwrap();

    let mut second_split = remain.split(".");
    let second = second_split.next().unwrap();
    let second_remain = second_split.next().unwrap_or("0");
    // to remove Z
    let mut decimal_split = second_remain.split("Z"); 
    let decimal = decimal_split.next().unwrap();
    // pad zeros
    let decimal = &format!("{:0<9}", decimal);


    (hour.parse().unwrap(), minute.parse().unwrap(), second.parse().unwrap(), decimal.parse().unwrap())    
}

fn convert_ymdhms_to_timestamp(year: u64, month: u64, day: u64, hour: u64, minute: u64, second: u64, decimal: u64) -> Timestamp {
    let mut ts: u64 = 0;
    ts = add_year_to_second(ts, year);
    ts += add_month_seconds(year, month);
    ts += (day - 1) * SECONDS_IN_DAY;
    ts += hour * SECONDS_IN_HOUR;
    ts += minute * SECONDS_IN_MINUTE;
    ts += second;

    return Timestamp::from_nanos(ts * 1_000_000_000 + decimal);
}

fn add_year_to_second(ts: u64, year: u64) -> u64 {
    let mut year_counter;
    let mut ts = ts;

    if year < 1972 {

        ts += (year - 1970) * SECONDS_IN_YEAR;

    } else {

        ts += 2 * SECONDS_IN_YEAR;
        year_counter = 1972;

        if year >= 2000 {

            ts += SECONDS_BETWEEN_JAN_1_1972_AND_DEC_31_1999;
            year_counter = 2000;

            (year_counter, ts) = increment_year_and_timestamp(year, year_counter, ts, 400, SECONDS_IN_400_YEARS);
            (year_counter, ts) = increment_leap_year(year, year_counter, ts);
            (year_counter, ts) = increment_year_and_timestamp(year, year_counter, ts, 100, SECONDS_IN_100_YEARS);
        }

        (year_counter, ts) = increment_year_and_timestamp(year, year_counter, ts, 4, SECONDS_IN_FOUR_YEARS_WITH_LEAP_YEAR);
        (year_counter, ts) = increment_leap_year(year, year_counter, ts);
        (_, ts) = increment_year_and_timestamp(year, year_counter, ts, 1, SECONDS_IN_YEAR);
    }

    return ts;
}


fn increment_year_and_timestamp(year: u64, year_counter: u64, ts: u64, divisor: u64, seconds: u64) -> (u64, u64) {

    let mut ts = ts;
    let mut year = year;

    let res = (year - year_counter) / divisor;
    year = year_counter + (res * divisor);
    ts = ts + (res * seconds);

    (year, ts)
}

fn increment_leap_year(year: u64, year_counter: u64, ts: u64) -> (u64, u64) {
    let mut year_counter = year_counter;
    let mut ts = ts;

    if (year_counter < year) && is_leap_year(year_counter) {

        year_counter += 1;
        ts += SECONDS_IN_YEAR + SECONDS_IN_DAY;
    }

    (year_counter, ts)
}

fn is_leap_year(year: u64) -> bool {

    if (year % 4) != 0 { return false; }
    if ((year % 400) == 0) || ((year % 100) != 0) { return true; }

    return false;
}

fn add_month_seconds(year: u64, month: u64) -> u64 {

    let month_seconds_map: Vec<u64>;

    if is_leap_year(year) {
        month_seconds_map = vec![0, 2678400, 5184000, 7862400, 10454400, 13132800,
                            15724800, 18403200, 21081600, 23673600, 26352000,
                            28944000, 31622400];
    } else {
        month_seconds_map = vec![0, 2678400, 5097600, 7776000, 10368000, 13046400,
                            15638400, 18316800, 20995200, 23587200, 26265600,
                            28857600, 31536000];
    }

    return month_seconds_map[(month as usize) - 1];
}