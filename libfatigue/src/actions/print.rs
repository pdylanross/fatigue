use crate::actions::{
    Action, ActionBuilder, ActionBuilderError, ActionExecutionInfo, ActionResult,
};
use crate::context::IterationContext;
use crate::{ActionPointer, FatigueTesterRunInformation};
use liquid::to_object;
use serde_yaml::Value;

struct PrintContextAction {}

#[async_trait]
impl Action for PrintContextAction {
    fn suggest_default_name(&self) -> Option<String> {
        None
    }

    async fn execute(&self, context: &mut IterationContext) -> ActionResult {
        let context_object = to_object(&context.items)?;
        let serialized_ctx = serde_yaml::to_string(&context_object)?;
        println!("{}", serialized_ctx);
        Ok(ActionExecutionInfo::Ok)
    }
}

struct PrintContextActionBuilder {}

impl ActionBuilder for PrintContextActionBuilder {
    fn get_type_name(&self) -> &'static str {
        "print_context"
    }

    fn build(
        &self,
        _properties: &Value,
        _run_info: &FatigueTesterRunInformation,
    ) -> Result<ActionPointer, ActionBuilderError> {
        Ok(Box::new(PrintContextAction {}))
    }
}

pub fn new_builder() -> Box<dyn ActionBuilder> {
    Box::new(PrintContextActionBuilder {})
}
