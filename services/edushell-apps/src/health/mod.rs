use chrono::Utc;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct HealthRecord {
    pub timestamp: String,
    pub category: HealthCategory,
    pub severity: HealthSeverity,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HealthCategory {
    Cpu,
    Memory,
    Disk,
    Network,
    Service,
    Module,
    Security,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HealthSeverity {
    Info,
    Warning,
    Critical,
}

pub struct HealthMonitor {
    records: VecDeque<HealthRecord>,
    max_records: usize,
    healthy: bool,
}

impl HealthMonitor {
    pub fn new() -> Self {
        Self {
            records: VecDeque::new(),
            max_records: 100,
            healthy: true,
        }
    }

    pub fn record(&mut self, category: HealthCategory, severity: HealthSeverity, message: &str) {
        if severity == HealthSeverity::Critical {
            self.healthy = false;
        }
        if self.records.len() >= self.max_records {
            self.records.pop_front();
        }
        self.records.push_back(HealthRecord {
            timestamp: Utc::now().to_rfc3339(),
            category,
            severity,
            message: message.to_string(),
        });
    }

    pub fn is_healthy(&self) -> bool { self.healthy }
    pub fn records(&self) -> &VecDeque<HealthRecord> { &self.records }
    pub fn clear(&mut self) { self.records.clear(); self.healthy = true; }

    pub fn critical_count(&self) -> usize {
        self.records.iter().filter(|r| r.severity == HealthSeverity::Critical).count()
    }

    pub fn warning_count(&self) -> usize {
        self.records.iter().filter(|r| r.severity == HealthSeverity::Warning).count()
    }

    pub fn last_critical(&self) -> Option<&HealthRecord> {
        self.records.iter().rev().find(|r| r.severity == HealthSeverity::Critical)
    }
}

impl Default for HealthMonitor {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_new() {
        let h = HealthMonitor::new();
        assert!(h.is_healthy());
        assert_eq!(h.records().len(), 0);
    }

    #[test]
    fn test_record_info() {
        let mut h = HealthMonitor::new();
        h.record(HealthCategory::Cpu, HealthSeverity::Info, "cpu at 30%");
        assert!(h.is_healthy());
        assert_eq!(h.records().len(), 1);
    }

    #[test]
    fn test_record_critical() {
        let mut h = HealthMonitor::new();
        h.record(HealthCategory::Memory, HealthSeverity::Critical, "oom");
        assert!(!h.is_healthy());
        assert_eq!(h.critical_count(), 1);
    }

    #[test]
    fn test_clear() {
        let mut h = HealthMonitor::new();
        h.record(HealthCategory::Disk, HealthSeverity::Warning, "disk nearly full");
        h.clear();
        assert!(h.is_healthy());
        assert_eq!(h.records().len(), 0);
    }

    #[test]
    fn test_last_critical() {
        let mut h = HealthMonitor::new();
        assert!(h.last_critical().is_none());
        h.record(HealthCategory::Security, HealthSeverity::Critical, "breach");
        assert!(h.last_critical().is_some());
    }

    #[test]
    fn test_max_records() {
        let mut h = HealthMonitor::new();
        h.max_records = 3;
        for i in 0..5 {
            h.record(HealthCategory::Cpu, HealthSeverity::Info, &format!("event {}", i));
        }
        assert_eq!(h.records().len(), 3);
    }
}
