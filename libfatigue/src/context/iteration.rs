use crate::context::TestDurationTracker;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::RwLock;
use std::time::{Duration, Instant};

pub(crate) struct IterationDurationTracker {
    warm_up: WarmUpTracker,
    iterations: u64,
    is_done: AtomicBool,
    count: AtomicU64,
}

impl IterationDurationTracker {
    pub fn new(warm_up: Option<Duration>, iterations: u64) -> Self {
        IterationDurationTracker {
            warm_up: WarmUpTracker::new(warm_up),
            iterations,
            is_done: AtomicBool::new(false),
            count: AtomicU64::new(0),
        }
    }
}

impl TestDurationTracker for IterationDurationTracker {
    fn mark_iteration(&self) {
        if self.should_track_iteration() {
            let old = self.count.fetch_add(1, Ordering::Relaxed);
            let approx_cur = old + 1;
            if approx_cur >= self.iterations {
                self.mark_exit();
            }
        }
    }

    fn mark_exit(&self) {
        self.is_done.store(true, Ordering::Release)
    }

    fn is_done(&self) -> bool {
        self.is_done.load(Ordering::Relaxed)
    }

    fn should_track_iteration(&self) -> bool {
        self.warm_up.is_done()
    }
}

pub(crate) struct TimedDurationTracker {
    ends_at: Instant,
    is_done: AtomicBool,
    warm_up: WarmUpTracker,
}

impl TimedDurationTracker {
    pub(crate) fn new(until: Duration, warm_up: Option<Duration>) -> Self {
        let started_at = Instant::now();
        let ends_at = match warm_up {
            None => started_at + until,
            Some(warm) => started_at + until + warm,
        };
        let is_done = AtomicBool::new(false);
        TimedDurationTracker {
            ends_at,
            is_done,
            warm_up: WarmUpTracker::new(warm_up),
        }
    }
}

impl TestDurationTracker for TimedDurationTracker {
    fn mark_iteration(&self) {
        // no-op
    }

    fn mark_exit(&self) {
        self.is_done.store(true, Ordering::Release);
    }

    fn is_done(&self) -> bool {
        let now = Instant::now();
        (now > self.ends_at) || (self.is_done.load(Ordering::Relaxed))
    }

    fn should_track_iteration(&self) -> bool {
        self.warm_up.is_done()
    }
}

struct WarmUpTracker {
    inner: RwLock<Option<WarmUpTrackerInner>>,
}

struct WarmUpTrackerInner {
    ends_at: Instant,
}

impl WarmUpTracker {
    fn new(until: Option<Duration>) -> Self {
        let inner = match until {
            None => None,
            Some(d) => Some(WarmUpTrackerInner::new(d)),
        };
        let inner = RwLock::new(inner);
        WarmUpTracker { inner }
    }

    fn is_done(&self) -> bool {
        {
            let guard = self.inner.read().unwrap();
            match &*guard {
                Some(inner) => {
                    let inner_res = inner.is_done();
                    if !inner_res {
                        return false;
                    }
                    // inner tracker is done, exit this scope to give up
                    // the reader so we can set the rwlock to none
                }
                None => return true,
            }
        }
        {
            let mut guard = self.inner.write().unwrap();
            return match &*guard {
                None => true,
                Some(inner) => {
                    let inner_res = inner.is_done();
                    if !inner_res {
                        false
                    } else {
                        *guard = None;
                        true
                    }
                }
            };
        }
    }
}

impl WarmUpTrackerInner {
    fn new(until: Duration) -> Self {
        let started_at = Instant::now();
        let ends_at = started_at + until;
        WarmUpTrackerInner { ends_at }
    }

    fn is_done(&self) -> bool {
        Instant::now() > self.ends_at
    }
}
