use crate::output::OutputFormatter;
use console::Style;
use crossterm::cursor::{Hide, MoveDown, MoveTo};
use crossterm::style::{Attribute, Print, SetAttribute};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, QueueableCommand};
use humantime::format_duration;
use itertools::Itertools;
use libfatigue::context::{TestDurationStatus, TestResult};
use libfatigue::FatigueTestError;
use num_format::{Locale, ToFormattedString};
use prettytable::format::consts::FORMAT_CLEAN;
use prettytable::format::Alignment;
use prettytable::{row, Cell, Row, Table};
use std::io::{stdout, Stdout, Write};
use std::sync::Mutex;
use std::time::Duration;

pub(crate) struct PrettyOutputFormatter {
    inner: Mutex<PrettyInner>,
}

struct PrettyInner {
    cyan: Style,
    blue: Style,
    _red: Style,
    yellow: Style,
    _green: Style,
    out: Stdout,
}

impl PrettyOutputFormatter {
    pub fn new() -> Self {
        let mut out = stdout();
        out.execute(Hide).expect("todo: result handling");
        out.queue(Clear(ClearType::All))
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
            yellow,
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
        guard.clear();
        guard.write(&result);
    }

    fn write_err(&self, err: FatigueTestError) {
        let mut guard = self.inner.lock().unwrap();
        guard.write_err(err);
    }
}

impl PrettyInner {
    fn clear(&mut self) {
        self.out.queue(MoveTo(0, 0)).expect("todo: result handling");
        self.out
            .queue(Clear(ClearType::FromCursorDown))
            .expect("todo: result handling");
    }

    fn write_err(&mut self, err: FatigueTestError) {
        self.clear();
        self.out.queue(Print(err)).expect("todo: result handling");
        self.out.flush().expect("todo: result handling");
    }

    fn write(&mut self, val: &TestResult) {
        self.out.queue(MoveTo(0, 0)).expect("todo: result handling");

        self.display_status_row(val);
        self.out
            .queue(Clear(ClearType::FromCursorDown))
            .expect("todo: result handling");
        let table = self.build_output_table(val);
        self.out.queue(Print(table)).expect("todo: result handling");
        self.out.flush().expect("todo: result handling");
    }

    fn display_status_row(&mut self, val: &TestResult) {
        let rps = format!(
            "rps: {:.2}\n",
            self.yellow.apply_to(val.requests_per_second)
        );
        let mut rps = Cell::new(rps.as_str());
        rps.align(Alignment::LEFT);

        let remaining = match &val.duration {
            None => String::new(),
            Some(duration_status) => match duration_status {
                TestDurationStatus::Iteration { current, until } => {
                    format!(
                        "Iterations: {} / {}",
                        current.to_formatted_string(&Locale::en),
                        until.to_formatted_string(&Locale::en)
                    )
                }
                TestDurationStatus::Timed { remaining } => {
                    let cleansed = Duration::new(remaining.as_secs(), 0);
                    let readable = format_duration(cleansed);

                    format!("Remaining: {}", readable)
                }
            },
        };
        let mut remaining = Cell::new(remaining.as_str());
        remaining.align(Alignment::RIGHT);

        let mut table = Table::new();
        table.set_format(*FORMAT_CLEAN);

        table.add_row(Row::new(vec![rps, remaining]));

        self.out
            .queue(SetAttribute(Attribute::Bold))
            .expect("todo: result handling");
        self.out.queue(Print(table)).expect("todo: result handling");
        self.out
            .queue(SetAttribute(Attribute::Reset))
            .expect("todo: result handling");
        self.out.queue(MoveDown(1)).expect("todo: result handling");
    }

    fn build_output_table(&self, val: &TestResult) -> Table {
        let mut table = Table::new();
        table.set_format(*FORMAT_CLEAN);
        table.set_titles(row![
            "Test",
            "Status",
            "# Requests",
            "Median ms",
            "95th % ms",
            "99.999% ms"
        ]);

        for test_name in val.timings.keys().sorted() {
            table.add_row(row![self.cyan.apply_to(test_name.as_str())]);
            let status_map = &val.timings[test_name];
            for status_code in status_map.keys().sorted() {
                let timings = &status_map[status_code];
                table.add_row(row![
                    "",
                    self.blue.apply_to(&status_code.as_str()),
                    timings.metric_len.to_formatted_string(&Locale::en),
                    timings.median_ms,
                    timings.pct_995_ms,
                    timings.pct_99999_ms
                ]);
            }
        }

        table
    }
}
