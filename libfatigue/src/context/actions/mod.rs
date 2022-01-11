use crate::{ContextActionFactory, ContextActionFactoryError};

pub mod json;
pub mod literal;

pub fn register_default_context_actions(
    factory: &mut ContextActionFactory,
) -> Result<(), ContextActionFactoryError> {
    factory.register_builder(literal::get_builder())?;
    factory.register_builder(json::get_builder())?;
    Ok(())
}
