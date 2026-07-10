use crate::workspace::WorkspaceEngineV2;
use chrono::Utc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrashSeverity {
    Minor,
    Moderate,
    Critical,
    Fatal,
}

#[derive(Debug, Clone)]
pub struct CrashReport {
    pub id: String,
    pub timestamp: String,
    pub severity: CrashSeverity,
    pub component: String,
    pub error_message: String,
    pub backtrace: Option<String>,
    pub session_state: Option<String>,
    pub window_count: u32,
    pub workspace_count: u32,
    pub was_recovered: bool,
    pub recovery_action: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryAction {
    RestartWM,
    RestoreSession,
    RestoreWorkspace,
    RestoreWindows,
    FallbackToMuffin,
    SafeMode,
    CollectLogs,
    NotifyUser,
    Nothing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RecoveryMode {
    Automatic,
    Manual,
    SafeMode,
}

pub struct CrashRecovery {
    crash_log: Vec<CrashReport>,
    max_reports: usize,
    mode: RecoveryMode,
    fallback_to_muffin: bool,
    auto_restore: bool,
    last_session: Option<String>,
    recovery_attempts: u32,
    max_attempts: u32,
}

impl CrashRecovery {
    pub fn new() -> Self {
        Self {
            crash_log: Vec::new(),
            max_reports: 100,
            mode: RecoveryMode::Automatic,
            fallback_to_muffin: true,
            auto_restore: true,
            last_session: None,
            recovery_attempts: 0,
            max_attempts: 3,
        }
    }

    pub fn set_mode(&mut self, mode: RecoveryMode) {
        self.mode = mode;
    }

    pub fn get_mode(&self) -> &RecoveryMode {
        &self.mode
    }

    pub fn set_fallback(&mut self, enabled: bool) {
        self.fallback_to_muffin = enabled;
    }

    pub fn set_auto_restore(&mut self, enabled: bool) {
        self.auto_restore = enabled;
    }

    pub fn record_crash(
        &mut self,
        severity: CrashSeverity,
        component: &str,
        message: &str,
    ) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let report = CrashReport {
            id: id.clone(),
            timestamp: Utc::now().to_rfc3339(),
            severity,
            component: component.to_string(),
            error_message: message.to_string(),
            backtrace: None,
            session_state: self.last_session.clone(),
            window_count: 0,
            workspace_count: 0,
            was_recovered: false,
            recovery_action: String::new(),
        };
        if self.crash_log.len() >= self.max_reports {
            self.crash_log.remove(0);
        }
        self.crash_log.push(report);
        self.recovery_attempts += 1;
        id
    }

    pub fn get_reports(&self) -> &[CrashReport] {
        &self.crash_log
    }

    pub fn get_report(&self, id: &str) -> Option<&CrashReport> {
        self.crash_log.iter().find(|r| r.id == id)
    }

    pub fn clear_reports(&mut self) {
        self.crash_log.clear();
        self.recovery_attempts = 0;
    }

    pub fn report_count(&self) -> usize {
        self.crash_log.len()
    }

    pub fn attempt_recovery(
        &mut self,
        workspace_engine: Option<&WorkspaceEngineV2>,
    ) -> RecoveryAction {
        if !self.can_recover() {
            return self.fallback_action();
        }

        self.recovery_attempts += 1;
        let severity = self.severity_from_attempts();

        if self.mode == RecoveryMode::Manual {
            return RecoveryAction::NotifyUser;
        }

        if self.mode == RecoveryMode::SafeMode {
            return RecoveryAction::SafeMode;
        }

        match severity {
            CrashSeverity::Minor => {
                if self.auto_restore && self.has_session() {
                    RecoveryAction::RestoreSession
                } else {
                    RecoveryAction::RestartWM
                }
            }
            CrashSeverity::Moderate => {
                if workspace_engine.is_some() {
                    RecoveryAction::RestoreWorkspace
                } else {
                    RecoveryAction::CollectLogs
                }
            }
            CrashSeverity::Critical => {
                if self.fallback_to_muffin {
                    RecoveryAction::FallbackToMuffin
                } else {
                    RecoveryAction::SafeMode
                }
            }
            CrashSeverity::Fatal => {
                if self.fallback_to_muffin {
                    RecoveryAction::FallbackToMuffin
                } else {
                    RecoveryAction::Nothing
                }
            }
        }
    }

    pub fn severity_from_attempts(&self) -> CrashSeverity {
        match self.recovery_attempts {
            0 => CrashSeverity::Minor,
            1 => CrashSeverity::Moderate,
            2 => CrashSeverity::Critical,
            _ => CrashSeverity::Fatal,
        }
    }

    pub fn save_session(&mut self, workspace_engine: &WorkspaceEngineV2) -> bool {
        let workspaces: Vec<_> = workspace_engine
            .list_workspaces()
            .iter()
            .map(|ws| {
                let windows: Vec<String> = ws.windows.iter().map(|w| w.to_string()).collect();
                serde_json::json!({
                    "id": ws.id,
                    "name": ws.name,
                    "windows": windows,
                    "monitor": ws.monitor,
                    "is_active": ws.is_active,
                })
            })
            .collect();

        let snapshot = serde_json::json!({
            "timestamp": Utc::now().to_rfc3339(),
            "active_workspace": workspace_engine.active().id,
            "workspaces": workspaces,
        });

        self.last_session = Some(snapshot.to_string());
        true
    }

    pub fn restore_session(&mut self, workspace_engine: &mut WorkspaceEngineV2) -> bool {
        let session = match self.last_session.as_ref() {
            Some(s) => s,
            None => return false,
        };

        let parsed: serde_json::Value = match serde_json::from_str(session) {
            Ok(v) => v,
            Err(_) => return false,
        };

        let active_ws = parsed["active_workspace"].as_u64().unwrap_or(0) as usize;
        let ws_count = parsed["workspaces"]
            .as_array()
            .map(|a| a.len())
            .unwrap_or(0);

        if ws_count == 0 {
            return false;
        }

        workspace_engine.init(ws_count);
        workspace_engine.switch_to(active_ws);
        true
    }

    pub fn get_last_session(&self) -> Option<&str> {
        self.last_session.as_deref()
    }

    pub fn has_session(&self) -> bool {
        self.last_session.is_some()
    }

    pub fn fallback_action(&self) -> RecoveryAction {
        RecoveryAction::FallbackToMuffin
    }

    pub fn can_recover(&self) -> bool {
        self.recovery_attempts < self.max_attempts
    }
}

impl Default for CrashRecovery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::WorkspaceConfig;

    #[test]
    fn test_record_crash() {
        let mut cr = CrashRecovery::new();
        let id = cr.record_crash(CrashSeverity::Minor, "compositor", "test error");
        assert!(!id.is_empty());
        assert_eq!(cr.report_count(), 1);
        let report = cr.get_report(&id).unwrap();
        assert_eq!(report.severity, CrashSeverity::Minor);
        assert_eq!(report.component, "compositor");
    }

    #[test]
    fn test_recovery_attempt_maxed() {
        let mut cr = CrashRecovery::new();
        cr.max_attempts = 2;
        assert!(cr.can_recover());
        cr.record_crash(CrashSeverity::Critical, "wm", "crash1");
        assert!(cr.can_recover());
        cr.record_crash(CrashSeverity::Critical, "wm", "crash2");
        assert!(!cr.can_recover());
        let action = cr.attempt_recovery(None);
        assert_eq!(action, RecoveryAction::FallbackToMuffin);
    }

    #[test]
    fn test_session_save_restore() {
        let mut cr = CrashRecovery::new();
        let mut ws = WorkspaceEngineV2::new(WorkspaceConfig::Static { count: 4 });
        ws.init(4);
        assert!(cr.save_session(&ws));
        assert!(cr.has_session());
        let mut ws2 = WorkspaceEngineV2::new(WorkspaceConfig::Static { count: 1 });
        ws2.init(1);
        assert!(cr.restore_session(&mut ws2));
        assert_eq!(ws2.list_workspaces().len(), 4);
    }

    #[test]
    fn test_severity_escalation() {
        let mut cr = CrashRecovery::new();
        assert_eq!(cr.severity_from_attempts(), CrashSeverity::Minor);
        cr.recovery_attempts = 1;
        assert_eq!(cr.severity_from_attempts(), CrashSeverity::Moderate);
        cr.recovery_attempts = 2;
        assert_eq!(cr.severity_from_attempts(), CrashSeverity::Critical);
        cr.recovery_attempts = 3;
        assert_eq!(cr.severity_from_attempts(), CrashSeverity::Fatal);
    }

    #[test]
    fn test_mode_automatic_manual() {
        let mut cr = CrashRecovery::new();
        assert_eq!(*cr.get_mode(), RecoveryMode::Automatic);
        let action = cr.attempt_recovery(None);
        assert!(
            action == RecoveryAction::RestartWM || action == RecoveryAction::CollectLogs,
            "expected RestartWM or CollectLogs, got {:?}",
            action
        );
        cr.set_mode(RecoveryMode::Manual);
        assert_eq!(*cr.get_mode(), RecoveryMode::Manual);
        assert_eq!(cr.attempt_recovery(None), RecoveryAction::NotifyUser);
        cr.set_mode(RecoveryMode::SafeMode);
        assert_eq!(cr.attempt_recovery(None), RecoveryAction::SafeMode);
    }

    #[test]
    fn test_clear_reports() {
        let mut cr = CrashRecovery::new();
        cr.record_crash(CrashSeverity::Minor, "test", "err");
        cr.record_crash(CrashSeverity::Moderate, "test", "err2");
        assert_eq!(cr.report_count(), 2);
        cr.clear_reports();
        assert_eq!(cr.report_count(), 0);
    }

    #[test]
    fn test_can_recover() {
        let mut cr = CrashRecovery::new();
        cr.max_attempts = 3;
        assert!(cr.can_recover());
        cr.record_crash(CrashSeverity::Minor, "a", "e1");
        assert!(cr.can_recover());
        cr.record_crash(CrashSeverity::Minor, "a", "e2");
        assert!(cr.can_recover());
        cr.record_crash(CrashSeverity::Minor, "a", "e3");
        assert!(!cr.can_recover());
    }

    #[test]
    fn test_fallback_action() {
        let cr = CrashRecovery::new();
        assert_eq!(cr.fallback_action(), RecoveryAction::FallbackToMuffin);
    }
}
