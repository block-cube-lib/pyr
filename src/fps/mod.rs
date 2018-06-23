use std::time::{Duration, SystemTime};

fn duration_to_f64(d: Duration) -> f64 {
    d.as_secs() as f64 + d.subsec_nanos() as f64 * 1e-9
}

pub struct FpsController {
    setting_fps: u32,
    fps: f64,
    begin_frame_time: SystemTime,
    end_frame_time: SystemTime,
    is_frame_ended: bool,
    frame_count: usize,
}

impl FpsController {
    pub fn new(fps: u32) -> FpsController {
        FpsController {
            setting_fps: fps,
            fps: 0.0,
            begin_frame_time: SystemTime::now(),
            end_frame_time: SystemTime::now(),
            is_frame_ended: true,
            frame_count: 0,
        }
    }

    pub fn begin_frame(&mut self) {
        if !self.is_frame_ended {
            return;
        }
        self.is_frame_ended = false;

        self.begin_frame_time = SystemTime::now();
        let one_frame_time = if self.setting_fps == 0 {
            0.0
        } else {
            1.0 / (self.setting_fps as f64)
        };
        let one_frame_time = Duration::new(0, (one_frame_time * 1e9) as u32);

        self.end_frame_time = self.begin_frame_time + one_frame_time;
        self.frame_count += 1;
    }

    pub fn end_frame(&mut self) {
        if self.is_frame_ended {
            return;
        }
        while SystemTime::now() <= self.end_frame_time {}

        self.fps = 1.0 / duration_to_f64(self.begin_frame_time.elapsed().unwrap());
        self.is_frame_ended = true;
    }

    pub fn set_fps(&mut self, fps: u32) {
        self.setting_fps = fps;
    }

    pub fn fps(&self) -> f64 {
        self.fps
    }

    pub fn frame_count(&self) -> usize {
        self.frame_count
    }
}
