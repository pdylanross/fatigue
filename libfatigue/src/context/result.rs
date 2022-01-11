use crate::actions::{ActionExecutionInfo, InternalActionResult};
use crate::context::{IterationContext, IterationResult, TestResultTimingLogItem};
use crate::TestResult;
use hdrhistogram::Histogram;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::Instant;

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

struct TestResultBuilderInner {
    http_timings: HashMap<String, HashMap<StatusCode, Histogram<u64>>>,
    started_at: Instant,
}

impl TestResultBuilderInner {
    fn new() -> Self {
        TestResultBuilderInner {
            http_timings: HashMap::new(),
            started_at: Instant::now(),
        }
    }

    fn mark_success_iteration(
        &mut self,
        actions: Vec<InternalActionResult>,
        _context: IterationContext,
    ) {
        for action in &actions {
            match &action.internal {
                Ok(info) => self.mark_success_action(info, action),
                Err(_) => {}
            }
        }
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

    fn build_http_timings(
        &self,
    ) -> (
        HashMap<String, HashMap<String, TestResultTimingLogItem>>,
        u64,
    ) {
        let mut res = HashMap::with_capacity(self.http_timings.len());
        let mut call_count = 0;
        for (name, log) in &self.http_timings {
            let mut current_group = HashMap::new();
            for (status, hist) in log {
                let log_item = TestResultTimingLogItem::map_from_histogram(hist);
                call_count += log_item.metric_len;
                current_group.insert(
                    status.to_string(),
                    TestResultTimingLogItem::map_from_histogram(hist),
                );
            }

            res.insert(name.to_string(), current_group);
        }

        (res, call_count)
    }

    pub fn build(&self) -> TestResult {
        let (timings, requests) = self.build_http_timings();
        let time_since_started = Instant::now() - self.started_at;
        let requests_per_second = (requests as f64) / time_since_started.as_secs_f64();
        TestResult {
            timings,
            requests_per_second,
            duration: None,
        }
    }
}
