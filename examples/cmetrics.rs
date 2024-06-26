use std::{thread, time::Duration};

use anyhow::Result;
use concurrency::CmapMetrics;
use rand::{thread_rng, Rng};

const N: usize = 2;
const M: usize = 4;

fn main() -> Result<()> {
    let metrics = CmapMetrics::new();

    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(5));
        // println!("{:?}", metrics.snapshot()?);
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, metrics: CmapMetrics) -> Result<()> {
    thread::spawn(move || {
        let mut rng = thread_rng();
        loop {
            thread::sleep(Duration::from_millis(rng.gen_range(100..5000)));

            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    Ok(())
}

fn request_worker(metrics: CmapMetrics) -> Result<()> {
    thread::spawn(move || {
        loop {
            let mut rng = thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..10);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
