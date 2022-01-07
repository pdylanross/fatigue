#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate clap;

use crate::output::{get_output_formatter, OutputFormatter};
use clap::{App, Arg};
use libfatigue::context::TestResult;
use libfatigue::factories::{ActionFactoryError, ContextActionFactoryError};
use libfatigue::{
    FatigueTestError, FatigueTester, FatigueTesterBuilder, FatigueTesterBuilderError,
    TestRunSettings, TestRunWatchSettings,
};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::join;
use tokio::sync::watch;

mod output;

#[derive(Error, Debug)]
enum AppError {
    #[error("missing arg `{0}`")]
    MissingArg(&'static str),
    #[error("error setting up fatigue tester: `{0}`")]
    FatigueBuilder(#[from] FatigueTesterBuilderError),
    #[error("error setting up actions: `{0}`")]
    ActionFactory(#[from] ActionFactoryError),
    #[error("error setting up context actions: `{0}`")]
    ContextActionFactory(#[from] ContextActionFactoryError),
    #[error("error running test: `{0}`")]
    TestError(#[from] FatigueTestError),
}

fn init() -> Result<(), AppError> {
    Ok(())
}

fn build_command<'a, 'b>() -> App<'a, 'b> {
    app_from_crate!().arg(
        Arg::with_name("plan")
            .short("p")
            .long("plan")
            .value_name("FILE")
            .help("the test plan to execute")
            .takes_value(true)
            .required(true),
    )
}

async fn execute_command<'a, 'b>(app: App<'a, 'b>) -> Result<(), AppError> {
    let matches = app.get_matches();

    let plan_file = match matches.value_of("plan") {
        None => return Err(AppError::MissingArg("plan")),
        Some(val) => Path::new(val),
    };

    let tester = FatigueTesterBuilder::new_from_config_file(plan_file)?
        .with_default_actions()?
        .with_default_context_actions()?
        .build()?;

    let (watch_tx, watch_rx) = watch::channel(Default::default());

    let test_settings = TestRunSettings {
        watch_settings: Some(TestRunWatchSettings {
            result_watch: Some(watch_tx),
        }),
        ..Default::default()
    };

    let output_formatter = Arc::new(get_output_formatter());
    let is_done = Arc::new(AtomicBool::new(false));

    join!(
        result_watch(watch_rx, output_formatter.clone()),
        run_tester(
            tester,
            test_settings,
            output_formatter.clone(),
            is_done.clone()
        ),
        ui_update(output_formatter.clone(), is_done.clone())
    );

    Ok(())
}

async fn result_watch(
    mut rx: watch::Receiver<TestResult>,
    output_formatter: Arc<Box<dyn OutputFormatter>>,
) {
    while rx.changed().await.is_ok() {
        output_formatter.update_result_status(&*rx.borrow());
    }
}

async fn ui_update(output_formatter: Arc<Box<dyn OutputFormatter>>, is_done: Arc<AtomicBool>) {
    while !is_done.load(Ordering::Relaxed) {
        output_formatter.tick();
        tokio::time::sleep(Duration::from_millis(33)).await;
    }
}

async fn run_tester(
    tester: FatigueTester,
    settings: TestRunSettings,
    output_formatter: Arc<Box<dyn OutputFormatter>>,
    is_done: Arc<AtomicBool>,
) {
    let result = tester.execute_async(settings).await;
    match result {
        Ok(result) => output_formatter.write_final_results(result),
        Err(err) => output_formatter.write_err(err),
    }

    is_done.store(true, Ordering::Release);
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    init()?;
    let cmd = build_command();
    execute_command(cmd).await
}
