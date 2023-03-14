use active_win_pos_rs::get_active_window;
use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

pub fn monitor_loop(initial_app: &str, duration: Duration, tracker_tx: Sender<String>) {
    let mut intial_name = initial_app.to_string();
    loop {
        thread::sleep(duration);
        let change = get_window_name();
        if change != intial_name.to_string() {
            println!("Changed from {} to {}", initial_app, change);
            intial_name = change.clone();
            if tracker_tx.send(change).is_err() {
                break;
            }
        }
    }
}

pub fn activity_loop(tracker_tx: Receiver<String>, actitities: &mut HashMap<String, u64>) {
    let mut last_instant = Instant::now();
    let mut last_activity = String::from("");
    loop {        
        let new_activity = tracker_tx.recv().unwrap();
        let now = Instant::now();
        let elapsed = (now - last_instant).as_secs();
        println!("seconds elapsed {} on {} \n", last_activity.clone(), elapsed);
        actitities.entry(last_activity).and_modify(|e| *e += elapsed).or_insert(elapsed);
        last_instant = Instant::now();
        last_activity = new_activity;
        for (key, value) in actitities.clone() {
            println!("{}: {}", key, value);
        }
    }
}

fn get_window_name() -> String {
    match get_active_window() {
        Ok(active_window) => {
            return active_window.process_name;
        }
        Err(()) => {
            return "".into();
        }
    }
}
