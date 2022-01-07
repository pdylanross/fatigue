use crate::config::types::FatigueStaticContextAction;
use crate::context::{
    context_validation_error, ContextMap, StaticContext, StaticContextAction,
    StaticContextActionBuilder, StaticContextActionBuilderError, StaticContextResult,
};
use crate::{FatigueTesterRunInformation, StaticContextActionPointer};
use liquid::model::to_value;
use serde_yaml::Value;
use std::collections::HashMap;
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
        let mut map: ContextMap = HashMap::new();

        let val = to_value(&self.props.value)?;
        map.insert(self.name.clone(), val);
        let res = StaticContext { items: map };

        Ok(res)
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
