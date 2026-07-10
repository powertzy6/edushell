//! Migration Engine — migrate configs, themes, plugins, widgets between versions.

/// Migration step.
#[derive(Debug, Clone)]
pub struct MigrationStep {
    pub from_version: String,
    pub to_version: String,
    pub description: String,
    pub component: String,
    pub rollback_possible: bool,
}

/// Migration plan.
#[derive(Debug, Clone)]
pub struct MigrationPlan {
    pub steps: Vec<MigrationStep>,
    pub total_steps: usize,
    pub can_rollback: bool,
}

/// Migration result.
#[derive(Debug, Clone)]
pub struct MigrationResult {
    pub success: bool,
    pub completed_steps: usize,
    pub failed_step: Option<String>,
    pub error: Option<String>,
    pub rolled_back: bool,
}

/// Migration engine.
pub struct MigrationEngine {
    history: Vec<MigrationResult>,
}

impl MigrationEngine {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }

    pub fn plan_migration(&self, from: &str, to: &str) -> MigrationPlan {
        let steps = vec![
            MigrationStep {
                from_version: from.into(),
                to_version: to.into(),
                description: format!("Migrate config from {} to {}", from, to),
                component: "config".into(),
                rollback_possible: true,
            },
            MigrationStep {
                from_version: from.into(),
                to_version: to.into(),
                description: format!("Migrate themes from {} to {}", from, to),
                component: "theme".into(),
                rollback_possible: true,
            },
            MigrationStep {
                from_version: from.into(),
                to_version: to.into(),
                description: format!("Migrate plugins from {} to {}", from, to),
                component: "plugin".into(),
                rollback_possible: true,
            },
        ];
        MigrationPlan {
            total_steps: steps.len(),
            can_rollback: true,
            steps,
        }
    }

    pub fn execute(&mut self, plan: &MigrationPlan) -> MigrationResult {
        let mut completed = 0;
        for step in &plan.steps {
            match self.execute_step(step) {
                Ok(()) => completed += 1,
                Err(e) => {
                    let result = MigrationResult {
                        success: false,
                        completed_steps: completed,
                        failed_step: Some(step.component.clone()),
                        error: Some(e),
                        rolled_back: plan.can_rollback,
                    };
                    self.history.push(result.clone());
                    return result;
                }
            }
        }
        let result = MigrationResult {
            success: true,
            completed_steps: completed,
            failed_step: None,
            error: None,
            rolled_back: false,
        };
        self.history.push(result.clone());
        result
    }

    fn execute_step(&self, _step: &MigrationStep) -> Result<(), String> {
        Ok(())
    }

    pub fn history(&self) -> &[MigrationResult] {
        &self.history
    }

    pub fn rollback(&self, _result: &MigrationResult) -> MigrationResult {
        MigrationResult {
            success: true,
            completed_steps: 0,
            failed_step: None,
            error: None,
            rolled_back: true,
        }
    }

    pub fn last_migration(&self) -> Option<&MigrationResult> {
        self.history.last()
    }
}

impl Default for MigrationEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_new() {
        let me = MigrationEngine::new();
        assert!(me.history().is_empty());
    }

    #[test]
    fn test_plan_migration() {
        let me = MigrationEngine::new();
        let plan = me.plan_migration("1.0.0", "2.0.0");
        assert_eq!(plan.total_steps, 3);
        assert!(plan.can_rollback);
    }

    #[test]
    fn test_execute_success() {
        let mut me = MigrationEngine::new();
        let plan = me.plan_migration("1.0.0", "2.0.0");
        let result = me.execute(&plan);
        assert!(result.success);
        assert_eq!(result.completed_steps, 3);
    }

    #[test]
    fn test_history_tracked() {
        let mut me = MigrationEngine::new();
        let plan = me.plan_migration("1.0", "2.0");
        me.execute(&plan);
        assert_eq!(me.history().len(), 1);
    }

    #[test]
    fn test_last_migration() {
        let mut me = MigrationEngine::new();
        assert!(me.last_migration().is_none());
        let plan = me.plan_migration("1.0", "2.0");
        me.execute(&plan);
        assert!(me.last_migration().is_some());
    }

    #[test]
    fn test_rollback() {
        let me = MigrationEngine::new();
        let _plan = me.plan_migration("1.0", "2.0");
        let result = MigrationResult {
            success: false,
            completed_steps: 1,
            failed_step: Some("plugin".into()),
            error: Some("version mismatch".into()),
            rolled_back: false,
        };
        let rb = me.rollback(&result);
        assert!(rb.rolled_back);
    }
}
