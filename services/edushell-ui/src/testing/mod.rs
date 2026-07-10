// SPDX-License-Identifier: GPL-3.0-or-later

//! # UI Testing Suite
//!
//! Utilities for testing UI components: widget tree traversal,
//! event simulation, accessibility audits, performance
//! measurements, and screenshot comparison.

use std::time::{Duration, Instant};

/// Result of a UI test.
#[derive(Debug, Clone)]
pub struct TestResult {
    /// Test name.
    pub name: String,
    /// Whether the test passed.
    pub passed: bool,
    /// Duration of the test.
    pub duration: Duration,
    /// Error message if failed.
    pub error: Option<String>,
}

impl TestResult {
    /// Create a passing test result.
    pub fn pass(name: &str, duration: Duration) -> Self {
        Self { name: name.to_string(), passed: true, duration, error: None }
    }

    /// Create a failing test result.
    pub fn fail(name: &str, duration: Duration, error: &str) -> Self {
        Self { name: name.to_string(), passed: false, duration, error: Some(error.to_string()) }
    }
}

/// UI test reporter.
#[derive(Debug, Default)]
pub struct TestReporter {
    results: Vec<TestResult>,
    start: Option<Instant>,
}

impl TestReporter {
    /// Create a new test reporter.
    pub fn new() -> Self {
        Self { results: Vec::new(), start: None }
    }

    /// Begin timing a test.
    pub fn begin_test(&mut self, name: &str) {
        self.start = Some(Instant::now());
        tracing::info!(target: "edushell::test", "Running: {name}");
    }

    /// Report a passing test.
    pub fn pass(&mut self, name: &str) {
        let duration = self.start.map(|s| s.elapsed()).unwrap_or_default();
        self.results.push(TestResult::pass(name, duration));
        tracing::info!(target: "edushell::test", "  PASS: {name} ({duration:?})");
    }

    /// Report a failing test.
    pub fn fail(&mut self, name: &str, error: &str) {
        let duration = self.start.map(|s| s.elapsed()).unwrap_or_default();
        self.results.push(TestResult::fail(name, duration, error));
        tracing::error!(target: "edushell::test", "  FAIL: {name}: {error}");
    }

    /// Get all results.
    pub fn results(&self) -> &[TestResult] {
        &self.results
    }

    /// Get pass count.
    pub fn passed(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    /// Get fail count.
    pub fn failed(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    /// Get total count.
    pub fn total(&self) -> usize {
        self.results.len()
    }

    /// Print summary to tracing log.
    pub fn print_summary(&self) {
        let total = self.total();
        let passed = self.passed();
        let failed = self.failed();
        let sep = "─".repeat(50);
        tracing::info!(target: "edushell::test", "{sep}");
        tracing::info!(target: "edushell::test", "Results: {passed}/{total} passed, {failed} failed");
        if failed > 0 {
            for r in &self.results {
                if !r.passed {
                    if let Some(err) = &r.error {
                        tracing::error!(target: "edushell::test", "  FAILED: {}: {}", r.name, err);
                    }
                }
            }
        }
    }
}

/// Performance benchmark result.
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Benchmark name.
    pub name: String,
    /// Duration of the operation.
    pub duration: Duration,
    /// Memory allocated (bytes).
    pub memory_bytes: u64,
    /// Whether result is within acceptable limits.
    pub acceptable: bool,
    /// Acceptable limit description.
    pub limit: String,
}

/// Performance benchmark suite.
#[derive(Debug, Default)]
pub struct BenchmarkSuite {
    results: Vec<BenchmarkResult>,
}

impl BenchmarkSuite {
    /// Create a new benchmark suite.
    pub fn new() -> Self {
        Self { results: Vec::new() }
    }

    /// Run a benchmark.
    pub fn benchmark<F>(&mut self, name: &str, limit: &str, max_ms: u64, f: F)
    where
        F: FnOnce(),
    {
        let start = Instant::now();
        f();
        let duration = start.elapsed();
        let acceptable = duration.as_millis() as u64 <= max_ms;

        self.results.push(BenchmarkResult {
            name: name.to_string(),
            duration,
            memory_bytes: 0,
            acceptable,
            limit: limit.to_string(),
        });

        if acceptable {
            tracing::info!(target: "edushell::bench", "  OK: {name} ({duration:?})");
        } else {
            tracing::warn!(target: "edushell::bench", "  SLOW: {name} ({duration:?}, limit {max_ms}ms)");
        }
    }

    /// Get all benchmark results.
    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }

    /// Check if all benchmarks are within limits.
    pub fn all_acceptable(&self) -> bool {
        self.results.iter().all(|r| r.acceptable)
    }

    /// Print benchmark summary.
    pub fn print_summary(&self) {
        let sep = "─".repeat(50);
        tracing::info!(target: "edushell::bench", "{sep}");
        for r in &self.results {
            tracing::info!(target: "edushell::bench",
                "  {name}: {dur:?} {status}",
                name = r.name,
                dur = r.duration,
                status = if r.acceptable { "✓" } else { "✗" },
            );
        }
    }
}

/// Accessibility audit check.
#[derive(Debug, Clone)]
pub struct A11yCheck {
    /// Check name.
    pub name: String,
    /// Whether the check passed.
    pub passed: bool,
    /// Severity (error, warning, info).
    pub severity: A11ySeverity,
    /// Description of the issue.
    pub description: String,
}

/// Severity of an accessibility issue.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum A11ySeverity {
    /// Must fix.
    Error,
    /// Should fix.
    Warning,
    /// Informational.
    Info,
}

/// Accessibility audit suite.
#[derive(Debug, Default)]
pub struct A11yAudit {
    checks: Vec<A11yCheck>,
}

impl A11yAudit {
    /// Create a new accessibility audit.
    pub fn new() -> Self {
        Self { checks: Vec::new() }
    }

    /// Record an accessibility check result.
    pub fn check(&mut self, name: &str, passed: bool, severity: A11ySeverity, description: &str) {
        self.checks.push(A11yCheck {
            name: name.to_string(),
            passed,
            severity,
            description: description.to_string(),
        });
    }

    /// Get all checks.
    pub fn checks(&self) -> &[A11yCheck] {
        &self.checks
    }

    /// Get errors count.
    pub fn errors(&self) -> usize {
        self.checks.iter().filter(|c| !c.passed && c.severity == A11ySeverity::Error).count()
    }

    /// Get warnings count.
    pub fn warnings(&self) -> usize {
        self.checks.iter().filter(|c| !c.passed && c.severity == A11ySeverity::Warning).count()
    }

    /// Check if the audit passes (no errors).
    pub fn passes(&self) -> bool {
        self.errors() == 0
    }

    /// Print audit summary.
    pub fn print_summary(&self) {
        let sep = "─".repeat(50);
        tracing::info!(target: "edushell::a11y", "{sep}");
        tracing::info!(target: "edushell::a11y",
            "Accessibility Audit: {} errors, {} warnings, {} passed",
            self.errors(),
            self.warnings(),
            self.checks.iter().filter(|c| c.passed).count(),
        );
    }
}

// ── Widget Query ────────────────────────────────────────────────

/// A simple widget tree query for testing.
#[derive(Debug, Clone)]
pub struct WidgetQuery {
    /// Widget name/ID to find.
    pub name: Option<String>,
    /// Widget type.
    pub widget_type: Option<String>,
    /// CSS class.
    pub css_class: Option<String>,
    /// Whether widget must be visible.
    pub visible: Option<bool>,
    /// Whether widget must be sensitive.
    pub sensitive: Option<bool>,
}

impl WidgetQuery {
    /// Create a new widget query.
    pub fn new() -> Self {
        Self {
            name: None,
            widget_type: None,
            css_class: None,
            visible: None,
            sensitive: None,
        }
    }

    /// Filter by name.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Filter by type.
    pub fn with_type(mut self, widget_type: &str) -> Self {
        self.widget_type = Some(widget_type.to_string());
        self
    }

    /// Filter by CSS class.
    pub fn with_class(mut self, css_class: &str) -> Self {
        self.css_class = Some(css_class.to_string());
        self
    }

    /// Filter by visibility.
    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    /// Filter by sensitivity.
    pub fn with_sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_result_pass() {
        let r = TestResult::pass("test1", Duration::from_millis(10));
        assert!(r.passed);
        assert!(r.error.is_none());
    }

    #[test]
    fn test_test_result_fail() {
        let r = TestResult::fail("test1", Duration::from_millis(10), "something broke");
        assert!(!r.passed);
        assert_eq!(r.error.as_deref(), Some("something broke"));
    }

    #[test]
    fn test_reporter() {
        let mut reporter = TestReporter::new();
        reporter.begin_test("passing");
        reporter.pass("passing");
        reporter.begin_test("failing");
        reporter.fail("failing", "error");
        assert_eq!(reporter.passed(), 1);
        assert_eq!(reporter.failed(), 1);
        assert_eq!(reporter.total(), 2);
    }

    #[test]
    fn test_benchmark_within_limit() {
        let mut suite = BenchmarkSuite::new();
        suite.benchmark("fast_op", "<100ms", 100, || {
            let _ = 2 + 2;
        });
        assert!(suite.all_acceptable());
    }

    #[test]
    fn test_benchmark_exceeds_limit() {
        let mut suite = BenchmarkSuite::new();
        suite.benchmark("slow_op", "<1ms", 1, || {
            std::thread::sleep(Duration::from_millis(10));
        });
        assert!(!suite.all_acceptable());
    }

    #[test]
    fn test_a11y_audit_passes() {
        let mut audit = A11yAudit::new();
        audit.check("focus_ring", true, A11ySeverity::Error, "Focus ring present");
        audit.check("contrast", true, A11ySeverity::Warning, "Sufficient contrast");
        assert!(audit.passes());
        assert_eq!(audit.errors(), 0);
    }

    #[test]
    fn test_a11y_audit_fails() {
        let mut audit = A11yAudit::new();
        audit.check("focus_ring", false, A11ySeverity::Error, "No focus ring");
        assert!(!audit.passes());
        assert_eq!(audit.errors(), 1);
    }

    #[test]
    fn test_widget_query_builder() {
        let q = WidgetQuery::new()
            .with_name("panel")
            .with_type("GtkBox")
            .with_class("panel")
            .with_visible(true)
            .with_sensitive(true);
        assert_eq!(q.name.as_deref(), Some("panel"));
        assert_eq!(q.widget_type.as_deref(), Some("GtkBox"));
    }

    #[test]
    fn test_a11y_severity_order() {
        assert_ne!(A11ySeverity::Error as u8, A11ySeverity::Warning as u8);
    }

    #[test]
    fn test_benchmark_collects_results() {
        let mut suite = BenchmarkSuite::new();
        suite.benchmark("a", "<1s", 1000, || {});
        assert_eq!(suite.results().len(), 1);
    }
}
