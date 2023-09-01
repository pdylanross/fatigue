use crate::OutputFormatter;
use libfatigue::context::TestResult;
use libfatigue::TestRunSettings;
use std::fs::File;
use std::path::PathBuf;
use tokio::task::JoinHandle;

#[derive(Default)]
pub(crate) struct JsonOutputFormatter {}

impl OutputFormatter for JsonOutputFormatter {
    fn begin(&self, _settings: &mut TestRunSettings) -> JoinHandle<()> {
        tokio::spawn(async {})
    }

    fn update_result_status(&self, _result: &TestResult) {}

    fn write_final_results(&self, results: TestResult) {
        let json = serde_json::to_string_pretty(&results).expect("todo: error handling");
        println!("{}", json);
    }
}

#[derive(Default)]
pub(crate) struct JsonFileFormatter {
    pub(crate) path: PathBuf,
}

impl OutputFormatter for JsonFileFormatter {
    fn begin(&self, _settings: &mut TestRunSettings) -> JoinHandle<()> {
        tokio::spawn(async {})
    }

    fn update_result_status(&self, _result: &TestResult) {}

    fn write_final_results(&self, results: TestResult) {
        println!("{:#?}", self.path);
        let file = File::create(&self.path).expect("file open");

        serde_json::to_writer_pretty(&file, &results).expect("todo: jsonfile error handling");
    }
}
