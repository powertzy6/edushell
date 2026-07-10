use crate::monitor::MonitorTransform;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompositorMode {
    Wayland,
    X11,
    XWayland,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompositorState {
    Initializing,
    Running,
    Suspended,
    Paused,
    Stopped,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RenderBackend {
    Gl,
    Vulkan,
    Softpipe,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct CompositorCapabilities {
    pub has_gl: bool,
    pub has_vulkan: bool,
    pub has_hdr: bool,
    pub has_vrr: bool,
    pub max_texture_size: u32,
    pub supports_fractional_scaling: bool,
    pub supports_direct_scanout: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct DamageRegion {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct DamageTracker {
    pub regions: Vec<DamageRegion>,
    pub full_frame_dirty: bool,
}

impl DamageTracker {
    pub fn new() -> Self {
        Self {
            regions: Vec::new(),
            full_frame_dirty: false,
        }
    }

    pub fn mark_dirty(&mut self, region: DamageRegion) {
        if !self.full_frame_dirty {
            self.regions.push(region);
        }
    }

    pub fn mark_full_frame(&mut self) {
        self.full_frame_dirty = true;
        self.regions.clear();
    }

    pub fn clear(&mut self) {
        self.regions.clear();
        self.full_frame_dirty = false;
    }

    pub fn merge_regions(&self) -> Vec<DamageRegion> {
        if self.full_frame_dirty || self.regions.is_empty() {
            return self.regions.clone();
        }
        let mut merged: Vec<DamageRegion> = Vec::new();
        let mut rects: Vec<(i32, i32, i32, i32)> = self
            .regions
            .iter()
            .map(|r| (r.x, r.y, r.x + r.width as i32, r.y + r.height as i32))
            .collect();
        rects.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
        let mut current = rects[0];
        for &next in &rects[1..] {
            if next.0 <= current.2 && next.1 <= current.3 {
                current.2 = current.2.max(next.2);
                current.3 = current.3.max(next.3);
            } else {
                merged.push(DamageRegion {
                    x: current.0,
                    y: current.1,
                    width: (current.2 - current.0) as u32,
                    height: (current.3 - current.1) as u32,
                });
                current = next;
            }
        }
        merged.push(DamageRegion {
            x: current.0,
            y: current.1,
            width: (current.2 - current.0) as u32,
            height: (current.3 - current.1) as u32,
        });
        merged
    }

    pub fn is_dirty(&self) -> bool {
        self.full_frame_dirty || !self.regions.is_empty()
    }

    pub fn is_full_frame(&self) -> bool {
        self.full_frame_dirty
    }

    pub fn damage_count(&self) -> usize {
        self.regions.len()
    }
}

impl Default for DamageTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct FrameInfo {
    pub frame_number: u64,
    pub timestamp_us: u64,
    pub render_duration_us: u64,
    pub present_duration_us: u64,
    pub damage_rects: Vec<DamageRegion>,
    pub missed_vblank: bool,
    pub gpu_time_us: u64,
}

#[derive(Debug, Clone)]
pub struct FrameStats {
    pub total_frames: u64,
    pub dropped_frames: u64,
    pub avg_frame_time_us: f64,
    pub max_frame_time_us: u64,
    pub min_frame_time_us: u64,
    pub percentile_95_us: u64,
}

impl FrameStats {
    fn new() -> Self {
        Self {
            total_frames: 0,
            dropped_frames: 0,
            avg_frame_time_us: 0.0,
            max_frame_time_us: 0,
            min_frame_time_us: u64::MAX,
            percentile_95_us: 0,
        }
    }
}

pub struct FramePacer {
    pub target_fps: f64,
    last_frame_time: u64,
    frame_start: u64,
    pub frame_interval_us: u64,
    pub min_frame_time_us: u64,
    pub stats: FrameStats,
    pub adaptive_sync: bool,
    frame_history: Vec<u64>,
    frame_counter: u64,
}

impl FramePacer {
    pub fn new(target_fps: f64) -> Self {
        let interval = if target_fps > 0.0 {
            (1_000_000.0 / target_fps) as u64
        } else {
            16666
        };
        Self {
            target_fps,
            last_frame_time: 0,
            frame_start: 0,
            frame_interval_us: interval,
            min_frame_time_us: interval / 2,
            stats: FrameStats::new(),
            adaptive_sync: false,
            frame_history: Vec::with_capacity(100),
            frame_counter: 0,
        }
    }

    pub fn begin_frame(&mut self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;
        self.frame_start = now;
        now
    }

    pub fn end_frame(&mut self) -> FrameInfo {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;
        let render_duration = now.saturating_sub(self.frame_start);
        self.frame_counter += 1;
        self.stats.total_frames = self.frame_counter;
        self.stats.avg_frame_time_us = if self.frame_counter > 0 {
            let total = self.stats.avg_frame_time_us * (self.frame_counter - 1) as f64
                + render_duration as f64;
            total / self.frame_counter as f64
        } else {
            render_duration as f64
        };
        self.stats.max_frame_time_us = self.stats.max_frame_time_us.max(render_duration);
        self.stats.min_frame_time_us = self.stats.min_frame_time_us.min(render_duration);
        if render_duration > self.frame_interval_us {
            self.stats.dropped_frames += 1;
        }
        self.frame_history.push(render_duration);
        if self.frame_history.len() > 100 {
            self.frame_history.remove(0);
        }
        let mut sorted = self.frame_history.clone();
        sorted.sort_unstable();
        let p95_idx = ((sorted.len() as f64) * 0.95).ceil() as usize - 1;
        self.stats.percentile_95_us = if p95_idx < sorted.len() {
            sorted[p95_idx]
        } else {
            render_duration
        };
        self.last_frame_time = now;
        FrameInfo {
            frame_number: self.frame_counter,
            timestamp_us: now,
            render_duration_us: render_duration,
            present_duration_us: 0,
            damage_rects: Vec::new(),
            missed_vblank: render_duration > self.frame_interval_us,
            gpu_time_us: 0,
        }
    }

    pub fn calculate_sleep_time(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;
        let elapsed = now.saturating_sub(self.last_frame_time);
        self.frame_interval_us.saturating_sub(elapsed)
    }

    pub fn update_target_fps(&mut self, fps: f64) {
        self.target_fps = fps;
        if fps > 0.0 {
            self.frame_interval_us = (1_000_000.0 / fps) as u64;
        }
    }

    pub fn get_stats(&self) -> &FrameStats {
        &self.stats
    }

    pub fn set_adaptive_sync(&mut self, enabled: bool) {
        self.adaptive_sync = enabled;
    }
}

impl Default for FramePacer {
    fn default() -> Self {
        Self::new(60.0)
    }
}

#[derive(Debug, Clone)]
pub struct RenderSurface {
    pub width: u32,
    pub height: u32,
    pub scale: f64,
    pub format: u32,
    pub transform: MonitorTransform,
}

#[derive(Debug, Clone)]
pub struct OutputState {
    pub surface: RenderSurface,
    pub mode: usize,
    pub damage: DamageTracker,
    pub frame_info: Option<FrameInfo>,
    pub scanned_out: bool,
    pub needs_repaint: bool,
}

pub struct Compositor {
    pub mode: CompositorMode,
    pub state: CompositorState,
    pub backend: RenderBackend,
    pub capabilities: CompositorCapabilities,
    pub frame_pacer: FramePacer,
    pub outputs: HashMap<usize, OutputState>,
    pub damage_tracker: DamageTracker,
    pub triple_buffering: bool,
    pub direct_scanout: bool,
    pub swapchain: Vec<u8>,
    pub render_timings: Vec<FrameInfo>,
}

impl Compositor {
    pub fn new() -> Self {
        Self {
            mode: CompositorMode::Wayland,
            state: CompositorState::Initializing,
            backend: RenderBackend::Gl,
            capabilities: CompositorCapabilities {
                has_gl: true,
                has_vulkan: false,
                has_hdr: false,
                has_vrr: false,
                max_texture_size: 8192,
                supports_fractional_scaling: false,
                supports_direct_scanout: false,
            },
            frame_pacer: FramePacer::new(60.0),
            outputs: HashMap::new(),
            damage_tracker: DamageTracker::new(),
            triple_buffering: false,
            direct_scanout: false,
            swapchain: Vec::new(),
            render_timings: Vec::with_capacity(100),
        }
    }

    pub fn start(&mut self) -> bool {
        if self.state == CompositorState::Initializing || self.state == CompositorState::Stopped {
            self.state = CompositorState::Running;
            true
        } else {
            false
        }
    }

    pub fn stop(&mut self) -> bool {
        if self.state == CompositorState::Running {
            self.state = CompositorState::Stopped;
            true
        } else {
            false
        }
    }

    pub fn pause(&mut self) -> bool {
        if self.state == CompositorState::Running {
            self.state = CompositorState::Paused;
            true
        } else {
            false
        }
    }

    pub fn resume(&mut self) -> bool {
        if self.state == CompositorState::Paused {
            self.state = CompositorState::Running;
            true
        } else {
            false
        }
    }

    pub fn set_mode(&mut self, mode: CompositorMode) -> bool {
        self.mode = mode;
        true
    }

    pub fn set_backend(&mut self, backend: RenderBackend) -> bool {
        self.backend = backend;
        true
    }

    pub fn get_mode(&self) -> &CompositorMode {
        &self.mode
    }

    pub fn get_state(&self) -> &CompositorState {
        &self.state
    }

    pub fn begin_frame(&mut self) -> u64 {
        self.frame_pacer.begin_frame()
    }

    pub fn end_frame(&mut self, output_id: usize) -> Option<FrameInfo> {
        let info = self.frame_pacer.end_frame();
        if let Some(output) = self.outputs.get_mut(&output_id) {
            let mut info = info;
            info.damage_rects = output.damage.merge_regions();
            output.frame_info = Some(info.clone());
            output.damage.clear();
            output.needs_repaint = false;
            self.render_timings.push(info.clone());
            if self.render_timings.len() > 100 {
                self.render_timings.remove(0);
            }
            Some(info)
        } else {
            None
        }
    }

    pub fn mark_dirty(&mut self, output_id: usize, region: DamageRegion) {
        if let Some(output) = self.outputs.get_mut(&output_id) {
            output.damage.mark_dirty(region);
            output.needs_repaint = true;
        }
    }

    pub fn mark_full_frame(&mut self) {
        self.damage_tracker.mark_full_frame();
        for (_, output) in self.outputs.iter_mut() {
            output.damage.mark_full_frame();
            output.needs_repaint = true;
        }
    }

    pub fn attach_output(&mut self, id: usize, surface: RenderSurface) -> bool {
        if self.outputs.contains_key(&id) {
            return false;
        }
        self.outputs.insert(
            id,
            OutputState {
                surface,
                mode: 0,
                damage: DamageTracker::new(),
                frame_info: None,
                scanned_out: false,
                needs_repaint: true,
            },
        );
        true
    }

    pub fn detach_output(&mut self, id: usize) -> bool {
        self.outputs.remove(&id).is_some()
    }

    pub fn get_output_state(&self, id: usize) -> Option<&OutputState> {
        self.outputs.get(&id)
    }

    pub fn needs_repaint(&self) -> bool {
        self.outputs.values().any(|o| o.needs_repaint)
    }

    pub fn any_output_dirty(&self) -> bool {
        self.outputs.values().any(|o| o.damage.is_dirty())
    }

    pub fn set_triple_buffering(&mut self, enabled: bool) {
        self.triple_buffering = enabled;
    }

    pub fn set_direct_scanout(&mut self, enabled: bool) {
        self.direct_scanout = enabled;
    }

    pub fn get_frame_pacer(&self) -> &FramePacer {
        &self.frame_pacer
    }

    pub fn get_frame_pacer_mut(&mut self) -> &mut FramePacer {
        &mut self.frame_pacer
    }

    pub fn get_render_stats(&self) -> Vec<FrameInfo> {
        self.render_timings.clone()
    }

    pub fn simulate_frame(&mut self, output_id: usize) -> FrameInfo {
        let start = self.frame_pacer.begin_frame();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros() as u64;
        let render_dur = timestamp.saturating_sub(start);
        let info = FrameInfo {
            frame_number: self.frame_pacer.frame_counter + 1,
            timestamp_us: timestamp,
            render_duration_us: render_dur,
            present_duration_us: render_dur / 2,
            damage_rects: Vec::new(),
            missed_vblank: render_dur > self.frame_pacer.frame_interval_us,
            gpu_time_us: render_dur / 3,
        };
        self.frame_pacer.frame_counter += 1;
        self.frame_pacer.stats.total_frames = self.frame_pacer.frame_counter;
        if let Some(output) = self.outputs.get_mut(&output_id) {
            output.frame_info = Some(info.clone());
            output.damage.clear();
            output.needs_repaint = false;
        }
        self.render_timings.push(info.clone());
        info
    }
}

impl Default for Compositor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_damage_tracker_merge() {
        let mut dt = DamageTracker::new();
        dt.mark_dirty(DamageRegion {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        });
        dt.mark_dirty(DamageRegion {
            x: 50,
            y: 50,
            width: 100,
            height: 100,
        });
        assert!(dt.is_dirty());
        assert_eq!(dt.damage_count(), 2);
        let merged = dt.merge_regions();
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].x, 0);
        assert_eq!(merged[0].y, 0);
        assert!(merged[0].width >= 150);
        assert!(merged[0].height >= 150);
    }

    #[test]
    fn test_damage_full_frame() {
        let mut dt = DamageTracker::new();
        assert!(!dt.is_full_frame());
        dt.mark_full_frame();
        assert!(dt.is_full_frame());
        assert!(dt.is_dirty());
        dt.clear();
        assert!(!dt.is_dirty());
        assert!(!dt.is_full_frame());
    }

    #[test]
    fn test_frame_pacer_60fps() {
        let mut pacer = FramePacer::new(60.0);
        assert_eq!(pacer.frame_interval_us, 16666);
        let start = pacer.begin_frame();
        assert!(start > 0);
        let info = pacer.end_frame();
        assert_eq!(info.frame_number, 1);
        assert!(info.render_duration_us < 1_000_000);
    }

    #[test]
    fn test_compositor_lifecycle() {
        let mut comp = Compositor::new();
        assert_eq!(*comp.get_state(), CompositorState::Initializing);
        assert!(comp.start());
        assert_eq!(*comp.get_state(), CompositorState::Running);
        assert!(!comp.start());
        assert!(comp.pause());
        assert_eq!(*comp.get_state(), CompositorState::Paused);
        assert!(!comp.pause());
        assert!(comp.resume());
        assert_eq!(*comp.get_state(), CompositorState::Running);
        assert!(comp.stop());
        assert_eq!(*comp.get_state(), CompositorState::Stopped);
        assert!(!comp.stop());
    }

    #[test]
    fn test_output_attach_detach() {
        let mut comp = Compositor::new();
        let surface = RenderSurface {
            width: 1920,
            height: 1080,
            scale: 1.0,
            format: 0,
            transform: MonitorTransform::Normal,
        };
        assert!(comp.attach_output(0, surface));
        assert!(comp.get_output_state(0).is_some());
        assert!(!comp.attach_output(
            0,
            RenderSurface {
                width: 1920,
                height: 1080,
                scale: 1.0,
                format: 0,
                transform: MonitorTransform::Normal,
            }
        ));
        assert!(comp.detach_output(0));
        assert!(comp.get_output_state(0).is_none());
        assert!(!comp.detach_output(0));
    }

    #[test]
    fn test_simulate_frame() {
        let mut comp = Compositor::new();
        let surface = RenderSurface {
            width: 1920,
            height: 1080,
            scale: 1.0,
            format: 0,
            transform: MonitorTransform::Normal,
        };
        comp.attach_output(0, surface);
        comp.start();
        let info = comp.simulate_frame(0);
        assert_eq!(info.frame_number, 1);
        assert!(comp.get_output_state(0).unwrap().frame_info.is_some());
    }

    #[test]
    fn test_triple_buffering() {
        let mut comp = Compositor::new();
        assert!(!comp.triple_buffering);
        comp.set_triple_buffering(true);
        assert!(comp.triple_buffering);
        comp.set_triple_buffering(false);
        assert!(!comp.triple_buffering);
    }

    #[test]
    fn test_frame_stats_collection() {
        let mut pacer = FramePacer::new(60.0);
        pacer.begin_frame();
        pacer.end_frame();
        pacer.begin_frame();
        pacer.end_frame();
        pacer.begin_frame();
        pacer.end_frame();
        let stats = pacer.get_stats();
        assert_eq!(stats.total_frames, 3);
        assert!(stats.avg_frame_time_us >= 0.0);
        assert!(stats.min_frame_time_us <= stats.max_frame_time_us);
    }
}
