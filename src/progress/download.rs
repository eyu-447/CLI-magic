use crate::progress::bar::ProgressBar;
use std::thread;
use std::time::Duration;

pub fn download_file_sim(file_size: u64) {
    let mut pb = ProgressBar::new(file_size);
    for i in 0..=file_size {
        pb.set_position(i);
        thread::sleep(Duration::from_millis(50));
    }
    pb.finish("Download complete!");
}
