use std::io;
use std::io::BufRead;

mod ledger;
use ledger::title::Title;

#[derive(Debug)]
struct Register<'a> {
    account: &'a str,
    amount: f32,
}

#[derive(Debug)]
struct Entry<'a> {
    title: Title<'a>,
    lines: Vec<Register<'a>>,
}

struct ErrorMsg<'a> {
    line_no: i32,
    message: &'a str,
}

fn parse<'a>(source: impl BufRead) -> Result<Vec<Entry<'a>>, ErrorMsg<'a>> {
    Err(ErrorMsg {
        line_no: 1,
        message: &"todo",
    })
}

fn main() {
    let result = parse(io::stdin().lock());
}
