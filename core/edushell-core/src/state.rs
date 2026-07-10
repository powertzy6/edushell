// SPDX-License-Identifier: GPL-3.0-or-later

//! # State Management
//!
//! Thread-safe global state with event-driven change
//! notifications. Follows the observer pattern:
//! state changes are published to the event bus,
//! and components react to changes asynchronously.

use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};

use crate::error::EduResult;
use crate::event::{EventBus, SystemEvent};

/// A typed key for accessing state values.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateKey(pub String);

impl StateKey {
    /// Create a new state key.
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl fmt::Display for StateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for StateKey {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// A single value stored in the global state.
#[derive(Debug, Clone)]
pub enum StateValue {
    /// Boolean value.
    Bool(bool),
    /// Integer value (i64).
    Int(i64),
    /// Float value (f64).
    Float(f64),
    /// String value.
    String(String),
    /// List of strings.
    StringList(Vec<String>),
    /// Arbitrary JSON value.
    Json(serde_json::Value),
}

impl fmt::Display for StateValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bool(v) => write!(f, "{v}"),
            Self::Int(v) => write!(f, "{v}"),
            Self::Float(v) => write!(f, "{v}"),
            Self::String(v) => write!(f, "{v}"),
            Self::StringList(v) => write!(f, "{v:?}"),
            Self::Json(v) => write!(f, "{v}"),
        }
    }
}

/// Change notification for a state key.
#[derive(Debug, Clone)]
pub struct StateChange {
    /// The key that changed.
    pub key: StateKey,
    /// The new value.
    pub new_value: StateValue,
    /// The previous value (None if first set).
    pub old_value: Option<StateValue>,
}

/// Thread-safe global state manager.
///
/// Maintains a map of key-value pairs and publishes
/// change events to the event bus whenever a value
/// is updated.
#[derive(Clone)]
pub struct StateManager {
    store: Arc<RwLock<HashMap<StateKey, StateValue>>>,
    event_bus: EventBus,
}

impl StateManager {
    /// Create a new state manager without event bus.
    pub fn new() -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            event_bus: EventBus::new(),
        }
    }

    /// Create a state manager connected to an event bus.
    pub fn with_event_bus(event_bus: EventBus) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            event_bus,
        }
    }

    /// Get a reference to the event bus.
    pub fn event_bus(&self) -> &EventBus {
        &self.event_bus
    }

    /// Set a state value and publish change event.
    pub fn set(&self, key: impl Into<StateKey>, value: StateValue) -> EduResult<()> {
        let key = key.into();
        {
            let mut store = self.store.write().map_err(|e| {
                crate::error::EduError::Unknown(format!("State lock poisoned: {e}"))
            })?;
            store.insert(key.clone(), value.clone());
        }

        self.event_bus.publish(SystemEvent::HealthCheck {
            healthy: true,
            details: format!("State changed: {} = {}", key, value),
        });

        tracing::debug!(
            target: "edushell::state",
            key = %key,
            "State updated"
        );

        Ok(())
    }

    /// Get a state value by key.
    pub fn get(&self, key: impl Into<StateKey>) -> Option<StateValue> {
        let key = key.into();
        let store = self.store.read().ok()?;
        store.get(&key).cloned()
    }

    /// Check if a key exists.
    pub fn contains(&self, key: impl Into<StateKey>) -> bool {
        let key = key.into();
        self.store.read().ok().map_or(false, |s| s.contains_key(&key))
    }

    /// Remove a state key.
    pub fn remove(&self, key: impl Into<StateKey>) -> EduResult<Option<StateValue>> {
        let key = key.into();
        let value = {
            let mut store = self.store.write().map_err(|e| {
                crate::error::EduError::Unknown(format!("State lock poisoned: {e}"))
            })?;
            store.remove(&key)
        };

        if value.is_some() {
            tracing::debug!(target: "edushell::state", key = %key, "State removed");
        }

        Ok(value)
    }

    /// Clear all state.
    pub fn clear(&self) -> EduResult<()> {
        {
            let mut store = self.store.write().map_err(|e| {
                crate::error::EduError::Unknown(format!("State lock poisoned: {e}"))
            })?;
            store.clear();
        }
        tracing::debug!(target: "edushell::state", "State cleared");
        Ok(())
    }

    /// Get the number of stored keys.
    pub fn len(&self) -> usize {
        self.store.read().ok().map_or(0, |s| s.len())
    }

    /// Check if state is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get all state entries.
    pub fn all(&self) -> HashMap<StateKey, StateValue> {
        self.store.read().ok().map_or_else(HashMap::new, |s| s.clone())
    }

    // ── Convenience setters ──

    /// Set a boolean value.
    pub fn set_bool(&self, key: impl Into<StateKey>, value: bool) -> EduResult<()> {
        self.set(key, StateValue::Bool(value))
    }

    /// Set an integer value.
    pub fn set_int(&self, key: impl Into<StateKey>, value: i64) -> EduResult<()> {
        self.set(key, StateValue::Int(value))
    }

    /// Set a float value.
    pub fn set_float(&self, key: impl Into<StateKey>, value: f64) -> EduResult<()> {
        self.set(key, StateValue::Float(value))
    }

    /// Set a string value.
    pub fn set_string(&self, key: impl Into<StateKey>, value: String) -> EduResult<()> {
        self.set(key, StateValue::String(value))
    }

    // ── Convenience getters ──

    /// Get a boolean value.
    pub fn get_bool(&self, key: impl Into<StateKey>) -> Option<bool> {
        match self.get(key) {
            Some(StateValue::Bool(v)) => Some(v),
            _ => None,
        }
    }

    /// Get an integer value.
    pub fn get_int(&self, key: impl Into<StateKey>) -> Option<i64> {
        match self.get(key) {
            Some(StateValue::Int(v)) => Some(v),
            _ => None,
        }
    }

    /// Get a string value.
    pub fn get_string(&self, key: impl Into<StateKey>) -> Option<String> {
        match self.get(key) {
            Some(StateValue::String(v)) => Some(v),
            _ => None,
        }
    }
}

impl Default for StateManager {
    fn default() -> Self {
        Self::new()
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_set_get() {
        let sm = StateManager::new();
        sm.set_bool("test_bool", true).unwrap();
        assert_eq!(sm.get_bool("test_bool"), Some(true));
    }

    #[test]
    fn test_state_get_missing() {
        let sm = StateManager::new();
        assert_eq!(sm.get_bool("nonexistent"), None);
    }

    #[test]
    fn test_state_remove() {
        let sm = StateManager::new();
        sm.set_string("key", "value".into()).unwrap();
        assert!(sm.contains("key"));

        let removed = sm.remove("key").unwrap();
        assert!(removed.is_some());
        assert!(!sm.contains("key"));
    }

    #[test]
    fn test_state_overwrite() {
        let sm = StateManager::new();
        sm.set_int("counter", 1).unwrap();
        sm.set_int("counter", 2).unwrap();
        assert_eq!(sm.get_int("counter"), Some(2));
    }

    #[test]
    fn test_state_clear() {
        let sm = StateManager::new();
        sm.set_bool("a", true).unwrap();
        sm.set_bool("b", false).unwrap();
        assert_eq!(sm.len(), 2);

        sm.clear().unwrap();
        assert_eq!(sm.len(), 0);
    }

    #[test]
    fn test_state_all() {
        let sm = StateManager::new();
        sm.set_string("name", "test".into()).unwrap();
        sm.set_int("count", 42).unwrap();

        let all = sm.all();
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn test_state_value_display() {
        assert_eq!(StateValue::Bool(true).to_string(), "true");
        assert_eq!(StateValue::Int(42).to_string(), "42");
        assert_eq!(StateValue::String("hello".into()).to_string(), "hello");
    }
}
