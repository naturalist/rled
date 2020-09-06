use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ACCOUNT_RE: Regex = Regex::new(r"\s+()").unwrap();
}

#[derive(Debug)]
pub enum Account<'a> {
    Full {
        name: &'a str,
        amount: &'a str,
        comment: &'a str,
    },
    Comment {
        comment: &'a str,
    },
}

impl<'c> Account<'c> {
    pub fn parse(text: &'c str) -> Result<Self, &str> {
        if text.starts_with(';') {
            Ok(Account::Comment { comment: text })
        } else {
            // Split at ; to separate comments
            let parts: Vec<&str> = text.split(';').map(|x| x.trim()).collect();
            let comment = parts.get(1).unwrap_or(&"");
            let re = Regex::new(r"(\s{2,}|\t)").unwrap();
            let parts: Vec<&str> = re.splitn(parts[0], 2).map(|x| x.trim()).collect();
            if parts.len() == 2 {
                Ok(Account::Full {
                    name: parts[0],
                    amount: parts[1],
                    comment: comment,
                })
            } else {
                Err(&"Invalid account line")
            }
        }
    }
}
