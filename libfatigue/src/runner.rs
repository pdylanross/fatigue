use crate::context::IterationResult;
use crate::{FatigueTestError, InternalAction, TestResult, TestRunContext, TestRunSettings};
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::future::select_all;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tokio::{spawn, time};

pub(crate) struct TestRunner {
    ctx: Arc<TestRunContext>,
    settings: TestRunSettings,
    actions: Arc<Vec<InternalAction>>,
    iteration_rx: Receiver<IterationResult>,
    iteration_tx: Sender<IterationResult>,
}

impl TestRunner {
    pub fn new(
        ctx: Arc<TestRunContext>,
        settings: TestRunSettings,
        actions: Arc<Vec<InternalAction>>,
    ) -> Self {
        // todo: buffer size here?
        let (iteration_tx, iteration_rx) = channel(256);

        TestRunner {
            ctx,
            settings,
            actions,
            iteration_tx,
            iteration_rx,
        }
    }

    pub async fn run(self) -> Result<TestResult, FatigueTestError> {
        let mut join_handles = self.start_workers();

        join_handles.push(start_iteration_result_watch(
            self.iteration_rx,
            self.ctx.clone(),
        ));

        {
            // own & drop tx handles to allow for rx's to close properly
            let _itx = self.iteration_tx;
        }

        if let Some(watch_settings) = self.settings.watch_settings {
            if let Some(sender) = watch_settings.result_watch {
                join_handles.push(start_test_run_watch_handler(sender, self.ctx.clone()));
            }
        }

        while !join_handles.is_empty() {
            let (current_result, _remaining, jh) = select_all(join_handles).await;
            join_handles = jh;

            match current_result {
                Ok(inner_result) => match inner_result {
                    Ok(_) => continue,
                    Err(e) => {
                        return Err(e);
                    }
                },
                Err(je) => {
                    je.into_panic();
                }
            }
        }

        Ok(self.ctx.get_test_results().await)
    }

    fn start_workers(&self) -> Vec<JoinHandle<Result<(), FatigueTestError>>> {
        let worker_count = self.ctx.info.concurrency.unwrap_or(1);
        let mut join_handles = Vec::with_capacity(worker_count);
        for _i in 0..worker_count {
            let iteration_tx = self.iteration_tx.clone();
            let ctx = self.ctx.clone();
            let actions = self.actions.clone();
            let join_handle = spawn(async move {
                let new_worker = TestRunWorker::new(iteration_tx, ctx, actions);
                new_worker.run().await
            });
            join_handles.push(join_handle);
        }

        join_handles
    }
}

fn start_iteration_result_watch(
    mut rx: Receiver<IterationResult>,
    ctx: Arc<TestRunContext>,
) -> JoinHandle<Result<(), FatigueTestError>> {
    spawn(async move {
        while ctx.is_not_done() {
            let rx_iter = rx.next().await;
            match rx_iter {
                None => return Ok(()),
                Some(res) => ctx.mark_iteration(res).await,
            }
        }

        Ok(())
    })
}

fn start_test_run_watch_handler(
    sender: watch::Sender<TestResult>,
    ctx: Arc<TestRunContext>,
) -> JoinHandle<Result<(), FatigueTestError>> {
    spawn(async move {
        while ctx.is_not_done() {
            let mut results = ctx.get_test_results().await;
            let status = ctx.get_duration_status();
            results.duration = Some(status);
            let send_res = sender.send(results);

            // todo: probably handle this better?
            if send_res.is_err() {
                return Ok(());
            }

            time::sleep(Duration::from_millis(200)).await;
        }

        Ok(())
    })
}

struct TestRunWorker {
    iteration_tx: Sender<IterationResult>,
    ctx: Arc<TestRunContext>,
    actions: Arc<Vec<InternalAction>>,
}

impl TestRunWorker {
    fn new(
        iteration_tx: Sender<IterationResult>,
        ctx: Arc<TestRunContext>,
        actions: Arc<Vec<InternalAction>>,
    ) -> Self {
        TestRunWorker {
            ctx,
            actions,
            iteration_tx,
        }
    }

    async fn run(mut self) -> Result<(), FatigueTestError> {
        while self.ctx.is_not_done() {
            let mut context = self.ctx.new_iteration_ctx().await?;

            let mut actions = Vec::with_capacity(self.actions.len());
            for action in self.actions.iter() {
                let current_res = action.execute(&mut context).await;
                match &current_res.internal {
                    Ok(_) => {}
                    Err(e) => {
                        println!("action err {:#?}", e);
                    }
                }
                actions.push(current_res);
            }
            let iteration_result = IterationResult::Ok { actions, context };

            if self.iteration_tx.feed(iteration_result).await.is_err() {
                return Ok(());
            }
        }

        Ok(())
    }
}
