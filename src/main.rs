use std::thread;
use std::sync::Arc;
use metered::{metered, Throughput, HitCount};
use std::panic::catch_unwind;

#[derive(Default, Debug, serde::Serialize)]
pub struct Biz {
    metrics: BizMetrics,
}

#[metered(registry = BizMetrics)]
impl Biz {
    #[measure([HitCount, Throughput])]
    pub fn biz(&self) {
        let delay = std::time::Duration::from_millis(rand::random::<u64>() % 200);
        std::thread::sleep(delay);
    }
}

fn test_biz() {
    let biz = Arc::new(Biz::default());
    let mut threads = Vec::new();
    for _ in 0..5 {
        let biz = Arc::clone(&biz);
        let t = thread::spawn(move || {
            for _ in 0..200 {
                biz.biz();
            }
        });
        threads.push(t);
    }
    for t in threads {
        t.join().unwrap();
    }
    // Print the results!
    let serialized = serde_yaml::to_string(&*biz).unwrap();
    println!("{}", serialized);
}

#[derive(Debug)]
struct PS {
    x: i64
}

fn main() {
    color_backtrace::install();
    let result = catch_unwind(|| unsafe { Box::from_raw(0 as *mut PS) });
    let out = unsafe { Box::from_raw(12345 as *mut PS) };
    let bo = Box::from(14234 as *const PS);
    let x = unsafe { &**bo };
    println!("Hm... {}", out.x);
    println!("What happened? {:#?}", result);
    test_biz();
    println!("Hello, world!");
}
