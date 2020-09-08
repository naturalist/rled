use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ACCOUNT_RE: Regex = Regex::new(r"\s+(([^;]+)(\s{2,}|\t)([^;]+))?(;.+)?").unwrap();
}

#[derive(Debug)]
pub struct Account<'a> {
    name: Option<&'a str>,
    amount: Option<&'a str>,
    comment: Option<&'a str>,
}

impl<'c> Account<'c> {
    pub fn parse(text: &'c str) -> Option<Self> {
        ACCOUNT_RE.captures(text).map(|caps| {
            let name = caps.get(2).map(|x| x.as_str().trim());
            let amount = caps.get(4).map(|x| x.as_str().trim());
            let comment = caps.get(5).map(|x| x.as_str().trim());
            Account {
                name: name,
                amount: amount,
                comment: comment,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_all(a: Account, name: Option<&str>, amount: Option<&str>, comment: Option<&str>) {
        assert_eq!(a.name, name);
        assert_eq!(a.amount, amount);
        assert_eq!(a.comment, comment);
    }

    #[test]
    fn simple() {
        let a = Account::parse(&" Bar:Foo  $123.45 ; Comment").unwrap();
        assert_all(a, Some("Bar:Foo"), Some("$123.45"), Some("; Comment"));
    }

    #[test]
    fn comment_only() {
        let a = Account::parse(&" ;Comment").unwrap();
        assert_all(a, None, None, Some(";Comment"));
    }

    #[test]
    fn long() {
        let a = Account::parse(&" Bar:Foo  ($123 + $345) ; Hello").unwrap();
        assert_all(a, Some("Bar:Foo"), Some("($123 + $345)"), Some("; Hello"));
    }
}
