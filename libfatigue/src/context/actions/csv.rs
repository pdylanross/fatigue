use crate::config::types::FatigueStaticContextAction;
use crate::context::helpers::to_ok_context_result;
use crate::context::{
    StaticContextAction, StaticContextActionBuilder, StaticContextActionBuilderError,
    StaticContextError, StaticContextResult,
};
use crate::{FatigueTesterRunInformation, StaticContextActionPointer};
use csv::{Reader, ReaderBuilder};
use std::collections::HashMap;
use std::fs::File;
use std::sync::Arc;

pub fn get_builder() -> Box<dyn StaticContextActionBuilder> {
    Box::new(CsvContextActionBuilder {})
}

#[derive(Deserialize, Debug)]
struct CsvContextActionProps {
    pub path: String,

    /// if the csv has headers. Default true
    #[serde(default)]
    pub has_headers: Option<bool>,

    /// the separator character for the csv. Default ','
    #[serde(default)]
    pub separator: Option<char>,
}

struct CsvContextAction {
    props: CsvContextActionProps,
    name: String,
}

#[async_trait]
impl StaticContextAction for CsvContextAction {
    async fn execute(&self) -> StaticContextResult {
        let file = File::open(self.props.path.as_str())?;
        let mut builder = ReaderBuilder::new();

        let has_headers = if self.props.has_headers.unwrap_or(true) {
            builder.has_headers(true);
            true
        } else {
            builder.has_headers(false);
            false
        };

        if let Some(separator) = self.props.separator {
            // is a cast to u8 correct here?
            builder.delimiter(separator as u8);
        }

        let reader = builder.from_reader(file);
        let results = if has_headers {
            self.parse_with_headers(reader)?
        } else {
            self.parse_without_headers(reader)?
        };

        to_ok_context_result(self, &results)
    }

    async fn should_refresh(&self) -> bool {
        false
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl CsvContextAction {
    fn parse_with_headers(
        &self,
        mut reader: Reader<File>,
    ) -> Result<Vec<HashMap<String, String>>, StaticContextError> {
        let mut results: Vec<HashMap<String, String>> = Vec::new();
        let mut result_iter = reader.deserialize();

        for row in result_iter.by_ref() {
            let row = row?;
            results.push(row);
        }

        Ok(results)
    }

    fn parse_without_headers(
        &self,
        mut reader: Reader<File>,
    ) -> Result<Vec<HashMap<String, String>>, StaticContextError> {
        let mut results: Vec<HashMap<String, String>> = Vec::new();
        let mut result_iter = reader.deserialize();
        for row in result_iter.by_ref() {
            let row: Vec<String> = row?;
            let mut row_result = HashMap::new();
            for (pos, val) in row.iter().enumerate() {
                row_result.insert(pos.to_string(), val.clone());
            }

            results.push(row_result);
        }

        Ok(results)
    }
}

struct CsvContextActionBuilder {}

impl StaticContextActionBuilder for CsvContextActionBuilder {
    fn get_type_name(&self) -> &'static str {
        "csv"
    }

    fn build(
        &self,
        action_info: &FatigueStaticContextAction,
        _run_info: &FatigueTesterRunInformation,
    ) -> Result<StaticContextActionPointer, StaticContextActionBuilderError> {
        let props: CsvContextActionProps = serde_yaml::from_value(action_info.properties.clone())?;
        let name = action_info
            .name
            .clone()
            .unwrap_or_else(|| String::from("csv"));

        Ok(Arc::new(CsvContextAction { props, name }))
    }
}
