use crate::animation::AnimationEngineV2;
use crate::compositor::Compositor;
use crate::core::{FocusManager, StackingManager, WindowCore};
use crate::input::InputManager;
use crate::monitor::MonitorManager;
use crate::wayland::WaylandCompat;
use crate::workspace::WorkspaceEngineV2;
use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InspectorType {
    Window,
    Frame,
    Input,
    Workspace,
    Rendering,
    GPU,
    Wayland,
    Performance,
    All,
}

#[derive(Debug, Clone)]
pub struct InspectResult {
    pub type_: InspectorType,
    pub timestamp: String,
    pub data: HashMap<String, String>,
    pub duration_us: u64,
}

pub struct DebugTool {
    inspections: Vec<InspectResult>,
    max_results: usize,
    enabled_inspectors: Vec<InspectorType>,
    profiling_enabled: bool,
    log_level: String,
    verbose: bool,
}

impl DebugTool {
    pub fn new() -> Self {
        Self {
            inspections: Vec::new(),
            max_results: 500,
            enabled_inspectors: vec![
                InspectorType::Window,
                InspectorType::Workspace,
                InspectorType::Input,
                InspectorType::Frame,
            ],
            profiling_enabled: false,
            log_level: "info".to_string(),
            verbose: false,
        }
    }

    pub fn inspect_window_system(
        &mut self,
        wc: &WindowCore,
        fm: &FocusManager,
        sm: &StackingManager,
    ) -> InspectResult {
        let start = Instant::now();
        let mut data = HashMap::new();
        data.insert("window_count".to_string(), wc.count().to_string());
        data.insert(
            "active_window".to_string(),
            fm.get_active().map(|id| id.to_string()).unwrap_or_default(),
        );
        data.insert("focus_stack_depth".to_string(), fm.len().to_string());
        data.insert(
            "stacking_order_depth".to_string(),
            sm.get_stacking_order().len().to_string(),
        );

        let result = InspectResult {
            type_: InspectorType::Window,
            timestamp: crate::core::now(),
            data,
            duration_us: start.elapsed().as_micros() as u64,
        };
        self.push_result(result.clone());
        result
    }

    pub fn inspect_workspace(&mut self, ws: &WorkspaceEngineV2) -> InspectResult {
        let start = Instant::now();
        let mut data = HashMap::new();
        data.insert(
            "workspace_count".to_string(),
            ws.list_workspaces().len().to_string(),
        );
        data.insert("active_workspace".to_string(), ws.active().id.to_string());
        data.insert("total_windows".to_string(), ws.total_windows().to_string());
        data.insert("overview_active".to_string(), ws.is_overview().to_string());

        for wsi in ws.list_workspaces() {
            data.insert(
                format!("ws_{}_windows", wsi.id),
                wsi.windows.len().to_string(),
            );
        }

        let result = InspectResult {
            type_: InspectorType::Workspace,
            timestamp: crate::core::now(),
            data,
            duration_us: start.elapsed().as_micros() as u64,
        };
        self.push_result(result.clone());
        result
    }

    pub fn inspect_monitors(&mut self, mm: &MonitorManager) -> InspectResult {
        let start = Instant::now();
        let mut data = HashMap::new();
        data.insert("monitor_count".to_string(), mm.count().to_string());
        for m in mm.list_monitors() {
            data.insert(
                format!("mon_{}_resolution", m.id),
                format!("{}x{}", m.geometry.2, m.geometry.3),
            );
            data.insert(format!("mon_{}_scale", m.id), m.scale.to_string());
            data.insert(format!("mon_{}_primary", m.id), m.is_primary.to_string());
        }

        let result = InspectResult {
            type_: InspectorType::Workspace,
            timestamp: crate::core::now(),
            data,
            duration_us: start.elapsed().as_micros() as u64,
        };
        self.push_result(result.clone());
        result
    }

    pub fn inspect_input(&mut self, im: &InputManager) -> InspectResult {
        let start = Instant::now();
        let mut data = HashMap::new();
        data.insert(
            "device_count".to_string(),
            im.list_devices().len().to_string(),
        );
        let pos = im.get_pointer_position();
        data.insert("pointer_x".to_string(), pos.0.to_string());
        data.insert("pointer_y".to_string(), pos.1.to_string());
        data.insert(
            "shortcut_count".to_string(),
            im.list_shortcuts().len().to_string(),
        );
        data.insert(
            "pointer_accel".to_string(),
            im.get_pointer_accel().to_string(),
        );

        let result = InspectResult {
            type_: InspectorType::Input,
            timestamp: crate::core::now(),
            data,
            duration_us: start.elapsed().as_micros() as u64,
        };
        self.push_result(result.clone());
        result
    }

    pub fn inspect_compositor(&mut self, comp: &Compositor) -> InspectResult {
        let start = Instant::now();
        let mut data = HashMap::new();
        data.insert("mode".to_string(), format!("{:?}", comp.mode));
        data.insert("state".to_string(), format!("{:?}", comp.state));
        data.insert("backend".to_string(), format!("{:?}", comp.backend));
        let stats = comp.frame_pacer.get_stats();
        data.insert("total_frames".to_string(), stats.total_frames.to_string());
        data.insert(
            "dropped_frames".to_string(),
            stats.dropped_frames.to_string(),
        );
        data.insert(
            "avg_frame_time_us".to_string(),
            stats.avg_frame_time_us.to_string(),
        );
        data.insert("output_count".to_string(), comp.outputs.len().to_string());
        data.insert(
            "damage_regions".to_string(),
            comp.damage_tracker.damage_count().to_string(),
        );
        data.insert(
            "needs_repaint".to_string(),
            comp.needs_repaint().to_string(),
        );

        let result = InspectResult {
            type_: InspectorType::Frame,
            timestamp: crate::core::now(),
            data,
            duration_us: start.elapsed().as_micros() as u64,
        };
        self.push_result(result.clone());
        result
    }

    pub fn inspect_animation(&mut self, anim: &AnimationEngineV2) -> InspectResult {
        let start = Instant::now();
        let mut data = HashMap::new();
        data.insert(
            "running_animations".to_string(),
            anim.running_count().to_string(),
        );
        data.insert(
            "total_animations".to_string(),
            anim.total_count().to_string(),
        );
        data.insert(
            "reduced_motion".to_string(),
            anim.get_reduced_motion().to_string(),
        );
        data.insert(
            "max_animations".to_string(),
            anim.get_max_animations().to_string(),
        );
        data.insert(
            "global_speed".to_string(),
            anim.get_global_speed().to_string(),
        );

        let result = InspectResult {
            type_: InspectorType::Rendering,
            timestamp: crate::core::now(),
            data,
            duration_us: start.elapsed().as_micros() as u64,
        };
        self.push_result(result.clone());
        result
    }

    pub fn inspect_wayland(&mut self, wl: &WaylandCompat) -> InspectResult {
        let start = Instant::now();
        let mut data = HashMap::new();
        data.insert("surface_count".to_string(), wl.surface_count().to_string());
        data.insert(
            "decoration_mode".to_string(),
            format!("{:?}", wl.get_decoration_mode()),
        );
        data.insert(
            "protocol_count".to_string(),
            wl.list_protocols().len().to_string(),
        );
        let fps = wl.get_fractional_scale_info();
        data.insert("fractional_scale".to_string(), fps.scale.to_string());

        for p in wl.list_protocols() {
            data.insert(format!("protocol_{}", p), "supported".to_string());
        }

        let result = InspectResult {
            type_: InspectorType::Wayland,
            timestamp: crate::core::now(),
            data,
            duration_us: start.elapsed().as_micros() as u64,
        };
        self.push_result(result.clone());
        result
    }

    #[allow(clippy::too_many_arguments)]
    pub fn inspect_all(
        &mut self,
        wc: &WindowCore,
        fm: &FocusManager,
        sm: &StackingManager,
        ws: &WorkspaceEngineV2,
        mm: &MonitorManager,
        im: &InputManager,
        comp: &Compositor,
        anim: &AnimationEngineV2,
        wl: &WaylandCompat,
    ) -> Vec<InspectResult> {
        vec![
            self.inspect_window_system(wc, fm, sm),
            self.inspect_workspace(ws),
            self.inspect_monitors(mm),
            self.inspect_input(im),
            self.inspect_compositor(comp),
            self.inspect_animation(anim),
            self.inspect_wayland(wl),
        ]
    }

    fn push_result(&mut self, result: InspectResult) {
        if self.inspections.len() >= self.max_results {
            self.inspections.remove(0);
        }
        self.inspections.push(result);
    }

    pub fn get_results(&self) -> &[InspectResult] {
        &self.inspections
    }

    pub fn clear_results(&mut self) {
        self.inspections.clear();
    }

    pub fn enable_inspector(&mut self, type_: InspectorType) {
        if !self.enabled_inspectors.contains(&type_) {
            self.enabled_inspectors.push(type_);
        }
    }

    pub fn disable_inspector(&mut self, type_: InspectorType) {
        self.enabled_inspectors.retain(|t| t != &type_);
    }

    pub fn is_inspector_enabled(&self, type_: InspectorType) -> bool {
        self.enabled_inspectors.contains(&type_)
    }

    pub fn set_profiling(&mut self, enabled: bool) {
        self.profiling_enabled = enabled;
    }

    pub fn is_profiling(&self) -> bool {
        self.profiling_enabled
    }

    pub fn set_verbose(&mut self, enabled: bool) {
        self.verbose = enabled;
    }

    pub fn set_log_level(&mut self, level: String) {
        self.log_level = level;
    }

    pub fn generate_report(&self, results: &[InspectResult]) -> String {
        let mut report = String::new();
        report.push_str("=== EduWM Debug Report ===\n");
        report.push_str(&format!("Generated: {}\n\n", crate::core::now()));

        for result in results {
            report.push_str(&format!(
                "[{:?}] {} ({} us)\n",
                result.type_, result.timestamp, result.duration_us
            ));
            for (key, value) in &result.data {
                report.push_str(&format!("  {}: {}\n", key, value));
            }
            report.push('\n');
        }
        report
    }
}

impl Default for DebugTool {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct ProfilerSnapshot {
    pub timestamp: String,
    pub label: String,
    pub cpu_usage_pct: f64,
    pub memory_mb: f64,
    pub gpu_usage_pct: f64,
    pub fps: f64,
    pub frame_time_us: u64,
    pub running_animations: usize,
    pub window_count: usize,
    pub uptime_seconds: u64,
}

pub struct ProfilingTool {
    snapshots: Vec<ProfilerSnapshot>,
    start_time: Instant,
    is_running: bool,
    #[allow(dead_code)]
    interval_ms: u64,
}

impl ProfilingTool {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            start_time: Instant::now(),
            is_running: false,
            interval_ms: 1000,
        }
    }

    pub fn start(&mut self) {
        self.is_running = true;
        self.start_time = Instant::now();
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn is_active(&self) -> bool {
        self.is_running
    }

    pub fn take_snapshot(
        &mut self,
        label: &str,
        fps: f64,
        frame_time: u64,
        windows: usize,
        anims: usize,
    ) -> ProfilerSnapshot {
        let snapshot = ProfilerSnapshot {
            timestamp: crate::core::now(),
            label: label.to_string(),
            cpu_usage_pct: 0.0,
            memory_mb: 0.0,
            gpu_usage_pct: 0.0,
            fps,
            frame_time_us: frame_time,
            running_animations: anims,
            window_count: windows,
            uptime_seconds: self.start_time.elapsed().as_secs(),
        };
        self.snapshots.push(snapshot.clone());
        snapshot
    }

    pub fn get_snapshots(&self) -> &[ProfilerSnapshot] {
        &self.snapshots
    }

    pub fn last_snapshot(&self) -> Option<&ProfilerSnapshot> {
        self.snapshots.last()
    }

    pub fn average_fps(&self, max_samples: usize) -> f64 {
        let samples: Vec<&ProfilerSnapshot> =
            self.snapshots.iter().rev().take(max_samples).collect();
        if samples.is_empty() {
            return 0.0;
        }
        let sum: f64 = samples.iter().map(|s| s.fps).sum();
        sum / samples.len() as f64
    }

    pub fn average_frame_time(&self, max_samples: usize) -> f64 {
        let samples: Vec<&ProfilerSnapshot> =
            self.snapshots.iter().rev().take(max_samples).collect();
        if samples.is_empty() {
            return 0.0;
        }
        let sum: f64 = samples.iter().map(|s| s.frame_time_us as f64).sum();
        sum / samples.len() as f64
    }

    pub fn peak_memory_mb(&self) -> f64 {
        self.snapshots
            .iter()
            .map(|s| s.memory_mb)
            .fold(0.0_f64, f64::max)
    }

    pub fn min_fps(&self) -> f64 {
        self.snapshots
            .iter()
            .map(|s| s.fps)
            .fold(f64::MAX, f64::min)
    }

    pub fn generate_profile_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== EduWM Profile Report ===\n");
        report.push_str(&format!("Snapshots: {}\n", self.snapshots.len()));
        report.push_str(&format!(
            "Duration: {}s\n",
            self.start_time.elapsed().as_secs()
        ));
        report.push_str(&format!("Avg FPS: {:.2}\n", self.average_fps(100)));
        report.push_str(&format!(
            "Avg Frame Time: {:.2} us\n",
            self.average_frame_time(100)
        ));
        report.push_str(&format!("Peak Memory: {:.2} MB\n", self.peak_memory_mb()));
        report.push_str(&format!("Min FPS: {:.2}\n", self.min_fps()));
        report.push('\n');

        for (i, snap) in self.snapshots.iter().enumerate() {
            report.push_str(&format!(
                "#{} [{}] {}: FPS={:.1}, Frame={}us, Windows={}, Anims={}\n",
                i + 1,
                snap.timestamp,
                snap.label,
                snap.fps,
                snap.frame_time_us,
                snap.window_count,
                snap.running_animations,
            ));
        }
        report
    }

    pub fn clear(&mut self) {
        self.snapshots.clear();
        self.start_time = Instant::now();
    }
}

impl Default for ProfilingTool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::animation::AnimationEngineV2;
    use crate::compositor::Compositor;
    use crate::core::{FocusManager, StackingManager, WindowCore};
    use crate::input::InputManager;
    use crate::monitor::MonitorManager;
    use crate::wayland::WaylandCompat;
    use crate::workspace::{WorkspaceConfig, WorkspaceEngineV2};

    #[test]
    fn test_inspect_window_system() {
        let mut dt = DebugTool::new();
        let wc = WindowCore::new();
        let fm = FocusManager::new();
        let sm = StackingManager::new();
        let result = dt.inspect_window_system(&wc, &fm, &sm);
        assert_eq!(result.type_, InspectorType::Window);
        assert_eq!(result.data.get("window_count").unwrap(), "0");
    }

    #[test]
    fn test_inspect_workspace() {
        let mut dt = DebugTool::new();
        let mut ws = WorkspaceEngineV2::new(WorkspaceConfig::Static { count: 3 });
        ws.init(3);
        let result = dt.inspect_workspace(&ws);
        assert_eq!(result.data.get("workspace_count").unwrap(), "3");
        assert_eq!(result.data.get("active_workspace").unwrap(), "0");
    }

    #[test]
    fn test_inspect_all() {
        let mut dt = DebugTool::new();
        let wc = WindowCore::new();
        let fm = FocusManager::new();
        let sm = StackingManager::new();
        let mut ws = WorkspaceEngineV2::new(WorkspaceConfig::Static { count: 2 });
        ws.init(2);
        let mm = MonitorManager::new();
        let im = InputManager::new();
        let comp = Compositor::new();
        let anim = AnimationEngineV2::new();
        let wl = WaylandCompat::new();
        let results = dt.inspect_all(&wc, &fm, &sm, &ws, &mm, &im, &comp, &anim, &wl);
        assert_eq!(results.len(), 7);
        assert!(results.iter().any(|r| r.type_ == InspectorType::Window));
        assert!(results.iter().any(|r| r.type_ == InspectorType::Workspace));
        assert!(results.iter().any(|r| r.type_ == InspectorType::Input));
        assert!(results.iter().any(|r| r.type_ == InspectorType::Frame));
    }

    #[test]
    fn test_profiler_snapshot() {
        let mut pt = ProfilingTool::new();
        pt.start();
        assert!(pt.is_active());
        let snap = pt.take_snapshot("test", 60.0, 16666, 5, 3);
        assert_eq!(snap.label, "test");
        assert!((snap.fps - 60.0).abs() < 0.01);
        assert_eq!(snap.frame_time_us, 16666);
        assert_eq!(snap.window_count, 5);
        assert_eq!(snap.running_animations, 3);
    }

    #[test]
    fn test_profiler_averages() {
        let mut pt = ProfilingTool::new();
        pt.take_snapshot("a", 30.0, 33333, 2, 1);
        pt.take_snapshot("b", 60.0, 16666, 4, 2);
        pt.take_snapshot("c", 90.0, 11111, 6, 3);
        let avg_fps = pt.average_fps(3);
        assert!((avg_fps - 60.0).abs() < 0.01);
        let avg_ft = pt.average_frame_time(3);
        assert!((avg_ft - 20370.0).abs() < 1.0);
    }

    #[test]
    fn test_generate_report() {
        let mut dt = DebugTool::new();
        let wc = WindowCore::new();
        let fm = FocusManager::new();
        let sm = StackingManager::new();
        let result = dt.inspect_window_system(&wc, &fm, &sm);
        let report = dt.generate_report(&[result]);
        assert!(report.contains("EduWM Debug Report"));
        assert!(report.contains("window_count"));
    }

    #[test]
    fn test_profiler_peak_memory() {
        let mut pt = ProfilingTool::new();
        pt.take_snapshot("a", 60.0, 16666, 0, 0);
        let mut snap = pt.last_snapshot().unwrap().clone();
        snap.memory_mb = 100.0;
        pt.snapshots.push(snap);
        pt.take_snapshot("b", 60.0, 16666, 0, 0);
        let snap2 = pt.last_snapshot().unwrap().clone();
        snap2.memory_mb;
        let peak = pt.peak_memory_mb();
        assert!(peak >= 100.0);
    }

    #[test]
    fn test_enable_disable_inspector() {
        let mut dt = DebugTool::new();
        assert!(!dt.is_inspector_enabled(InspectorType::Wayland));
        dt.enable_inspector(InspectorType::Wayland);
        assert!(dt.is_inspector_enabled(InspectorType::Wayland));
        dt.disable_inspector(InspectorType::Wayland);
        assert!(!dt.is_inspector_enabled(InspectorType::Wayland));
    }
}
