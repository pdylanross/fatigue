use crate::actions::{
    Action, ActionBuilder, ActionBuilderError, ActionError, ActionExecutionInfo, ActionResult,
};
use crate::context::IterationContext;
use crate::{ActionPointer, FatigueTesterRunInformation};
use liquid::{to_object, Object, ParserBuilder, Template};
use reqwest::{Client, Method, Url};
use serde_yaml::Value;
use std::time::Instant;

/// create a new request action builder
pub fn new_builder() -> Box<dyn ActionBuilder> {
    Box::new(RequestActionBuilder {})
}

struct RequestAction {
    client: Client,
    url: Url,
    props: RequestActionProperties,
    path_template: Template,
    body_template: Option<Template>,
    method: Method,
}

#[async_trait]
impl Action for RequestAction {
    fn suggest_default_name(&self) -> Option<String> {
        Some(self.props.path.clone())
    }

    async fn execute(&self, context: &mut IterationContext) -> ActionResult {
        let context_object = to_object(&context.items)?;
        let rendered_path = self.path_template.render(&context_object)?;
        let rendered_path_lines: Vec<_> = rendered_path
            .lines()
            .into_iter()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .collect();

        if rendered_path_lines.len() == 1 {
            Ok(self
                .execute_with_body(rendered_path_lines[0], context, &context_object)
                .await?)
        } else {
            let mut items = Vec::with_capacity(rendered_path_lines.len());
            for p in rendered_path_lines.into_iter() {
                items.push(self.execute_with_body(p, context, &context_object).await?)
            }

            Ok(ActionExecutionInfo::Multi { items })
        }
    }
}

impl RequestAction {
    async fn execute_with_body(
        &self,
        path: &str,
        ctx: &mut IterationContext,
        context_object: &Object,
    ) -> Result<ActionExecutionInfo, ActionError> {
        // todo: determine some way to do multi-body splitting
        let body = match &self.body_template {
            None => None,
            Some(tpl) => Some(tpl.render(context_object)?),
        };

        self.execute_single_call(path, body, ctx).await
    }

    async fn execute_single_call(
        &self,
        path: &str,
        body: Option<String>,
        ctx: &mut IterationContext,
    ) -> Result<ActionExecutionInfo, ActionError> {
        let url = self.url.join(path)?;
        let mut req = self.client.request(self.method.clone(), url.clone());

        if let Some(body) = body {
            req = req.body(body);
        }

        let start = Instant::now();
        let resp = req.send().await?;
        let end = Instant::now() - start;

        let status_code = resp.status();

        match &self.props.response_context_key {
            Some(key) => {
                let body = &resp.json::<Value>().await?;
                ctx.items
                    .insert(key.clone(), liquid::model::to_value(&body).unwrap());
            }
            None => {}
        };

        Ok(ActionExecutionInfo::HttpCall {
            timing: end,
            status_code,
            url,
        })
    }
}

#[derive(Serialize, Deserialize)]
struct RequestActionProperties {
    pub path: String,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub response_context_key: Option<String>,
}

struct RequestActionBuilder {}

impl ActionBuilder for RequestActionBuilder {
    fn get_type_name(&self) -> &'static str {
        "request"
    }

    fn build(
        &self,
        properties: &Value,
        run_info: &FatigueTesterRunInformation,
    ) -> Result<ActionPointer, ActionBuilderError> {
        let props: RequestActionProperties = serde_yaml::from_value(properties.clone())?;
        let url = Url::parse(run_info.base_url.as_str())?;
        let client = Client::builder().build()?;

        let template_parser = ParserBuilder::with_stdlib().build()?;
        let path_template = template_parser.parse(props.path.as_str())?;
        let method = self.get_request_method(&props)?;
        let body_template = match &props.body {
            None => None,
            Some(body) => Some(template_parser.parse(body.as_str())?),
        };

        Ok(Box::new(RequestAction {
            props,
            url,
            client,
            path_template,
            body_template,
            method,
        }))
    }
}

impl RequestActionBuilder {
    fn get_request_method(
        &self,
        prop: &RequestActionProperties,
    ) -> Result<Method, ActionBuilderError> {
        match &prop.method {
            None => Ok(Method::GET),
            Some(val) => {
                let lower = val.to_lowercase();
                match lower.as_str() {
                    "get" => Ok(Method::GET),
                    "post" => Ok(Method::POST),
                    "put" => Ok(Method::PUT),
                    "path" => Ok(Method::PATCH),
                    "delete" => Ok(Method::DELETE),
                    _ => Err(ActionBuilderError::ValidationError(
                        self.get_type_name(),
                        format!("unknown http request method: {}", lower),
                    )),
                }
            }
        }
    }
}
