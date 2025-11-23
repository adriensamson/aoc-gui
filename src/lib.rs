pub enum AocMessage {
    Log(String),
    Progress(usize, usize),
    ResultPart1(String),
    ResultPart2(String),
}

pub type AocIterator = dyn Iterator<Item=AocMessage>;
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

fn dummy_day(_input: String) -> Box<AocIterator> {
    todo!()
}