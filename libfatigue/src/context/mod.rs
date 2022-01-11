use crate::actions::InternalActionResult;
use crate::config::types::{FatigueStaticContextAction, RunDuration};
use crate::context::iteration::{IterationDurationTracker, TimedDurationTracker};
use crate::context::result::TestResultBuilder;
use crate::FatigueTesterRunInformation;
use dashmap::DashMap;
use hdrhistogram::Histogram;
use liquid::model::{to_value, Value};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

pub mod actions;
mod iteration;
mod result;

pub type ContextMap = HashMap<String, Value>;
pub type StaticContextActionPointer = Arc<dyn StaticContextAction + Send + Sync>;
pub type StaticContextResult = Result<StaticContext, StaticContextError>;

#[async_trait]
pub trait StaticContextAction {
    async fn execute(&self) -> StaticContextResult;
    async fn should_refresh(&self) -> bool;
    fn get_name(&self) -> &str;
}

pub trait StaticContextActionBuilder {
    fn get_type_name(&self) -> &'static str;
    fn build(
        &self,
        action_info: &FatigueStaticContextAction,
        run_info: &FatigueTesterRunInformation,
    ) -> Result<StaticContextActionPointer, StaticContextActionBuilderError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaticContext {
    pub items: ContextMap,
}

#[derive(Debug, Error)]
pub enum StaticContextError {
    #[error("templating error: {0}")]
    TemplateError(#[from] liquid::Error),
    #[error("error deserializing properties: {0}")]
    YamlDeserializationError(#[from] serde_yaml::Error),
    #[error("error deserializing json: {0}")]
    JsonDeserialization(#[from] serde_json::Error),
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}
#[derive(Debug, Error)]
pub enum StaticContextActionBuilderError {
    #[error("error deserializing properties: {0}")]
    YamlDeserializationError(#[from] serde_yaml::Error),
    #[error("error building action {0}: {1}")]
    ValidationError(&'static str, String),
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

#[derive(Debug)]
pub enum IterationContextItem {}

#[derive(Debug)]
pub struct IterationContext {
    pub items: ContextMap,
}

#[derive(Debug, Error)]
pub enum IterationContextError {
    #[error("error from static context {0}: {1}")]
    StaticContext(String, StaticContextError),
}

#[derive(Debug)]
pub(crate) enum IterationResult {
    Ok {
        actions: Vec<InternalActionResult>,
        context: IterationContext,
    },
}

#[derive(Serialize, Debug, Default, Clone)]
pub struct TestResult {
    pub timings: HashMap<String, HashMap<String, TestResultTimingLogItem>>,
    pub requests_per_second: f64,
    pub duration: Option<TestDurationStatus>,
}

#[derive(Serialize, Debug, Clone)]
pub enum TestDurationStatus {
    Iteration { until: u64, current: u64 },
    Timed { remaining: Duration },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestResultTimingLogItem {
    pub metric_len: u64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub mean_ms: f64,
    pub median_ms: f64,
    pub std_dev_ms: f64,
    pub pct_99999_ms: f64,
    pub pct_995_ms: f64,
    pub pct_95_ms: f64,
    pub pct_75_ms: f64,
}

impl TestResultTimingLogItem {
    fn map_from_histogram(hist: &Histogram<u64>) -> Self {
        let mean_ms = hist.mean() / 1000.0;
        let median_ms = hist.value_at_quantile(0.5) as f64 / 1000.0;
        let std_dev_ms = hist.stdev() / 1000.0;
        let pct_99999_ms = hist.value_at_quantile(0.99999) as f64 / 1000.0;
        let pct_995_ms = hist.value_at_quantile(0.995) as f64 / 1000.0;
        let pct_95_ms = hist.value_at_quantile(0.95) as f64 / 1000.0;
        let pct_75_ms = hist.value_at_quantile(0.75) as f64 / 1000.0;
        let metric_len = hist.len();
        let min_ms = hist.min() as f64 / 1000.0;
        let max_ms = hist.max() as f64 / 1000.0;
        TestResultTimingLogItem {
            mean_ms,
            median_ms,
            std_dev_ms,
            pct_99999_ms,
            pct_995_ms,
            pct_95_ms,
            pct_75_ms,
            metric_len,
            min_ms,
            max_ms,
        }
    }
}

pub(crate) struct TestRunContext {
    pub info: Arc<FatigueTesterRunInformation>,
    static_contexts: DashMap<String, StaticContextTracker>,
    duration_tracker: Arc<dyn TestDurationTracker + Send + Sync>,
    result_builder: Arc<TestResultBuilder>,
}

impl TestRunContext {
    pub(crate) fn new(
        info: Arc<FatigueTesterRunInformation>,
        context_actions: Arc<Vec<StaticContextActionPointer>>,
    ) -> Self {
        let duration_tracker = get_duration_tracker(&info);
        let result_builder = Arc::new(TestResultBuilder::new());
        let static_contexts: DashMap<String, StaticContextTracker> = context_actions
            .iter()
            .cloned()
            .map(|a| StaticContextTracker::new(a))
            .map(|a| (a.action.get_name().to_string(), a))
            .collect();

        TestRunContext {
            info,
            static_contexts,
            duration_tracker,
            result_builder,
        }
    }

    pub(crate) async fn new_iteration_ctx(
        &self,
    ) -> Result<IterationContext, IterationContextError> {
        let mut items = HashMap::with_capacity(self.static_contexts.len());
        for v in &self.static_contexts {
            let name = v.key().clone();
            if let Err(e) = self.write_static_context(&mut items, name.as_str()).await {
                return Err(IterationContextError::StaticContext(name, e));
            }
        }

        Ok(IterationContext { items })
    }

    async fn write_static_context(
        &self,
        items: &mut HashMap<String, Value>,
        key: &str,
    ) -> Result<(), StaticContextError> {
        let v = self
            .static_contexts
            .get(key)
            .expect("should be called from an iterator");
        let v = v.value().get_val().await?;
        for (k, v) in v.items {
            items.insert(k.clone(), to_value(&v)?);
        }

        Ok(())
    }

    pub(crate) fn is_not_done(&self) -> bool {
        !self.duration_tracker.is_done()
    }

    pub(crate) async fn mark_iteration(&self, result: IterationResult) {
        if self.duration_tracker.should_track_iteration().await {
            self.result_builder.mark_iteration(result).await;
        }
        self.duration_tracker.mark_iteration().await;
    }

    pub(crate) async fn get_test_results(&self) -> TestResult {
        self.result_builder.build().await
    }
    pub(crate) fn get_duration_status(&self) -> TestDurationStatus {
        self.duration_tracker.get_status()
    }

    pub(crate) fn _mark_exit(&self) {
        self.duration_tracker.mark_exit()
    }
}

struct StaticContextTracker {
    action: StaticContextActionPointer,
    val: RwLock<Option<StaticContext>>,
}

impl StaticContextTracker {
    fn new(action: StaticContextActionPointer) -> Self {
        let val = RwLock::new(None);
        StaticContextTracker { action, val }
    }

    async fn get_val(&self) -> StaticContextResult {
        {
            let val = self.val.read().await;
            match val.as_ref() {
                None => {}
                Some(val) => {
                    if !self.action.should_refresh().await {
                        return Ok(val.clone());
                    }
                }
            }
        }

        {
            let mut val = self.val.write().await;

            if val.is_none() || self.action.should_refresh().await {
                let new_val = self.action.execute().await?;
                *val = Some(new_val);
            }

            match val.as_ref() {
                None => {
                    panic!("val should be some by now");
                }
                Some(v) => Ok(v.clone()),
            }
        }
    }
}

#[async_trait]
pub(crate) trait TestDurationTracker {
    async fn mark_iteration(&self);
    fn mark_exit(&self);
    fn is_done(&self) -> bool;
    async fn should_track_iteration(&self) -> bool;
    fn get_status(&self) -> TestDurationStatus;
}

fn get_duration_tracker(
    info: &Arc<FatigueTesterRunInformation>,
) -> Arc<dyn TestDurationTracker + Send + Sync> {
    match info.duration {
        RunDuration::Iteration {
            warm_up,
            iterations,
        } => Arc::new(IterationDurationTracker::new(warm_up, iterations)),
        RunDuration::Timed {
            duration: until,
            warm_up,
        } => Arc::new(TimedDurationTracker::new(until, warm_up)),
    }
}
