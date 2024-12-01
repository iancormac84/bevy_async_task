//! Task pool example - this demonstrates running several async tasks concurrently.

use async_std::task::sleep;
use bevy::{app::PanicHandlerPlugin, log::LogPlugin, prelude::*};
use bevy_async_task::AsyncTaskPool;
use std::{task::Poll, time::Duration};

fn system1(mut task_pool: AsyncTaskPool<'_, u64>) {
    if task_pool.is_idle() {
        info!("Queueing 5 tasks...");
        for i in 1..=5 {
            task_pool.spawn(async move {
                sleep(Duration::from_millis(i * 1000)).await;
                i
            });
        }
    }

    for status in task_pool.iter_poll() {
        if let Poll::Ready(t) = status {
            info!("Received {t}");
        }
    }
}

/// Entry point
pub fn main() {
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default(), PanicHandlerPlugin))
        .add_systems(Update, system1)
        .run();
}
