#[macro_use]
extern crate thiserror;

use crate::output::{get_output_formatter, OutputFormatter};
use figment::providers::{Env, Format, Yaml};
use figment::Figment;
use libfatigue::config::types::FatigueTesterConfig;
use libfatigue::context::TestResult;
use libfatigue::factories::{ActionFactoryError, ContextActionFactoryError};
use libfatigue::{
    FatigueTestError, FatigueTester, FatigueTesterBuilder, FatigueTesterBuilderError,
    TestRunSettings, TestRunWatchSettings,
};
use std::env::set_current_dir;
use std::path::{PathBuf};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::join;
use tokio::sync::watch;
use clap::Parser;

mod output;

#[derive(Error, Debug)]
enum AppError {
    #[error("error setting up fatigue tester: `{0}`")]
    FatigueBuilder(#[from] FatigueTesterBuilderError),
    #[error("error setting up actions: `{0}`")]
    ActionFactory(#[from] ActionFactoryError),
    #[error("error setting up context actions: `{0}`")]
    ContextActionFactory(#[from] ContextActionFactoryError),
    #[error("error running test: `{0}`")]
    TestError(#[from] FatigueTestError),
    #[error("error constructing config: {0}")]
    FigmentError(#[from] figment::Error),
}

#[derive(clap::Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long, value_name="PLAN_FILE")]
    pub plan: PathBuf
}

async fn execute_command(args: Args) -> Result<(), AppError> {
    let config: FatigueTesterConfig = Figment::new()
        .merge(Yaml::file(&args.plan))
        .merge(Env::prefixed("fatigue."))
        .extract()?;

    let env = Env::raw();
    let global = env.global();

    println!("{:#?}", global);

    let tester = FatigueTesterBuilder::new_from_config(config)
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

    // any file path based operations are done from the perspective of the config file
    let base_dir = args.plan
        .parent()
        .expect("could not load parent dir of the config file");
    set_current_dir(base_dir).expect("could not change cwd to the directory of the config file");

    let output_formatter = Arc::new(get_output_formatter());
    let is_done = Arc::new(AtomicBool::new(false));

    join!(
        result_watch(watch_rx, output_formatter.clone(), is_done.clone()),
        run_tester(
            tester,
            test_settings,
            output_formatter.clone(),
            is_done.clone()
        )
    );

    Ok(())
}

async fn result_watch(
    mut rx: watch::Receiver<TestResult>,
    output_formatter: Arc<Box<dyn OutputFormatter>>,
    is_done: Arc<AtomicBool>,
) {
    while rx.changed().await.is_ok() && !is_done.load(Ordering::Relaxed) {
        output_formatter.update_result_status(&*rx.borrow());
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
        Err(err) => {
            output_formatter.write_err(err);
            exit(-1);
        }
    }

    is_done.store(true, Ordering::Release);
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args = Args::parse();
    execute_command(args).await
}
