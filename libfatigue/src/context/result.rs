use crate::actions::{ActionExecutionInfo, InternalActionResult};
use crate::context::{IterationContext, IterationResult, TestResultTimingLogItem};
use crate::TestResult;
use hdrhistogram::Histogram;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::Mutex;

pub(crate) struct TestResultBuilder {
    inner: Mutex<TestResultBuilderInner>,
}

impl TestResultBuilder {
    pub fn new() -> Self {
        TestResultBuilder {
            inner: Mutex::new(TestResultBuilderInner::new()),
        }
    }

    pub async fn mark_iteration(&self, iteration: IterationResult) {
        match iteration {
            IterationResult::Ok { actions, context } => {
                self.mark_success_iteration(actions, context).await;
            }
            IterationResult::ContextError { err } => {
                // todo: handle this more gracefully
                panic!("iteration failed with err: {:#?}", err)
            }
        }
    }

    async fn mark_success_iteration(
        &self,
        actions: Vec<InternalActionResult>,
        context: IterationContext,
    ) {
        let mut guard = self.inner.lock().await;
        guard.mark_success_iteration(actions, context)
    }

    pub async fn build(&self) -> TestResult {
        let guard = self.inner.lock().await;
        guard.build()
    }
}

struct TestResultLogItem {
    _actions: Vec<InternalActionResult>,
    _context: IterationContext,
}

struct TestResultBuilderInner {
    http_timings: HashMap<String, HashMap<StatusCode, Histogram<u64>>>,
    _timings: HashMap<String, Vec<Duration>>,
    full_log: Vec<TestResultLogItem>,
}

impl TestResultBuilderInner {
    fn new() -> Self {
        TestResultBuilderInner {
            http_timings: HashMap::new(),
            _timings: HashMap::new(),
            full_log: Vec::with_capacity(512),
        }
    }

    fn mark_success_iteration(
        &mut self,
        actions: Vec<InternalActionResult>,
        context: IterationContext,
    ) {
        for action in &actions {
            match &action.internal {
                Ok(info) => self.mark_success_action(info, action),
                Err(_) => {}
            }
        }

        let log_item = TestResultLogItem {
            _actions: actions,
            _context: context,
        };
        self.full_log.push(log_item);
    }

    fn mark_success_action(&mut self, info: &ActionExecutionInfo, action: &InternalActionResult) {
        match info {
            ActionExecutionInfo::Ok => {}
            ActionExecutionInfo::HttpCall {
                timing,
                status_code,
                ..
            } => {
                let name_str = action.name.as_str();
                self.mark_http_timing(name_str, status_code, timing);
            }
            ActionExecutionInfo::Multi { items } => {
                for i in items {
                    self.mark_success_action(i, action);
                }
            }
        }
    }

    fn mark_http_timing(&mut self, name_str: &str, status_code: &StatusCode, timing: &Duration) {
        if !self.http_timings.contains_key(name_str) {
            self.http_timings
                .insert(name_str.to_string(), HashMap::new());
        }

        let timing_category = self
            .http_timings
            .get_mut(name_str)
            .expect("category must exist");

        if !timing_category.contains_key(status_code) {
            timing_category.insert(*status_code, Histogram::<u64>::new(2).unwrap());
        }

        let timings_log = timing_category
            .get_mut(status_code)
            .expect("hist must exist");
        let duration_as_ns = (timing.as_secs_f32() * 1000.0 * 1000.0) as u64;
        timings_log.record(duration_as_ns).unwrap();
    }

    fn build_http_timings(&self) -> HashMap<String, HashMap<String, TestResultTimingLogItem>> {
        let mut res = HashMap::with_capacity(self.http_timings.len());
        for (name, log) in &self.http_timings {
            let mut current_group = HashMap::new();
            for (status, hist) in log {
                current_group.insert(
                    status.to_string(),
                    TestResultTimingLogItem::map_from_histogram(hist),
                );
            }

            res.insert(name.to_string(), current_group);
        }

        res
    }

    pub fn build(&self) -> TestResult {
        let timings = self.build_http_timings();
        TestResult { timings }
    }
}
