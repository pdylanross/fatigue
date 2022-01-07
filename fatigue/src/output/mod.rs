use crate::output::pretty::PrettyOutputFormatter;
use libfatigue::context::TestResult;
use libfatigue::FatigueTestError;

mod pretty;

pub(crate) trait OutputFormatter {
    fn update_result_status(&self, result: &TestResult);
    fn write_final_results(&self, results: TestResult);
    fn write_err(&self, err: FatigueTestError);
    fn tick(&self);
}

pub(crate) fn get_output_formatter() -> Box<dyn OutputFormatter> {
    Box::new(PrettyOutputFormatter::new())
}
