use crate::actions::{
    action_validation_error, Action, ActionBuilder, ActionBuilderError, ActionExecutionInfo,
    ActionResult,
};
use crate::context::IterationContext;
use crate::{ActionPointer, FatigueTesterRunInformation};
use serde_yaml::Value;
use std::time::Duration;

struct DelayAction {
    delay: Duration,
}

#[async_trait]
impl Action for DelayAction {
    fn suggest_default_name(&self) -> Option<String> {
        None
    }

    async fn execute(&self, _context: &mut IterationContext) -> ActionResult {
        tokio::time::sleep(self.delay).await;
        Ok(ActionExecutionInfo::Ok)
    }
}

#[derive(Deserialize, Serialize)]
struct DelayActionProperties {
    milliseconds: Option<u64>,
    seconds: Option<u64>,
}

struct DelayActionBuilder {}

impl ActionBuilder for DelayActionBuilder {
    fn get_type_name(&self) -> &'static str {
        "delay"
    }

    fn build(
        &self,
        properties: &Value,
        _ri: &FatigueTesterRunInformation,
    ) -> Result<ActionPointer, ActionBuilderError> {
        let props: DelayActionProperties = serde_yaml::from_value(properties.clone())?;
        let delay = if props.milliseconds.is_some() {
            Duration::from_millis(props.milliseconds.unwrap())
        } else if props.seconds.is_some() {
            Duration::from_secs(props.seconds.unwrap())
        } else {
            return action_validation_error(self, "milliseconds or seconds must be set");
        };

        Ok(Box::new(DelayAction { delay }))
    }
}

/// create a new delay action builder
pub fn new_builder() -> Box<dyn ActionBuilder> {
    Box::new(DelayActionBuilder {})
}
