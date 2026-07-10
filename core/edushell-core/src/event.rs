// SPDX-License-Identifier: GPL-3.0-or-later

//! # Event System
//!
//! Event-driven architecture backbone.
//! Provides a global event bus using a broadcast channel
//! for pub/sub communication between all EduShell components.

use std::fmt;
use std::sync::Arc;
use tokio::sync::broadcast;

/// Capacity of the global event bus channel.
const EVENT_BUS_CAPACITY: usize = 1024;

/// Unique identifier for an event source or subscriber.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EventId(pub String);

impl EventId {
    /// Create a new event ID.
    pub fn new(name: &str) -> Self {
        Self(name.to_string())
    }
}

impl fmt::Display for EventId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ── System Events ───────────────────────────────────────────────

/// All events that can flow through the EduShell event bus.
#[derive(Debug, Clone)]
pub enum SystemEvent {
    // ── Window events ──
    /// A window was created.
    WindowCreated(WindowEventData),
    /// A window was closed.
    WindowClosed(WindowEventData),
    /// Input focus changed to a different window.
    FocusChanged(WindowEventData),
    /// A window's workspace changed.
    WindowWorkspaceChanged { window_id: String, workspace: u32 },

    // ── Workspace events ──
    /// The active workspace changed.
    WorkspaceChanged(u32),
    /// A workspace was added or removed.
    WorkspaceCountChanged(u32),

    // ── Theme events ──
    /// The theme mode changed (light/dark/auto).
    ThemeModeChanged(String),
    /// The accent color changed.
    ThemeAccentChanged(String),
    /// The icon theme changed.
    IconThemeChanged(String),
    /// The wallpaper changed.
    WallpaperChanged(String),

    // ── Language events ──
    /// The display language changed.
    LanguageChanged(String),

    // ── Power events ──
    /// Battery level changed.
    BatteryLevelChanged(f64),
    /// Battery charging state changed.
    BatteryChargingChanged(bool),
    /// Power source changed (AC/battery).
    PowerSourceChanged(String),

    // ── Session events ──
    /// User logged in.
    UserLoggedIn(String),
    /// User logged out.
    UserLoggedOut(String),
    /// Session was locked.
    SessionLocked,
    /// Session was unlocked.
    SessionUnlocked,
    /// Suspend requested.
    SuspendRequested,
    /// System resumed from suspend.
    SystemResumed,
    /// Shutdown requested.
    ShutdownRequested,

    // ── Network events ──
    /// Network connectivity changed.
    NetworkChanged(bool),
    /// WiFi network list changed.
    WiFiNetworksChanged,
    /// Bluetooth state changed.
    BluetoothChanged(bool),

    // ── Audio events ──
    /// System volume changed.
    VolumeChanged(f64),
    /// System brightness changed.
    BrightnessChanged(f64),

    // ── Notification events ──
    /// A notification was added.
    NotificationAdded(NotificationEventData),
    /// A notification was removed.
    NotificationRemoved(uuid::Uuid),

    // ── Application events ──
    /// A new application was installed (from desktop file).
    ApplicationInstalled(String),
    /// An application was removed.
    ApplicationRemoved(String),

    // ── Service events ──
    /// A service changed state.
    ServiceStateChanged { name: String, state: String },
    /// Core system health check result.
    HealthCheck { healthy: bool, details: String },
}

/// Data associated with a window event.
#[derive(Debug, Clone)]
pub struct WindowEventData {
    /// Window identifier (X11/Wayland handle).
    pub id: String,
    /// Application ID (desktop file name).
    pub app_id: String,
    /// Window title.
    pub title: String,
    /// Workspace the window is on.
    pub workspace: u32,
    /// Whether the window is fullscreen.
    pub fullscreen: bool,
    /// Whether the window is minimized.
    pub minimized: bool,
}

/// Data associated with a notification event.
#[derive(Debug, Clone)]
pub struct NotificationEventData {
    /// Unique notification ID.
    pub id: uuid::Uuid,
    /// Application that sent the notification.
    pub app_name: String,
    /// Notification summary/title.
    pub summary: String,
    /// Notification body text.
    pub body: String,
    /// Urgency level.
    pub urgency: NotificationUrgency,
    /// Timestamp of the notification.
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Urgency level for notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationUrgency {
    /// Low priority.
    Low,
    /// Normal priority.
    Normal,
    /// Critical priority.
    Critical,
}

// ── Event Bus ───────────────────────────────────────────────────

/// Thread-safe event bus for pub/sub communication.
///
/// Uses a tokio broadcast channel internally.
/// Clone is cheap (shares the same channel).
#[derive(Clone)]
pub struct EventBus {
    tx: broadcast::Sender<SystemEvent>,
}

impl EventBus {
    /// Create a new event bus with default capacity.
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(EVENT_BUS_CAPACITY);
        Self { tx }
    }

    /// Create a new event bus with a custom capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    /// Publish an event to all subscribers.
    ///
    /// Returns the number of subscribers that received the event,
    /// or `0` if there are no subscribers.
    pub fn publish(&self, event: SystemEvent) -> usize {
        let count = match self.tx.send(event) {
            Ok(n) => n,
            Err(_) => {
                tracing::warn!(
                    target: "edushell::event",
                    "Event bus send failed (no receivers or lagging)"
                );
                0
            }
        };
        count
    }

    /// Subscribe to events.
    ///
    /// Returns a receiver that will receive all events published
    /// after this subscription is created.
    pub fn subscribe(&self) -> broadcast::Receiver<SystemEvent> {
        self.tx.subscribe()
    }

    /// Get the number of active subscribers.
    pub fn subscriber_count(&self) -> usize {
        self.tx.receiver_count()
    }

    /// Create a filtered subscription that only receives events
    /// matching the given predicate.
    pub fn subscribe_filtered<F>(&self, filter: F) -> FilteredReceiver<F>
    where
        F: Fn(&SystemEvent) -> bool + Send + 'static,
    {
        FilteredReceiver {
            inner: self.tx.subscribe(),
            filter: Arc::new(filter),
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

// ── Filtered Receiver ───────────────────────────────────────────

/// An event receiver that filters events using a predicate.
pub struct FilteredReceiver<F>
where
    F: Fn(&SystemEvent) -> bool + Send + 'static,
{
    inner: broadcast::Receiver<SystemEvent>,
    filter: Arc<F>,
}

impl<F> FilteredReceiver<F>
where
    F: Fn(&SystemEvent) -> bool + Send + 'static,
{
    /// Receive the next event that matches the filter.
    pub async fn recv(&mut self) -> Option<SystemEvent> {
        loop {
            match self.inner.recv().await {
                Ok(event) => {
                    if (self.filter)(&event) {
                        return Some(event);
                    }
                }
                Err(broadcast::error::RecvError::Closed) => return None,
                Err(broadcast::error::RecvError::Lagged(n)) => {
                    tracing::warn!(
                        target: "edushell::event",
                        "Event receiver lagged by {n} events"
                    );
                    continue;
                }
            }
        }
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_event_bus_publish_subscribe() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let bus = EventBus::new();
            let mut rx = bus.subscribe();

            bus.publish(SystemEvent::SessionLocked);

            let event = tokio::time::timeout(
                std::time::Duration::from_secs(1),
                rx.recv(),
            )
            .await;

            assert!(event.is_ok());
            let event = event.unwrap().unwrap();
            assert!(matches!(event, SystemEvent::SessionLocked));
        });
    }

    #[test]
    fn test_event_bus_multiple_subscribers() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let bus = EventBus::new();
            let mut rx1 = bus.subscribe();
            let mut rx2 = bus.subscribe();

            bus.publish(SystemEvent::SystemResumed);

            let _ = tokio::time::timeout(
                std::time::Duration::from_secs(1),
                rx1.recv(),
            )
            .await
            .unwrap()
            .unwrap();

            let _ = tokio::time::timeout(
                std::time::Duration::from_secs(1),
                rx2.recv(),
            )
            .await
            .unwrap()
            .unwrap();
        });
    }

    #[test]
    fn test_filtered_receiver() {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let bus = EventBus::new();
            let mut rx = bus.subscribe_filtered(|e| {
                matches!(e, SystemEvent::ThemeModeChanged(_))
            });

            bus.publish(SystemEvent::SessionLocked);
            bus.publish(SystemEvent::ThemeModeChanged("dark".into()));

            let event = tokio::time::timeout(
                std::time::Duration::from_secs(1),
                rx.recv(),
            )
            .await
            .unwrap()
            .unwrap();

            assert!(matches!(event, SystemEvent::ThemeModeChanged(m) if m == "dark"));
        });
    }

    #[test]
    fn test_subscriber_count() {
        let bus = EventBus::new();
        assert_eq!(bus.subscriber_count(), 0);

        let _rx1 = bus.subscribe();
        assert_eq!(bus.subscriber_count(), 1);

        let _rx2 = bus.subscribe();
        assert_eq!(bus.subscriber_count(), 2);
    }
}
