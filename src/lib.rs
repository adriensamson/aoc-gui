#[derive(Clone, Debug)]
pub enum AocMessage {
    Log(String),
    Progress(usize, usize),
    ResultPart1(String),
    ResultPart2(String),
}

pub type AocIterator = dyn Iterator<Item=AocMessage> + Send + 'static;
type AocDay = fn(String) -> Box<AocIterator>;

pub const DAYS : [AocDay; 12] = [
    dummy_day,
    dummy_day,
    dummy_day,
    dummy_day,
    dummy_day,
    dummy_day,
    dummy_day,
    dummy_day,
    dummy_day,
    dummy_day,
    dummy_day,
    dummy_day,
];

fn dummy_day(input: String) -> Box<AocIterator> {
    let line_count = input.lines().count();
    let cols_count = input.lines().map(|line| line.len()).max().unwrap();
    Box::new(vec![
        AocMessage::Log(format!("lines: {line_count}")),
        AocMessage::Progress(1, 2),
        AocMessage::ResultPart1(format!("{line_count}")),
        AocMessage::Log(format!("cols: {cols_count}")),
        AocMessage::Progress(2, 2),
        AocMessage::ResultPart2(format!("{cols_count}")),
    ].into_iter())
}