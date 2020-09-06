use std::io;
use std::io::BufRead;

mod ledger;
use ledger::account::Account;
use ledger::title::Title;

#[derive(Debug)]
struct Transaction<'a> {
    title: Title<'a>,
    accounts: Vec<Account<'a>>,
}

struct ErrorMsg<'a> {
    line_no: i32,
    message: &'a str,
}

fn parse<'a>(source: impl BufRead) -> Result<Vec<Transaction<'a>>, ErrorMsg<'a>> {
    let mut num = 0i32;
    let mut lines_iter = source.lines();

    while let Some(line) = lines_iter.next() {
        num += 1;
        if let Ok(line) = line {
            let title = Title::parse(&line);
            if title.is_some() {
                println!("{}", &line);
            }
            while let Some(Ok(line)) = lines_iter.next() {
                if line.starts_with(' ') {
                    let account = Account::parse(line.trim());
                    if account.is_ok() {
                        println!("{:?}", account);
                    }
                    // parse accounts
                }
            }
        } else {
            return Err(ErrorMsg {
                line_no: num,
                message: &"Input error",
            });
        }
    }

    /*
    for line in source.lines() {
        num += 1;
        if let Ok(line) = line {
            let title = Title::parse(&line);
            if title.is_some() {
                println!("{}", &line);
            }
        } else {
            return Err(ErrorMsg {
                line_no: num,
                message: &"Input error",
            });
        }
    }
    */

    Err(ErrorMsg {
        line_no: 1,
        message: &"todo",
    })
}

fn main() {
    let result = parse(io::stdin().lock());
}
