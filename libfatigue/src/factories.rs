use crate::actions::{ActionBuilder, ActionBuilderError, InternalAction};
use crate::config::types::{
    FatigueStaticContextAction, FatigueTesterConfig, FatigueTesterConfigAction,
};
use crate::context::{
    StaticContextActionBuilder, StaticContextActionBuilderError, StaticContextActionPointer,
};
use crate::FatigueTesterRunInformation;
use std::collections::HashMap;

#[derive(Default)]
pub struct ActionFactory {
    builders: HashMap<&'static str, Box<dyn ActionBuilder>>,
}

#[derive(Debug, Error)]
pub enum ActionFactoryError {
    #[error("builder with type name `{0}` already exists")]
    DuplicateRegistration(&'static str),
    #[error("action type `{0}` not found")]
    ActionTypeNotFound(String),
    #[error("action builder error: `{0}`")]
    ActionBuilderError(#[from] ActionBuilderError),
}

impl ActionFactory {
    pub fn register_builder(
        &mut self,
        builder: Box<dyn ActionBuilder>,
    ) -> Result<(), ActionFactoryError> {
        let type_name = builder.get_type_name();
        if self.builders.contains_key(type_name) {
            Err(ActionFactoryError::DuplicateRegistration(type_name))
        } else {
            self.builders.insert(type_name, builder);
            Ok(())
        }
    }

    pub(crate) fn build(
        &self,
        action: &FatigueTesterConfigAction,
        run_info: &FatigueTesterRunInformation,
    ) -> Result<InternalAction, ActionFactoryError> {
        let sanitized_type = action.action_type.to_lowercase();
        if self.builders.contains_key(sanitized_type.as_str()) {
            let builder = self.builders.get(sanitized_type.as_str()).unwrap();
            let full = InternalAction::new(
                builder.build(&action.action_properties, run_info)?,
                action.name.clone(),
            );
            Ok(full)
        } else {
            Err(ActionFactoryError::ActionTypeNotFound(
                action.action_type.clone(),
            ))
        }
    }

    pub(crate) fn build_all(
        &self,
        config: &FatigueTesterConfig,
    ) -> Result<Vec<InternalAction>, ActionFactoryError> {
        let mut res = Vec::with_capacity(config.actions.len());
        for item in config.actions.iter() {
            res.push(self.build(item, &config.run_info)?);
        }

        Ok(res)
    }
}

#[derive(Default)]
pub struct ContextActionFactory {
    builders: HashMap<&'static str, Box<dyn StaticContextActionBuilder>>,
}

#[derive(Debug, Error)]
pub enum ContextActionFactoryError {
    #[error("context builder with type name `{0}` already exists")]
    DuplicateRegistration(&'static str),
    #[error("context action type `{0}` not found")]
    StaticContextActionTypeNotFound(String),
    #[error("context action builder error: `{0}`")]
    StaticContextActionBuilderError(#[from] StaticContextActionBuilderError),
}

impl ContextActionFactory {
    pub fn register_builder(
        &mut self,
        builder: Box<dyn StaticContextActionBuilder>,
    ) -> Result<(), ContextActionFactoryError> {
        let type_name = builder.get_type_name();
        if self.builders.contains_key(type_name) {
            Err(ContextActionFactoryError::DuplicateRegistration(type_name))
        } else {
            self.builders.insert(type_name, builder);
            Ok(())
        }
    }

    pub(crate) fn build(
        &self,
        action: &FatigueStaticContextAction,
        run_info: &FatigueTesterRunInformation,
    ) -> Result<StaticContextActionPointer, ContextActionFactoryError> {
        let sanitized_type = action.context_action_type.to_lowercase();
        if self.builders.contains_key(sanitized_type.as_str()) {
            let builder = self.builders.get(sanitized_type.as_str()).unwrap();
            let full = builder.build(action, run_info)?;
            Ok(full)
        } else {
            Err(ContextActionFactoryError::StaticContextActionTypeNotFound(
                action.context_action_type.clone(),
            ))
        }
    }

    pub(crate) fn build_all(
        &self,
        config: &FatigueTesterConfig,
    ) -> Result<Vec<StaticContextActionPointer>, ContextActionFactoryError> {
        let mut res = Vec::with_capacity(config.actions.len());
        for item in config.static_context.iter() {
            res.push(self.build(item, &config.run_info)?);
        }

        Ok(res)
    }
}
