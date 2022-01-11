use crate::config::types::FatigueStaticContextAction;
use crate::context::helpers::{context_validation_error, to_ok_context_result};
use crate::context::{
    StaticContextAction, StaticContextActionBuilder, StaticContextActionBuilderError,
    StaticContextResult,
};
use crate::{FatigueTesterRunInformation, StaticContextActionPointer};
use serde_json::Value;
use std::fs::File;
use std::sync::Arc;

pub fn get_builder() -> Box<dyn StaticContextActionBuilder> {
    Box::new(JsonContextActionBuilder {})
}

#[derive(Deserialize, Debug)]
struct JsonContextActionProps {
    pub path: Option<String>,
    pub raw: Option<String>,
}

struct JsonContextAction {
    props: JsonContextActionProps,
    name: String,
}

impl JsonContextAction {
    fn new(props: JsonContextActionProps, name: String) -> Self {
        JsonContextAction { props, name }
    }
}

#[async_trait]
impl StaticContextAction for JsonContextAction {
    async fn execute(&self) -> StaticContextResult {
        let res = match &self.props.path {
            None => match &self.props.raw {
                None => {
                    panic!("neither path or raw set, this should've been pre-validated")
                }
                Some(raw) => {
                    let res = serde_json::from_str(raw.as_str())?;
                    res
                }
            },
            Some(path) => {
                let reader = File::open(path.as_str())?;
                let res: Value = serde_json::from_reader(reader)?;
                res
            }
        };

        to_ok_context_result(self, &res)
    }

    async fn should_refresh(&self) -> bool {
        false
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

struct JsonContextActionBuilder {}

impl StaticContextActionBuilder for JsonContextActionBuilder {
    fn get_type_name(&self) -> &'static str {
        "json"
    }

    fn build(
        &self,
        action_info: &FatigueStaticContextAction,
        _run_info: &FatigueTesterRunInformation,
    ) -> Result<StaticContextActionPointer, StaticContextActionBuilderError> {
        let props: JsonContextActionProps = serde_yaml::from_value(action_info.properties.clone())?;
        let name = &action_info
            .name
            .clone()
            .unwrap_or_else(|| String::from("json"));

        if props.path.is_none() && props.raw.is_none() {
            return context_validation_error(self, "must set either path or raw");
        } else if props.path.is_some() && props.raw.is_some() {
            return context_validation_error(self, "cannot set both path and raw");
        }

        Ok(Arc::new(JsonContextAction::new(props, name.clone())))
    }
}
