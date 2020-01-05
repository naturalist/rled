use chrono::NaiveDate;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref TITLE_RE: Regex = Regex::new(
        r"^(\d{4}/\d{2}/\d{2})(=?(\d{4}/\d{2}/\d{2}))?(\s+([!*]))?(\s+\((#?\d+)\))?\s*(.+)$"
    )
    .unwrap();
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Pending,
    Cleared,
}

#[derive(Debug)]
pub struct Title<'a> {
    date: NaiveDate,
    aux_date: Option<NaiveDate>,
    status: Option<Status>,
    code: Option<i32>,
    payee: &'a str,
}

impl<'c> Title<'c> {
    fn parse_date<'a>(st: impl Into<&'a str>) -> Option<NaiveDate> {
        let mut s = String::from(st.into());

        // Aux date may have a = in front of it
        if s.starts_with('=') {
            s.remove(0);
        }

        let parts: Vec<i32> = s.split('/').map(|x| x.parse::<i32>().unwrap()).collect();
        if parts.len() != 3 {
            None
        } else {
            NaiveDate::from_ymd_opt(parts[0], parts[1] as u32, parts[2] as u32)
        }
    }

    // Parses the title
    pub fn parse(line: &'c str) -> Option<Self> {
        // Strings, with turn into a regex needed to parse the title
        TITLE_RE.captures(line).map(|caps| {
            let date = caps.get(1).and_then(Self::parse_date);
            let aux_date = caps.get(2).and_then(Self::parse_date);

            let status = caps.get(5).and_then(|s| match s.as_str() {
                "*" => Some(Status::Cleared),
                "!" => Some(Status::Pending),
                _ => None,
            });

            let code = caps.get(7).map(|s| {
                let mut ss = String::from(s.as_str());
                if ss.starts_with('#') {
                    ss.remove(0);
                }
                ss.parse::<i32>().unwrap()
            });

            let payee = caps.get(8).map(|s| s.as_str());

            Title {
                date: date.unwrap(),
                aux_date: aux_date,
                status: status,
                code: code,
                payee: payee.unwrap(),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let t = Title::parse(&"2020/01/02 Title").unwrap();
        assert_eq!(t.status, None);
        assert_eq!(t.date, NaiveDate::from_ymd(2020, 01, 02));
        assert_eq!(t.payee, "Title");
    }

    #[test]
    fn cleared() {
        let t = Title::parse(&"2020/01/02 * Title").unwrap();
        assert_eq!(t.status, Some(Status::Cleared));
        assert_eq!(t.date, NaiveDate::from_ymd(2020, 01, 02));
    }

    #[test]
    fn pending() {
        let t = Title::parse(&"2020/01/02 ! Title").unwrap();
        assert_eq!(t.status, Some(Status::Pending));
        assert_eq!(t.date, NaiveDate::from_ymd(2020, 01, 02));
    }

    #[test]
    fn aux_date() {
        let t = Title::parse(&"2020/01/02=2020/01/05 * Title").unwrap();
        assert_eq!(t.date, NaiveDate::from_ymd(2020, 01, 02));
        assert_eq!(t.aux_date, Some(NaiveDate::from_ymd(2020, 01, 05)));
    }

    #[test]
    fn code() {
        let t = Title::parse(&"2020/01/02=2020/01/05 * (#123) Title").unwrap();
        assert_eq!(t.code, Some(123));
        let t = Title::parse(&"2020/01/02=2020/01/05 * (123) Title").unwrap();
        assert_eq!(t.code, Some(123));
    }
}
