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
