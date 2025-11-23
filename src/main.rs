use std::fmt::{Display, Formatter};
use iced::widget::{column};

fn main() -> iced::Result {
    iced::application("AOC 2026", App::update, App::view)
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
    input: iced::widget::text_editor::Content,
    running: Option<Box<aoc_gui::AocIterator>>,
    logs: Vec<String>,
    result_part1: Option<String>,
    result_part2: Option<String>,
}

#[derive(Clone, Debug)]
enum AppMessage {
    Reset,
    SelectDay(DayName),
    InputAction(iced::widget::text_editor::Action),
    Run,
}

impl App {
    fn update(&mut self, message: AppMessage) {
        match message {
            AppMessage::Reset => {
                self.running = None;
                self.logs = vec![];
                self.result_part1 = None;
                self.result_part2 = None;
            },
            AppMessage::SelectDay(day) => {
                self.day = Some(day);
            },
            AppMessage::InputAction(action) => {
                self.input.perform(action);
            }
            AppMessage::Run => {
                if let Some(day) = self.day.and_then(|d| aoc_gui::DAYS.get(d.0)) {
                    self.running = Some(day(self.input.text()));
                    todo!("run iterator")
                }
            }
        }
    }

    fn view(&self) -> iced::Element<'_, AppMessage> {
        let day = iced::widget::pick_list(DAY_NAMES, self.day, AppMessage::SelectDay);
        let input = iced::widget::text_editor(&self.input).height(120).on_action(AppMessage::InputAction);
        let run = iced::widget::button("Run").on_press(AppMessage::Run);
        column![day, input, run].into()
    }
}
