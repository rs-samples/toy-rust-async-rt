mod executor;
mod sleep;

use std::{
    future::Future,
    io::Write,
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

// use futures::future::join;

#[derive(Debug)]
struct Yield {
    yielded: bool,
}

impl Future for Yield {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        log::error!("Yield polled cx: {:?}", cx);
        log::error!("Yielded = {}", self.yielded);
        let p = if !self.yielded {
            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        };

        log::error!("Yielded = {}, Poll is {:?}", self.yielded, p);
        p
    }
}

async fn simple() -> i32 {
    // sleep::sleep(Duration::from_secs(5)).await;
    let p5 = Duration::from_secs(5);
    log::warn!("going to sleep period {:?}", p5);
    let sleep5 = sleep::sleep(p5);

    let to3 = Duration::from_secs(3);
    log::warn!("setting timeout to {:?}", to3);

    let timeout = sleep::timeout(sleep5, to3);
    log::warn!("waiting on timeout");
    let s = match timeout.await {
        Some(()) => "Success timer hit",
        None => "We timed out",
    };
    log::warn!("timeout done: {s}");

    log::warn!("going to sleep period {:?}", to3);
    let sleep3 = sleep::sleep(to3);

    log::warn!("setting timeout to {:?}", p5);
    let timeout = sleep::timeout(sleep3, p5);
    let s = match timeout.await {
        Some(()) => "Success timer hit",
        None => "We timed out",
    };
    log::warn!("timeout done: {s}");

    // join(sleep3, sleep5).await;

    let inner = Yield { yielded: false };
    log::warn!("Yielding: {:?}", inner);
    inner.await;
    log::warn!("Yielded");
    10
}

fn main() -> anyhow::Result<()> {
    // simple_logger::SimpleLogger::new()
    //     // .with_level(log::LevelFilter::Trace)
    //     .with_threads(true)
    //     .with_colors(true)
    //     .env()
    //     .init()?;

    env_logger::Builder::new()
        .parse_default_env()
        .write_style(env_logger::WriteStyle::Always)
        .format(|buf, record| {
            let level = record.level();
            let style = buf.default_level_style(level);

            writeln!(
                buf,
                "{}:{} [{} {}] [{style}{}{style:#}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.target(),
                record.level(),
                record.args()
            )
        })
        .init();

    log::trace!("Main Start");
    let fut = simple();

    let output = executor::run_future(fut);

    log::trace!("output={output}");

    Ok(())
}
