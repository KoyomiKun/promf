use std::{
    sync::{
        atomic::{AtomicPtr, AtomicU64},
        Arc,
    },
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use anyhow::{anyhow, Ok, Result};
use reqwest::StatusCode;

use log::error;

const API: &'static str = "http://10.11.83.179:8480/api/v1/import/prometheus";

fn main() {
    env_logger::init();

    let p = Arc::new(Pusher::default());

    let mut mp = p.clone();
    thread::spawn(move || p.run());

    for

    mp.change_value(3);
}

struct Pusher {
    curr_v: AtomicU64,
}

impl Default for Pusher {
    fn default() -> Self {
        Self {
            curr_v: AtomicU64::new(0),
        }
    }
}

impl Pusher {
    pub fn change_value(&mut self, value: u64) {
        self.curr_v
            .store(value, std::sync::atomic::Ordering::SeqCst)
    }
    pub fn run(&self) {
        loop {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("time went backwards")
                .as_millis();
            if let Err(e) = self.push(self.push_metrics(
                "test",
                vec!["k", "V"],
                now as u64,
                self.curr_v.load(std::sync::atomic::Ordering::SeqCst),
            )) {
                error!("push metrics failed: {e}")
            };
            thread::sleep(Duration::from_secs(30));
        }
    }

    fn push(&self, s: String) -> Result<()> {
        let res = reqwest::blocking::Client::new().post(API).body(s).send()?;
        if StatusCode::is_success(&res.status()) {
            return Err(anyhow!("status code is {}", res.status()));
        }
        Ok(())
    }

    fn push_metrics(&self, name: &str, labels: Vec<&str>, ts: u64, value: u64) -> String {
        format!("{name}{} {value} {ts} ", Self::labels_to_string(labels))
    }

    fn labels_to_string(labels: Vec<&str>) -> String {
        let mut s = String::new();
        s.push('{');
        s.push_str(&format!(r#"{}="{}""#, labels[0], labels[1]));
        for i in (2..labels.len()).step_by(2) {
            s.push_str(&format!(r#",{}="{}""#, labels[i], labels[i + 1]));
        }
        s.push('}');
        s
    }
}

//func pusher(stop <-chan struct{}) {
//ticker := time.NewTicker(pushInterval)
//for {
//select {
//case <-stop:
//ticker.Stop()
//push()
//return
//case <-ticker.C:
//push()
//}
//}
//}

//func pushMetric(name string, labels []string, ts, value uint64) {
//pushBufferMu.Lock()
//defer pushBufferMu.Unlock()
//pushBuffer.WriteString(name)
//pushBuffer.WriteString(metricLabels(labels...))
//pushBuffer.WriteString(` ` + strconv.FormatUint(value, 10))
//pushBuffer.WriteString(` ` + strconv.FormatUint(ts, 10))
//pushBuffer.WriteString("\n")
//pushMetricCount[name]++
//}
