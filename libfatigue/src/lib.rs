#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate serde;

use crate::actions::{register_default_actions, ActionPointer, InternalAction};
use crate::config::types::{FatigueTesterConfig, FatigueTesterRunInformation};
use crate::context::actions::register_default_context_actions;
use crate::context::{
    IterationContextError, StaticContextActionPointer, TestResult, TestRunContext,
};
use crate::factories::{
    ActionFactory, ActionFactoryError, ContextActionFactory, ContextActionFactoryError,
};
use crate::runner::TestRunner;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Arc;
use tokio::runtime::{Builder, Handle, Runtime};
use tokio::sync::watch;

pub mod actions;
pub mod config;
pub mod context;
pub mod factories;
pub mod util;

mod runner;

pub struct FatigueTester {
    actions: Arc<Vec<InternalAction>>,
    context_actions: Arc<Vec<StaticContextActionPointer>>,
    run_info: Arc<FatigueTesterRunInformation>,
}

#[derive(Error, Debug)]
pub enum FatigueTestError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("No async runtime available: {0}")]
    NoRuntime(#[from] tokio::runtime::TryCurrentError),
    #[error("error setting up the context: {0}")]
    IterationContext(#[from] IterationContextError),
}

#[derive(Default)]
pub struct TestRunSettings {
    /// If none, assume that the async runtime is setup already
    /// otherwise this will describe how to setup a tokio RT
    pub async_settings: Option<TestRunAsyncSettings>,
    pub watch_settings: Option<TestRunWatchSettings>,
}

#[derive(Default)]
pub struct TestRunAsyncSettings {
    pub thread_type: TestRunThreadType,
}

pub enum TestRunThreadType {
    Single,
    Multi { worker_threads: usize },
}

impl Default for TestRunThreadType {
    fn default() -> Self {
        TestRunThreadType::Multi {
            worker_threads: num_cpus::get(),
        }
    }
}

#[derive(Default)]
pub struct TestRunWatchSettings {
    pub result_watch: Option<watch::Sender<TestResult>>,
}

impl FatigueTester {
    pub fn execute(&self, settings: TestRunSettings) -> Result<TestResult, FatigueTestError> {
        let mut _rt_guard = None;
        let _rt = match &settings.async_settings {
            None => None,
            Some(settings) => {
                let rt = self.setup_runtime(settings)?;
                _rt_guard = Some(rt.enter());
                Some(rt)
            }
        };

        let rt_handle = Handle::try_current()?;
        rt_handle.block_on(async move { self.execute_async(settings).await })
    }

    pub async fn execute_async(
        &self,
        settings: TestRunSettings,
    ) -> Result<TestResult, FatigueTestError> {
        let run_context = Arc::new(TestRunContext::new(
            self.run_info.clone(),
            self.context_actions.clone(),
        ));
        let runner = TestRunner::new(run_context, settings, self.actions.clone());
        runner.run().await
    }

    fn setup_runtime(&self, settings: &TestRunAsyncSettings) -> Result<Runtime, FatigueTestError> {
        let mut builder = match settings.thread_type {
            TestRunThreadType::Single => Builder::new_current_thread(),
            TestRunThreadType::Multi { worker_threads } => {
                let mut b = Builder::new_multi_thread();
                b.worker_threads(worker_threads);
                b
            }
        };

        Ok(builder.enable_all().build()?)
    }
}

#[derive(Default)]
pub struct FatigueTesterBuilder {
    action_factory: ActionFactory,
    context_action_factory: ContextActionFactory,
    config: Option<FatigueTesterConfig>,
}

#[derive(Debug, Error)]
pub enum FatigueTesterBuilderError {
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("yaml deserialization error: {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("config not set")]
    MissingConfig,
    #[error("action factory error: {0}")]
    ActionFactoryError(#[from] ActionFactoryError),
    #[error("context action factory error: {0}")]
    ContextActionFactoryError(#[from] ContextActionFactoryError),
}

impl FatigueTesterBuilder {
    pub fn build(self) -> Result<FatigueTester, FatigueTesterBuilderError> {
        let config = match self.config {
            Some(c) => c,
            None => return Err(FatigueTesterBuilderError::MissingConfig),
        };

        let actions = Arc::new(self.action_factory.build_all(&config)?);
        let context_actions = Arc::new(self.context_action_factory.build_all(&config)?);
        let run_info = Arc::new(config.run_info);

        Ok(FatigueTester {
            actions,
            run_info,
            context_actions,
        })
    }

    pub fn new_from_config_file<P: AsRef<Path>>(
        config_file: P,
    ) -> Result<FatigueTesterBuilder, FatigueTesterBuilderError> {
        let mut file = File::open(config_file)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        FatigueTesterBuilder::new_from_config_string(contents)
    }

    pub fn new_from_config_string<S: AsRef<str>>(
        config_str: S,
    ) -> Result<FatigueTesterBuilder, FatigueTesterBuilderError> {
        let config: FatigueTesterConfig = serde_yaml::from_str(config_str.as_ref())?;
        Ok(FatigueTesterBuilder::new_from_config(config))
    }

    pub fn new_from_config(config: FatigueTesterConfig) -> Self {
        FatigueTesterBuilder {
            config: Some(config),
            ..Default::default()
        }
    }

    pub fn with_config(mut self, config: FatigueTesterConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn with_action_factory(mut self, factory: ActionFactory) -> Self {
        self.action_factory = factory;
        self
    }

    pub fn configure_action_factory<F: FnOnce(&mut ActionFactory)>(mut self, cfg: F) -> Self {
        cfg(&mut self.action_factory);
        self
    }

    pub fn with_context_action_factory(mut self, factory: ContextActionFactory) -> Self {
        self.context_action_factory = factory;
        self
    }

    pub fn configure_context_action_factory<F: FnOnce(&mut ContextActionFactory)>(
        mut self,
        cfg: F,
    ) -> Self {
        cfg(&mut self.context_action_factory);
        self
    }

    pub fn with_default_actions(mut self) -> Result<Self, ActionFactoryError> {
        register_default_actions(&mut self.action_factory)?;
        Ok(self)
    }

    pub fn with_default_context_actions(mut self) -> Result<Self, ContextActionFactoryError> {
        register_default_context_actions(&mut self.context_action_factory)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::FatigueTesterBuilder;

    #[test]
    pub fn test_fatigue_builder() {
        let builder = FatigueTesterBuilder::new_from_config_string(
            r#"
run:
  base_url: http://google.com
  duration:
    iteration:
      iterations: 5000
actions:
  - type: request
    properties:
      path: /
    "#,
        )
        .unwrap();

        let builder = builder.with_default_actions().unwrap();

        assert_eq!(
            "http://google.com",
            builder.config.as_ref().unwrap().run_info.base_url
        );

        let tester = builder.build().unwrap();

        assert_eq!(1, tester.actions.len())
    }
}
