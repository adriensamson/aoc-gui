use std::thread::sleep;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;
use tokio_stream::wrappers::UnboundedReceiverStream;

#[derive(Clone, Debug)]
pub enum AocMessage {
    Log(String),
    Progress(usize, usize),
    ResultPart1(String),
    ResultPart2(String),
}

pub type AocStream = UnboundedReceiverStream<AocMessage>;
pub struct AocDay(&'static (dyn Fn(AocSender, String) + Send + Sync));

struct AocSender(UnboundedSender<AocMessage>);

impl AocSender {
    fn log(&self, log: impl ToString) {
        self.0.send(AocMessage::Log(log.to_string())).unwrap();
    }

    fn result_part1(&self, res: impl ToString) {
        self.0.send(AocMessage::ResultPart1(res.to_string())).unwrap();
    }

    fn result_part2(&self, res: impl ToString) {
        self.0.send(AocMessage::ResultPart2(res.to_string())).unwrap();
    }

    fn progress(&self, prog: usize, total: usize) {
        self.0.send(AocMessage::Progress(prog, total)).unwrap();
    }
}

impl AocDay {
    pub fn run(&'static self, input: String) -> AocStream {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        tokio::task::spawn_blocking(move || {
            self.0(AocSender(tx), input);
        });
        UnboundedReceiverStream::new(rx)
    }
}

pub const DAYS : [AocDay; 12] = [
    AocDay(&dummy_day),
    AocDay(&dummy_day),
    AocDay(&dummy_day),
    AocDay(&dummy_day),
    AocDay(&dummy_day),
    AocDay(&dummy_day),
    AocDay(&dummy_day),
    AocDay(&dummy_day),
    AocDay(&dummy_day),
    AocDay(&dummy_day),
    AocDay(&dummy_day),
    AocDay(&dummy_day),
];

fn dummy_day(sender: AocSender, input: String) {
    let line_count = input.lines().count();
    sender.log(format!("lines: {line_count}"));
    sender.progress(1, 2);
    sender.result_part1(line_count);

    sleep(Duration::from_secs(2));

    let cols_count = input.lines().map(|line| line.len()).max().unwrap();
    sender.log(format!("cols: {cols_count}"));
    sender.progress(2, 2);
    sender.result_part2(cols_count);
}
