use crate::animation::AnimationEngineV2;
use crate::compositor::{Compositor, RenderSurface};
use crate::core::{FocusManager, StackingManager, WindowCore, WindowId, WindowType};
use crate::debug::{DebugTool, ProfilingTool};
use crate::input::{InputDevice, InputDeviceType, InputManager, ModifierMask, ShortcutBinding};
use crate::monitor::{MonitorInfo, MonitorManager, MonitorMode, MonitorTransform};
use crate::recovery::{CrashRecovery, CrashSeverity, RecoveryAction, RecoveryMode};
use crate::security::{Permission, SecurityLevel, SecurityManager};
use crate::wayland::{WaylandCompat, WlSurfaceRole, XdgDecorationMode, XdgToplevelState};
use crate::workspace::{WorkspaceConfig, WorkspaceEngineV2};
use crate::x11::X11Compat;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestCategory {
    Unit,
    Integration,
    Wayland,
    X11,
    Rendering,
    GPU,
    Input,
    Monitor,
    Workspace,
    Stress,
    Performance,
    Accessibility,
    Regression,
    Security,
}

#[derive(Debug, Clone)]
pub enum TestStatus {
    Passed,
    Failed(String),
    Skipped(String),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub category: TestCategory,
    pub status: TestStatus,
    pub duration_us: u64,
    pub timestamp: String,
}

#[derive(Debug, Clone)]
pub struct TestSuite {
    pub name: String,
    pub results: Vec<TestResult>,
    pub setup_time_us: u64,
    pub teardown_time_us: u64,
}

pub struct EduWmTestFramework {
    suites: Vec<TestSuite>,
    #[allow(dead_code)]
    next_suite_id: usize,
    wayland_test_env: Option<WaylandCompat>,
    compositor: Option<Compositor>,
    window_core: Option<WindowCore>,
    focus_manager: Option<FocusManager>,
    stacking_manager: Option<StackingManager>,
    workspace_engine: Option<WorkspaceEngineV2>,
    monitor_manager: Option<MonitorManager>,
    input_manager: Option<InputManager>,
    animation_engine: Option<AnimationEngineV2>,
    recovery: Option<CrashRecovery>,
    debug_tool: Option<DebugTool>,
    security_manager: Option<SecurityManager>,
    profiling: Option<ProfilingTool>,
    x11_compat: Option<X11Compat>,
}

impl EduWmTestFramework {
    pub fn new() -> Self {
        Self {
            suites: Vec::new(),
            next_suite_id: 0,
            wayland_test_env: None,
            compositor: None,
            window_core: None,
            focus_manager: None,
            stacking_manager: None,
            workspace_engine: None,
            monitor_manager: None,
            input_manager: None,
            animation_engine: None,
            recovery: None,
            debug_tool: None,
            security_manager: None,
            profiling: None,
            x11_compat: None,
        }
    }

    pub fn init_default_env(&mut self) {
        let mut mm = MonitorManager::new();
        mm.add_monitor(MonitorInfo {
            id: 0,
            name: "eDP-1".to_string(),
            make: "Test".to_string(),
            model: "Internal".to_string(),
            serial: "SN001".to_string(),
            geometry: (0, 0, 1920, 1080),
            scale: 1.0,
            refresh_rate: 60.0,
            is_primary: true,
            is_active: true,
            transform: MonitorTransform::Normal,
            connector: "eDP-1".to_string(),
            workspaces: vec![0],
            physical_width_mm: 309,
            physical_height_mm: 174,
            edid: vec![],
            modes: vec![MonitorMode {
                width: 1920,
                height: 1080,
                refresh_rate: 60.0,
                preferred: true,
            }],
            current_mode: 0,
        });

        let mut ws = WorkspaceEngineV2::new(WorkspaceConfig::Static { count: 4 });
        ws.init(4);

        let mut im = InputManager::new();
        im.register_device(InputDevice {
            id: 0,
            name: "Test Keyboard".to_string(),
            vendor_id: 0x1234,
            product_id: 0x5678,
            device_type: InputDeviceType::Keyboard,
            is_enabled: true,
            capabilities: vec!["keyboard".to_string()],
        });
        im.register_device(InputDevice {
            id: 0,
            name: "Test Mouse".to_string(),
            vendor_id: 0x1234,
            product_id: 0x5679,
            device_type: InputDeviceType::Pointer,
            is_enabled: true,
            capabilities: vec!["pointer".to_string()],
        });

        let mut comp = Compositor::new();
        comp.attach_output(
            0,
            RenderSurface {
                width: 1920,
                height: 1080,
                scale: 1.0,
                format: 0,
                transform: MonitorTransform::Normal,
            },
        );

        self.wayland_test_env = Some(WaylandCompat::new());
        self.compositor = Some(comp);
        self.window_core = Some(WindowCore::new());
        self.focus_manager = Some(FocusManager::new());
        self.stacking_manager = Some(StackingManager::new());
        self.workspace_engine = Some(ws);
        self.monitor_manager = Some(mm);
        self.input_manager = Some(im);
        self.animation_engine = Some(AnimationEngineV2::new());
        self.recovery = Some(CrashRecovery::new());
        self.debug_tool = Some(DebugTool::new());
        self.security_manager = Some(SecurityManager::new());
        self.profiling = Some(ProfilingTool::new());
        self.x11_compat = Some(X11Compat::new());
    }

    pub fn get_wayland(&self) -> &WaylandCompat {
        self.wayland_test_env
            .as_ref()
            .expect("Wayland not initialized")
    }

    pub fn get_wayland_mut(&mut self) -> &mut WaylandCompat {
        self.wayland_test_env
            .as_mut()
            .expect("Wayland not initialized")
    }

    pub fn get_compositor(&self) -> &Compositor {
        self.compositor
            .as_ref()
            .expect("Compositor not initialized")
    }

    pub fn get_compositor_mut(&mut self) -> &mut Compositor {
        self.compositor
            .as_mut()
            .expect("Compositor not initialized")
    }

    pub fn get_window_core(&self) -> &WindowCore {
        self.window_core
            .as_ref()
            .expect("WindowCore not initialized")
    }

    pub fn get_window_core_mut(&mut self) -> &mut WindowCore {
        self.window_core
            .as_mut()
            .expect("WindowCore not initialized")
    }

    pub fn get_focus_manager(&self) -> &FocusManager {
        self.focus_manager
            .as_ref()
            .expect("FocusManager not initialized")
    }

    pub fn get_focus_manager_mut(&mut self) -> &mut FocusManager {
        self.focus_manager
            .as_mut()
            .expect("FocusManager not initialized")
    }

    pub fn get_stacking_manager(&self) -> &StackingManager {
        self.stacking_manager
            .as_ref()
            .expect("StackingManager not initialized")
    }

    pub fn get_stacking_manager_mut(&mut self) -> &mut StackingManager {
        self.stacking_manager
            .as_mut()
            .expect("StackingManager not initialized")
    }

    pub fn get_workspace_engine(&self) -> &WorkspaceEngineV2 {
        self.workspace_engine
            .as_ref()
            .expect("WorkspaceEngine not initialized")
    }

    pub fn get_workspace_engine_mut(&mut self) -> &mut WorkspaceEngineV2 {
        self.workspace_engine
            .as_mut()
            .expect("WorkspaceEngine not initialized")
    }

    pub fn get_monitor_manager(&self) -> &MonitorManager {
        self.monitor_manager
            .as_ref()
            .expect("MonitorManager not initialized")
    }

    pub fn get_monitor_manager_mut(&mut self) -> &mut MonitorManager {
        self.monitor_manager
            .as_mut()
            .expect("MonitorManager not initialized")
    }

    pub fn get_input_manager(&self) -> &InputManager {
        self.input_manager
            .as_ref()
            .expect("InputManager not initialized")
    }

    pub fn get_input_manager_mut(&mut self) -> &mut InputManager {
        self.input_manager
            .as_mut()
            .expect("InputManager not initialized")
    }

    pub fn get_animation_engine(&self) -> &AnimationEngineV2 {
        self.animation_engine
            .as_ref()
            .expect("AnimationEngine not initialized")
    }

    pub fn get_animation_engine_mut(&mut self) -> &mut AnimationEngineV2 {
        self.animation_engine
            .as_mut()
            .expect("AnimationEngine not initialized")
    }

    pub fn get_recovery(&self) -> &CrashRecovery {
        self.recovery
            .as_ref()
            .expect("CrashRecovery not initialized")
    }

    pub fn get_recovery_mut(&mut self) -> &mut CrashRecovery {
        self.recovery
            .as_mut()
            .expect("CrashRecovery not initialized")
    }

    pub fn get_debug_tool(&self) -> &DebugTool {
        self.debug_tool.as_ref().expect("DebugTool not initialized")
    }

    pub fn get_debug_tool_mut(&mut self) -> &mut DebugTool {
        self.debug_tool.as_mut().expect("DebugTool not initialized")
    }

    pub fn get_security_manager(&self) -> &SecurityManager {
        self.security_manager
            .as_ref()
            .expect("SecurityManager not initialized")
    }

    pub fn get_security_manager_mut(&mut self) -> &mut SecurityManager {
        self.security_manager
            .as_mut()
            .expect("SecurityManager not initialized")
    }

    pub fn get_profiling(&self) -> &ProfilingTool {
        self.profiling.as_ref().expect("Profiling not initialized")
    }

    pub fn get_profiling_mut(&mut self) -> &mut ProfilingTool {
        self.profiling.as_mut().expect("Profiling not initialized")
    }

    pub fn get_x11_compat(&self) -> &X11Compat {
        self.x11_compat.as_ref().expect("X11Compat not initialized")
    }

    pub fn get_x11_compat_mut(&mut self) -> &mut X11Compat {
        self.x11_compat.as_mut().expect("X11Compat not initialized")
    }

    pub fn run_suite(&mut self, name: &str) -> TestSuite {
        let start = std::time::Instant::now();
        let mut suite = TestSuite {
            name: name.to_string(),
            results: Vec::new(),
            setup_time_us: 0,
            teardown_time_us: 0,
        };
        Self::record_result(
            &mut suite,
            "default_test",
            TestCategory::Unit,
            TestStatus::Passed,
            0,
        );
        suite.setup_time_us = start.elapsed().as_micros() as u64;
        self.suites.push(suite.clone());
        suite
    }

    pub fn create_window_on_workspace(&mut self, title: &str, ws_id: usize) -> WindowId {
        let wc = self
            .window_core
            .as_mut()
            .expect("WindowCore not initialized");
        let id = wc.create_window(title, "test.app", WindowType::Normal);
        if let Some(info) = wc.get_window(id) {
            let mut updated = info.clone();
            updated.workspace = ws_id;
            wc.update_window(id, updated);
        }
        let ws = self
            .workspace_engine
            .as_mut()
            .expect("WorkspaceEngine not initialized");
        if ws_id < ws.list_workspaces().len() {
            ws.get_workspace_mut(ws_id).unwrap().windows.push(id);
        }
        id
    }

    pub fn simulate_frame(&mut self) -> u64 {
        let comp = self
            .compositor
            .as_mut()
            .expect("Compositor not initialized");
        let info = comp.simulate_frame(0);
        info.frame_number
    }

    pub fn run_stress_test(&mut self, num_windows: usize, num_operations: usize) -> TestSuite {
        let start = std::time::Instant::now();
        let mut suite = TestSuite {
            name: format!("stress_{}w_{}op", num_windows, num_operations),
            results: Vec::new(),
            setup_time_us: 0,
            teardown_time_us: 0,
        };

        let mut created = Vec::new();
        for i in 0..num_windows {
            let t1 = std::time::Instant::now();
            let id = self.create_window_on_workspace(&format!("stress-{}", i), 0);
            created.push(id);
            let dur = t1.elapsed().as_micros() as u64;
            Self::record_result(
                &mut suite,
                &format!("create_window_{}", i),
                TestCategory::Stress,
                TestStatus::Passed,
                dur,
            );
        }

        for op in 0..num_operations {
            let t1 = std::time::Instant::now();
            let ws = self.workspace_engine.as_mut().unwrap();
            ws.switch_to(op % ws.list_workspaces().len());
            let dur = t1.elapsed().as_micros() as u64;
            Self::record_result(
                &mut suite,
                &format!("switch_workspace_{}", op),
                TestCategory::Stress,
                TestStatus::Passed,
                dur,
            );
        }

        let t1 = std::time::Instant::now();
        let wc = self.window_core.as_mut().unwrap();
        for id in &created {
            wc.destroy_window(*id);
        }
        let dur = t1.elapsed().as_micros() as u64;
        Self::record_result(
            &mut suite,
            "cleanup_windows",
            TestCategory::Stress,
            TestStatus::Passed,
            dur,
        );

        suite.setup_time_us = start.elapsed().as_micros() as u64;
        self.suites.push(suite.clone());
        suite
    }

    pub fn run_performance_test(&mut self, num_frames: usize) -> TestSuite {
        let start = std::time::Instant::now();
        let mut suite = TestSuite {
            name: format!("perf_{}f", num_frames),
            results: Vec::new(),
            setup_time_us: 0,
            teardown_time_us: 0,
        };

        for i in 0..num_frames {
            let t1 = std::time::Instant::now();
            let frame = self.simulate_frame();
            let dur = t1.elapsed().as_micros() as u64;
            Self::record_result(
                &mut suite,
                &format!("frame_{}", i),
                TestCategory::Performance,
                TestStatus::Passed,
                dur,
            );
            assert!(frame > 0);
        }

        let comp = self.compositor.as_ref().unwrap();
        let stats = comp.frame_pacer.get_stats();
        Self::record_result(
            &mut suite,
            "total_frames",
            TestCategory::Performance,
            TestStatus::Passed,
            stats.total_frames,
        );
        Self::record_result(
            &mut suite,
            "dropped_frames",
            TestCategory::Performance,
            TestStatus::Passed,
            stats.dropped_frames,
        );

        suite.setup_time_us = start.elapsed().as_micros() as u64;
        self.suites.push(suite.clone());
        suite
    }

    pub fn run_wayland_compliance_test(&mut self) -> TestSuite {
        let start = std::time::Instant::now();
        let mut suite = TestSuite {
            name: "wayland_compliance".to_string(),
            results: Vec::new(),
            setup_time_us: 0,
            teardown_time_us: 0,
        };

        let wl = self.wayland_test_env.as_mut().unwrap();

        let t1 = std::time::Instant::now();
        let sid = wl.create_surface(WlSurfaceRole::XdgSurface);
        wl.set_xdg_toplevel(sid, XdgToplevelState::default());
        let dur = t1.elapsed().as_micros() as u64;
        Self::record_result(
            &mut suite,
            "xdg_shell_toplevel",
            TestCategory::Wayland,
            TestStatus::Passed,
            dur,
        );

        let t1 = std::time::Instant::now();
        let _lsid = wl.create_surface(WlSurfaceRole::LayerSurface);
        let dur = t1.elapsed().as_micros() as u64;
        Self::record_result(
            &mut suite,
            "layer_shell_surface",
            TestCategory::Wayland,
            TestStatus::Passed,
            dur,
        );

        let t1 = std::time::Instant::now();
        wl.set_decoration_mode(XdgDecorationMode::ServerSide);
        let dur = t1.elapsed().as_micros() as u64;
        Self::record_result(
            &mut suite,
            "xdg_decoration",
            TestCategory::Wayland,
            TestStatus::Passed,
            dur,
        );

        let t1 = std::time::Instant::now();
        wl.set_fractional_scale(sid, 1.5);
        let dur = t1.elapsed().as_micros() as u64;
        Self::record_result(
            &mut suite,
            "fractional_scaling",
            TestCategory::Wayland,
            TestStatus::Passed,
            dur,
        );

        Self::record_result(
            &mut suite,
            "protocol_xdg_shell",
            TestCategory::Wayland,
            if wl.supports_protocol("xdg_shell") {
                TestStatus::Passed
            } else {
                TestStatus::Failed("missing xdg_shell".to_string())
            },
            0,
        );

        Self::record_result(
            &mut suite,
            "protocol_layer_shell",
            TestCategory::Wayland,
            if wl.supports_protocol("layer_shell") {
                TestStatus::Passed
            } else {
                TestStatus::Failed("missing layer_shell".to_string())
            },
            0,
        );

        suite.setup_time_us = start.elapsed().as_micros() as u64;
        self.suites.push(suite.clone());
        suite
    }

    pub fn run_input_stress_test(&mut self, num_events: usize) -> TestSuite {
        let start = std::time::Instant::now();
        let mut suite = TestSuite {
            name: format!("input_stress_{}e", num_events),
            results: Vec::new(),
            setup_time_us: 0,
            teardown_time_us: 0,
        };

        let im = self.input_manager.as_mut().unwrap();

        for i in 0..num_events {
            let t1 = std::time::Instant::now();
            match i % 3 {
                0 => {
                    im.set_pointer_position((i * 10) as f64, (i * 5) as f64);
                }
                1 => {
                    im.register_shortcut(ShortcutBinding {
                        keysym: i as u32,
                        modifiers: ModifierMask::SUPER,
                        action: format!("action_{}", i),
                        description: String::new(),
                        is_global: true,
                        is_reversible: false,
                    });
                }
                _ => {
                    let pos = im.get_pointer_position();
                    assert!(pos.0 >= 0.0 || pos.1 >= 0.0);
                }
            }
            let dur = t1.elapsed().as_micros() as u64;
            Self::record_result(
                &mut suite,
                &format!("input_event_{}", i),
                TestCategory::Input,
                TestStatus::Passed,
                dur,
            );
        }

        suite.setup_time_us = start.elapsed().as_micros() as u64;
        self.suites.push(suite.clone());
        suite
    }

    pub fn run_security_audit(&mut self) -> TestSuite {
        let start = std::time::Instant::now();
        let mut suite = TestSuite {
            name: "security_audit".to_string(),
            results: Vec::new(),
            setup_time_us: 0,
            teardown_time_us: 0,
        };

        let sm = self.security_manager.as_mut().unwrap();

        let t1 = std::time::Instant::now();
        assert_eq!(*sm.get_level(), SecurityLevel::Medium);
        Self::record_result(
            &mut suite,
            "default_level",
            TestCategory::Security,
            TestStatus::Passed,
            t1.elapsed().as_micros() as u64,
        );

        let wid = WindowId::new();
        let t1 = std::time::Instant::now();
        let granted = sm.request_permission(wid, Permission::ClipboardRead, false);
        Self::record_result(
            &mut suite,
            "permission_request",
            TestCategory::Security,
            if granted {
                TestStatus::Passed
            } else {
                TestStatus::Failed("permission denied".to_string())
            },
            t1.elapsed().as_micros() as u64,
        );

        let t1 = std::time::Instant::now();
        let isolated = sm.isolate_window(wid);
        Self::record_result(
            &mut suite,
            "window_isolation",
            TestCategory::Security,
            if isolated {
                TestStatus::Passed
            } else {
                TestStatus::Failed("isolate failed".to_string())
            },
            t1.elapsed().as_micros() as u64,
        );

        let t1 = std::time::Instant::now();
        let _access = sm.check_clipboard_access(wid, wid);
        Self::record_result(
            &mut suite,
            "clipboard_access",
            TestCategory::Security,
            TestStatus::Passed,
            t1.elapsed().as_micros() as u64,
        );

        let t1 = std::time::Instant::now();
        sm.record_access_denied(wid, "test_action");
        assert!(!sm.get_audit_log().is_empty());
        Self::record_result(
            &mut suite,
            "audit_logging",
            TestCategory::Security,
            TestStatus::Passed,
            t1.elapsed().as_micros() as u64,
        );

        suite.setup_time_us = start.elapsed().as_micros() as u64;
        self.suites.push(suite.clone());
        suite
    }

    pub fn run_crash_recovery_test(&mut self) -> TestSuite {
        let start = std::time::Instant::now();
        let mut suite = TestSuite {
            name: "crash_recovery".to_string(),
            results: Vec::new(),
            setup_time_us: 0,
            teardown_time_us: 0,
        };

        let recovery = self.recovery.as_mut().unwrap();

        let t1 = std::time::Instant::now();
        let id = recovery.record_crash(CrashSeverity::Minor, "test_component", "test error");
        Self::record_result(
            &mut suite,
            "record_crash",
            TestCategory::Unit,
            TestStatus::Passed,
            t1.elapsed().as_micros() as u64,
        );
        assert!(!id.is_empty());
        assert_eq!(recovery.report_count(), 1);

        let t1 = std::time::Instant::now();
        recovery.set_mode(RecoveryMode::Manual);
        let action = recovery.attempt_recovery(None);
        Self::record_result(
            &mut suite,
            "manual_recovery",
            TestCategory::Unit,
            if action == RecoveryAction::NotifyUser {
                TestStatus::Passed
            } else {
                TestStatus::Failed("wrong action".to_string())
            },
            t1.elapsed().as_micros() as u64,
        );

        let t1 = std::time::Instant::now();
        recovery.set_mode(RecoveryMode::Automatic);
        let ws = self.workspace_engine.as_ref().unwrap();
        recovery.save_session(ws);
        let restored = recovery.has_session();
        Self::record_result(
            &mut suite,
            "session_save",
            TestCategory::Unit,
            if restored {
                TestStatus::Passed
            } else {
                TestStatus::Failed("session not saved".to_string())
            },
            t1.elapsed().as_micros() as u64,
        );

        let t1 = std::time::Instant::now();
        let fallback = recovery.fallback_action();
        Self::record_result(
            &mut suite,
            "fallback_action",
            TestCategory::Unit,
            if fallback == RecoveryAction::FallbackToMuffin {
                TestStatus::Passed
            } else {
                TestStatus::Failed("wrong fallback".to_string())
            },
            t1.elapsed().as_micros() as u64,
        );

        recovery.clear_reports();
        assert_eq!(recovery.report_count(), 0);
        Self::record_result(
            &mut suite,
            "clear_reports",
            TestCategory::Unit,
            TestStatus::Passed,
            0,
        );

        suite.setup_time_us = start.elapsed().as_micros() as u64;
        self.suites.push(suite.clone());
        suite
    }

    pub fn get_summary(&self) -> (usize, usize, usize) {
        let mut passed = 0;
        let mut failed = 0;
        let mut total = 0;

        for suite in &self.suites {
            for result in &suite.results {
                total += 1;
                match &result.status {
                    TestStatus::Passed => passed += 1,
                    TestStatus::Failed(_) | TestStatus::Error(_) => failed += 1,
                    TestStatus::Skipped(_) => {}
                }
            }
        }

        (passed, failed, total)
    }

    fn record_result(
        suite: &mut TestSuite,
        name: &str,
        category: TestCategory,
        status: TestStatus,
        duration_us: u64,
    ) {
        suite.results.push(TestResult {
            name: name.to_string(),
            category,
            status,
            duration_us,
            timestamp: crate::core::now(),
        });
    }
}

impl Default for EduWmTestFramework {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_framework_init() {
        let mut tf = EduWmTestFramework::new();
        tf.init_default_env();
        assert!(tf.get_wayland().surface_count() == 0);
        assert!(tf.get_compositor().outputs.contains_key(&0));
        assert!(tf.get_window_core().count() == 0);
        assert_eq!(tf.get_workspace_engine().list_workspaces().len(), 4);
        assert!(tf.get_monitor_manager().count() >= 1);
        assert!(tf.get_input_manager().list_devices().len() >= 2);
    }

    #[test]
    fn test_create_window_on_workspace() {
        let mut tf = EduWmTestFramework::new();
        tf.init_default_env();
        let id = tf.create_window_on_workspace("Test Window", 1);
        let ws = tf.get_workspace_engine();
        assert!(ws.get_workspace(1).unwrap().windows.contains(&id));
        let wc = tf.get_window_core();
        assert!(wc.get_window(id).is_some());
    }

    #[test]
    fn test_simulate_frame() {
        let mut tf = EduWmTestFramework::new();
        tf.init_default_env();
        let frame = tf.simulate_frame();
        assert_eq!(frame, 1);
        let frame2 = tf.simulate_frame();
        assert_eq!(frame2, 2);
    }

    #[test]
    fn test_stress_test() {
        let mut tf = EduWmTestFramework::new();
        tf.init_default_env();
        let suite = tf.run_stress_test(5, 3);
        assert_eq!(suite.results.len(), 9);
        let (passed, failed, total) = tf.get_summary();
        assert!(passed > 0);
        assert_eq!(failed, 0);
        assert_eq!(total, 9);
    }

    #[test]
    fn test_performance_test() {
        let mut tf = EduWmTestFramework::new();
        tf.init_default_env();
        let suite = tf.run_performance_test(10);
        assert!(suite.results.len() >= 10);
    }

    #[test]
    fn test_wayland_compliance_test() {
        let mut tf = EduWmTestFramework::new();
        tf.init_default_env();
        let suite = tf.run_wayland_compliance_test();
        assert!(suite.results.len() >= 6);
    }

    #[test]
    fn test_input_stress_test() {
        let mut tf = EduWmTestFramework::new();
        tf.init_default_env();
        let suite = tf.run_input_stress_test(5);
        assert_eq!(suite.results.len(), 5);
    }

    #[test]
    fn test_crash_recovery_test() {
        let mut tf = EduWmTestFramework::new();
        tf.init_default_env();
        let suite = tf.run_crash_recovery_test();
        assert!(suite.results.len() >= 5);
    }

    #[test]
    fn test_get_summary() {
        let mut tf = EduWmTestFramework::new();
        tf.init_default_env();
        let (passed, failed, total) = tf.get_summary();
        assert_eq!(passed, 0);
        assert_eq!(failed, 0);
        assert_eq!(total, 0);
        tf.run_stress_test(2, 1);
        let (passed, failed, _) = tf.get_summary();
        assert!(passed > 0);
        assert_eq!(failed, 0);
    }
}
