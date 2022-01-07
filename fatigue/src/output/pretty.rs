use crate::output::OutputFormatter;
use console::Style;
use crossterm::cursor::MoveTo;
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, QueueableCommand};
use itertools::Itertools;
use libfatigue::context::TestResult;
use libfatigue::FatigueTestError;
use num_format::{Locale, ToFormattedString};
use std::io::{stdout, Stdout, Write};
use std::sync::Mutex;

pub(crate) struct PrettyOutputFormatter {
    inner: Mutex<PrettyInner>,
}

struct PrettyInner {
    cyan: Style,
    blue: Style,
    _red: Style,
    _yellow: Style,
    _green: Style,
    out: Stdout,
}

impl PrettyOutputFormatter {
    pub fn new() -> Self {
        let mut out = stdout();
        out.execute(Clear(ClearType::All))
            .expect("todo: result handling");
        let cyan = Style::new().cyan();
        let blue = Style::new().blue();
        let red = Style::new().red();
        let yellow = Style::new().yellow();
        let green = Style::new().green();
        let inner = PrettyInner {
            cyan,
            blue,
            _red: red,
            _yellow: yellow,
            _green: green,
            out,
        };
        let inner = Mutex::new(inner);
        PrettyOutputFormatter { inner }
    }
}

impl OutputFormatter for PrettyOutputFormatter {
    fn update_result_status(&self, result: &TestResult) {
        let mut guard = self.inner.lock().unwrap();
        guard.write(result);
    }

    fn write_final_results(&self, result: TestResult) {
        let mut guard = self.inner.lock().unwrap();
        guard.write(&result);
    }

    fn write_err(&self, _err: FatigueTestError) {
        todo!()
    }

    fn tick(&self) {}
}

impl PrettyInner {
    fn write(&mut self, val: &TestResult) {
        self.out.queue(MoveTo(0, 0)).expect("todo: result handling");
        self.out
            .queue(Clear(ClearType::All))
            .expect("todo: result handling");
        for test_name in val.timings.keys().sorted() {
            self.out
                .queue(Print(format!(
                    "* {}\n",
                    self.cyan.apply_to(test_name.as_str())
                )))
                .expect("todo: result handling");

            let status_map = &val.timings[test_name];
            for status_code in status_map.keys().sorted() {
                let timings = &status_map[status_code];
                self.out
                    .queue(Print(format!(
                        "\t- {}\t{}\t{}\t{}\n",
                        self.blue.apply_to(&status_code.as_str()[..3]),
                        timings.metric_len.to_formatted_string(&Locale::en),
                        timings.pct_995_ms,
                        timings.median_ms
                    )))
                    .expect("todo: result handling");
            }
        }

        self.out.flush().expect("todo: result handling");
    }
}
