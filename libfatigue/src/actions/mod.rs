use crate::context::IterationContext;
use crate::factories::ActionFactoryError;
use crate::util::rand_action_name;
use crate::{ActionFactory, FatigueTesterRunInformation};
use reqwest::StatusCode;
use serde_yaml::Value;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

pub mod delay;
pub mod print;
pub mod request;

pub fn register_default_actions(factory: &mut ActionFactory) -> Result<(), ActionFactoryError> {
    factory.register_builder(request::new_builder())?;
    factory.register_builder(delay::new_builder())?;
    factory.register_builder(print::new_builder())?;

    Ok(())
}

pub type ActionResult = Result<ActionExecutionInfo, ActionError>;

/// The result of a successful action
#[derive(Debug)]
pub enum ActionExecutionInfo {
    /// The action executed, but we don't want to measure perf
    Ok,
    /// An http call was made
    HttpCall {
        timing: Duration,
        status_code: StatusCode,
        url: Url,
    },
    // multiple actions were performed
    Multi {
        items: Vec<ActionExecutionInfo>,
    },
}

#[async_trait]
pub trait Action {
    fn suggest_default_name(&self) -> Option<String>;
    async fn execute(&self, context: &mut IterationContext) -> ActionResult;
}

pub(crate) struct InternalAction {
    internal: ActionPointer,
    name: Arc<String>,
}

impl InternalAction {
    pub fn new(internal: ActionPointer, name: Option<String>) -> Self {
        let name = match name {
            Some(n) => n,
            None => internal
                .suggest_default_name()
                .unwrap_or(rand_action_name()),
        };
        let name = Arc::new(name);

        InternalAction { internal, name }
    }

    pub async fn execute(&self, context: &mut IterationContext) -> InternalActionResult {
        let internal = self.internal.execute(context).await;
        let name = self.name.clone();

        InternalActionResult { internal, name }
    }
}

#[derive(Debug)]
pub(crate) struct InternalActionResult {
    pub internal: ActionResult,
    pub name: Arc<String>,
}

pub type ActionPointer = Box<dyn Action + Send + Sync>;

#[derive(Debug, Error)]
pub enum ActionError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Templating(#[from] liquid::Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),
}

/// A builder that constructs Action instances
/// from their loosely typed config values
pub trait ActionBuilder: Send + Sync {
    /// Gets the type name of the action this builder supports
    ///
    /// note: in the builder pipeline, these are mapped to config with
    /// String.tolower to improve usability. It's important that the type name
    /// provided is all lowercase
    fn get_type_name(&self) -> &'static str;

    /// Build an action, given the loosely typed properties
    fn build(
        &self,
        properties: &Value,
        run_info: &FatigueTesterRunInformation,
    ) -> Result<ActionPointer, ActionBuilderError>;
}

/// A helper to build an error when field validation failed on an action builder
pub fn action_validation_error<S: Into<String>>(
    builder: &impl ActionBuilder,
    msg: S,
) -> Result<ActionPointer, ActionBuilderError> {
    Err(ActionBuilderError::ValidationError(
        builder.get_type_name(),
        msg.into(),
    ))
}

/// Errors for action builders
#[derive(Debug, Error)]
pub enum ActionBuilderError {
    #[error("error deserializing properties: `{0}`")]
    YamlDeserializationError(#[from] serde_yaml::Error),
    #[error("error building action `{0}`: `{1}`")]
    ValidationError(&'static str, String),
    #[error("error parsing url: `{0}`")]
    UrlParse(#[from] url::ParseError),
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Templating(#[from] liquid::Error),
}
