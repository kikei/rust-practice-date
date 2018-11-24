extern crate chrono;
use chrono::{TimeZone, Weekday, ParseResult};
use chrono::prelude::{DateTime, Utc, Local, Datelike, Timelike};

fn epoch_from_datetime(&datetime: &DateTime<Local>) -> f64 {
    let secs = datetime.timestamp() as f64;
    let nanos = (datetime.timestamp_subsec_nanos() as f64) / 1e9;
    secs + nanos
}

fn local_from_epoch(nanos: f64) -> DateTime<Local> {
    let secs = nanos as i64;
    let nsecs = (nanos - secs as f64) * 1e9;
    Local.timestamp(secs, nsecs as u32)
}

fn format_japan_weekday(weekday: &Weekday) -> &str {
    match weekday {
        Weekday::Mon => "月",
        Weekday::Tue => "火",
        Weekday::Wed => "水",
        Weekday::Thu => "木",
        Weekday::Fri => "金",
        Weekday::Sat => "土",
        Weekday::Sun => "日",
    }
}

fn format_japan_date(datetime: &DateTime<Local>) -> String {
    format!("{}年{:02}月{:02}日({}) {:02}時{:02}分{:02}秒",
            datetime.year(),
            datetime.month(),
            datetime.day(),
            format_japan_weekday(&datetime.weekday()),
            datetime.hour(),
            datetime.minute(),
            datetime.second())
}

fn parse_japan_date(s: &String) -> ParseResult<DateTime<Local>>  {
    let chars = s.chars();
    let s1: String = chars.clone().take_while(|&c| c != '(').collect();
    let s2: String = chars.skip_while(|&c| c != ')').skip(1).collect();
    let text = s1 + &s2;
    Local.datetime_from_str(&text, "%Y年%m月%d日 %H時%M分%S秒")
}

fn print_datetime() {
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();

    println!("Now:\n    utc: {:?}\n    local: {:?}",
             utc.to_string(), local.to_string());
    println!("Timestamp:\n    utc.timestamp(): {}\n    local.timestamp(): {}",
             utc.timestamp(), local.timestamp());
    println!("TimeZone:\n    local.offset()={}", local.offset().to_string());
}

fn main() {
    print_datetime();
}

#[cfg(test)]
mod tests {
    use chrono::prelude::{Local, TimeZone};
    use chrono::Duration;
    use chrono::offset::FixedOffset;
    use *;
    
    #[test]
    fn test_now() {
        let utc = Utc::now();
        let local = Local::now();
        assert!(utc.timestamp() == local.timestamp());
    }

    #[test]
    fn test_format_iso() {
        let datetime = Utc.ymd(2018, 1, 2).and_hms(3, 44, 55);
        assert!(datetime.format("%Y-%m-%dT%H:%M:%S%z").to_string() ==
                "2018-01-02T03:44:55+0000");
    }

    #[test]
    fn test_parse_iso() {
        let datetime = "2018-01-02T03:44:55+0900";
        assert!(DateTime::parse_from_str(datetime, "%Y-%m-%dT%H:%M:%S%z") ==
                Ok(FixedOffset::east(60 * 60 * 9)
                   .ymd(2018, 1, 2).and_hms(3, 44, 55)));
    }

    #[test]
    fn test_epoch_from_datetime() {
        let local = Local.ymd(2018, 1, 1).and_hms(0, 0, 0);
        assert!(epoch_from_datetime(&local) == 1514732400.0);
    }

    #[test]
    fn test_local_from_epoch() {
        let epoch: f64 = 1514732400.0;
        assert!(local_from_epoch(epoch) ==
                Local.ymd(2018, 1, 1).and_hms(0, 0, 0));
    }

    #[test]
    fn test_format_japan_date() {
        let local = Local.ymd(2018, 1, 2).and_hms(3, 44, 55);
        assert!(format_japan_date(&local) == "2018年01月02日(火) 03時44分55秒");
    }

    #[test]
    fn test_parse_japan_date() {
        let date = String::from("2018年01月02日(火) 03時44分55秒");
        assert!(parse_japan_date(&date).expect("error") ==
                Local.ymd(2018, 1, 2).and_hms(3, 44, 55));
    }

        
    #[test]
    fn test_tomorrow() {
        let local = Local::now();
        let offset = Duration::days(1);
        let tomorrow = local + offset;
        assert!(local.timestamp() + 24 * 60 * 60 == tomorrow.timestamp());
    }

    #[test]
    fn test_yesterday() {
        let local = Local::now();
        let offset = Duration::days(-1);
        let tomorrow = local + offset;
        assert!(local.timestamp() - 24 * 60 * 60 == tomorrow.timestamp());
    }

    #[test]
    fn test_diff() {
        let local = Local::now();
        let tomorrow = local + Duration::days(1);
        assert!(tomorrow - local == Duration::days(1));
    }
}
