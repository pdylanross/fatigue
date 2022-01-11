use crate::context::{
    ContextMap, StaticContext, StaticContextAction, StaticContextActionBuilder,
    StaticContextActionBuilderError, StaticContextResult,
};
use crate::StaticContextActionPointer;
use liquid::model::to_value;
use serde::Serialize;
use std::collections::HashMap;

pub fn to_ok_context_result<T>(me: &impl StaticContextAction, val: &T) -> StaticContextResult
where
    T: Serialize,
{
    let mut map: ContextMap = HashMap::new();

    let val = to_value(val)?;
    map.insert(me.get_name().to_string(), val);
    let res = StaticContext { items: map };

    Ok(res)
}

/// A helper to build an error when field validation failed on a context action builder
pub fn context_validation_error<S: Into<String>>(
    builder: &impl StaticContextActionBuilder,
    msg: S,
) -> Result<StaticContextActionPointer, StaticContextActionBuilderError> {
    Err(StaticContextActionBuilderError::ValidationError(
        builder.get_type_name(),
        msg.into(),
    ))
}
