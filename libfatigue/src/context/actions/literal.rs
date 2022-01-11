use crate::config::types::FatigueStaticContextAction;
use crate::context::helpers::{context_validation_error, to_ok_context_result};
use crate::context::{
    StaticContextAction, StaticContextActionBuilder, StaticContextActionBuilderError,
    StaticContextResult,
};
use crate::{FatigueTesterRunInformation, StaticContextActionPointer};
use serde_yaml::Value;
use std::sync::Arc;

struct LiteralContextAction {
    props: LiteralContextActionProps,
    name: String,
}

struct LiteralContextActionProps {
    pub value: Value,
}

#[async_trait]
impl StaticContextAction for LiteralContextAction {
    async fn execute(&self) -> StaticContextResult {
        to_ok_context_result(self, &self.props.value)
    }

    async fn should_refresh(&self) -> bool {
        false
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

struct LiteralContextActionBuilder {}

impl StaticContextActionBuilder for LiteralContextActionBuilder {
    fn get_type_name(&self) -> &'static str {
        "literal"
    }

    fn build(
        &self,
        action_info: &FatigueStaticContextAction,
        _run_info: &FatigueTesterRunInformation,
    ) -> Result<StaticContextActionPointer, StaticContextActionBuilderError> {
        let name = match &action_info.name {
            None => {
                return context_validation_error(self, "literal context action requires a name")
            }
            Some(name) => name.clone(),
        };

        Ok(Arc::new(LiteralContextAction {
            props: LiteralContextActionProps {
                value: action_info.properties.clone(),
            },
            name,
        }))
    }
}

pub fn get_builder() -> Box<dyn StaticContextActionBuilder> {
    Box::new(LiteralContextActionBuilder {})
}
