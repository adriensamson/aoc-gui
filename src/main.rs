use std::fmt::{Display, Formatter};
use iced::{widget, Element, Task};
use iced::futures::Stream;
use iced::widget::{column};
use aoc_gui::AocMessage;

fn main() -> iced::Result {
    iced::application("AOC 2025", App::update, App::view)
        .centered()
        .run()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct DayName(usize);
impl Display for DayName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day {}", self.0 + 1)
    }
}

const DAY_NAMES: [DayName; 12] = [
    DayName(0),
    DayName(1),
    DayName(2),
    DayName(3),
    DayName(4),
    DayName(5),
    DayName(6),
    DayName(7),
    DayName(8),
    DayName(9),
    DayName(10),
    DayName(11),
];

#[derive(Default)]
struct App {
    day: Option<DayName>,
    input: widget::text_editor::Content,
    running: bool,
    logs: Vec<String>,
    progress: (usize, usize),
    result_part1: Option<String>,
    result_part2: Option<String>,
}

#[derive(Clone, Debug)]
enum AppMessage {
    Reset,
    SelectDay(DayName),
    InputAction(widget::text_editor::Action),
    Run,
    AocMessage(AocMessage),
}

impl App {
    fn update(&mut self, message: AppMessage) -> Task<AppMessage> {
        match message {
            AppMessage::Reset => {
                self.running = false;
                self.logs = vec![];
                self.result_part1 = None;
                self.result_part2 = None;
                self.progress = (0, 0);
                Task::none()
            },
            AppMessage::SelectDay(day) => {
                self.day = Some(day);
                Task::none()
            },
            AppMessage::InputAction(action) => {
                self.input.perform(action);
                Task::none()
            }
            AppMessage::Run => {
                if let Some(day) = self.day.and_then(|d| aoc_gui::DAYS.get(d.0)) {
                    self.running = true;
                    Task::run(iter_to_stream(day(self.input.text())), AppMessage::AocMessage)
                } else {
                    Task::none()
                }
            },
            AppMessage::AocMessage(AocMessage::Log(log)) => {
                self.logs.push(log);
                Task::none()
            },
            AppMessage::AocMessage(AocMessage::Progress(prog, total)) => {
                self.progress = (prog, total);
                Task::none()
            },
            AppMessage::AocMessage(AocMessage::ResultPart1(res)) => {
                self.result_part1 = Some(res);
                Task::none()
            },
            AppMessage::AocMessage(AocMessage::ResultPart2(res)) => {
                self.result_part2 = Some(res);
                Task::none()
            },
        }
    }

    fn view(&self) -> Element<'_, AppMessage> {
        let day = widget::pick_list(DAY_NAMES, self.day, AppMessage::SelectDay);
        let input = widget::text_editor(&self.input).height(120).on_action(AppMessage::InputAction);
        let run : Element<_> = if self.running {
            column![
                widget::button("Reset").on_press(AppMessage::Reset),
                widget::progress_bar(0f32..=(self.progress.1 as f32), self.progress.0 as f32),
                if let Some(res) = &self.result_part1 { widget::text!("Part1: {res}") } else { widget::text("Part1...") },
                if let Some(res) = &self.result_part2 { widget::text!("Part2: {res}") } else { widget::text("Part2...") },
                column(self.logs.iter().map(|l| widget::text(l).into())),
            ].into()
        } else {
            widget::button("Run").on_press(AppMessage::Run).into()
        };
        column![day, input, run].into()
    }
}

fn iter_to_stream<T: Send + 'static>(iter: impl Iterator<Item=T> + Send + 'static) -> impl Stream<Item=T> + Send {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    tokio::task::spawn_blocking(move || {
        for item in iter {
            tx.send(item).unwrap();
        }
    });
    tokio_stream::wrappers::UnboundedReceiverStream::new(rx)
}