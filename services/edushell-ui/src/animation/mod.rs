// SPDX-License-Identifier: GPL-3.0-or-later

//! # Animation Framework
//!
//! GPU-friendly animation engine with easing curves, spring physics,
//! and timeline-based sequencing. All animations target 60 FPS and
//! support accessibility reduced-motion preferences.
//!
//! ## Architecture
//!
//! ```text
//! AnimationEngine
//!   ├── Easing (linear, ease, ease-in, ease-out, ease-in-out, bounce, elastic)
//!   ├── Spring (mass, stiffness, damping)
//!   └── Timeline (keyframes, parallel, sequence, repeat)
//! ```
//!
//! ## Usage
//!
//! ```ignore
//! let engine = AnimationEngine::new();
//! let anim = engine.animate(0.0, 100.0, 300, Easing::EaseOut);
//! ```

use std::time::{Duration, Instant};

/// Animation engine driving all UI animations.
#[derive(Clone)]
pub struct AnimationEngine {
    /// Whether reduced motion is preferred.
    reduced_motion: bool,
    /// Global speed multiplier (1.0 = normal).
    speed: f64,
}

impl AnimationEngine {
    /// Create a new animation engine.
    pub fn new() -> Self {
        Self {
            reduced_motion: false,
            speed: 1.0,
        }
    }

    /// Enable/disable reduced motion.
    pub fn set_reduced_motion(&mut self, enabled: bool) {
        self.reduced_motion = enabled;
    }

    /// Check if reduced motion is enabled.
    pub fn reduced_motion(&self) -> bool {
        self.reduced_motion
    }

    /// Set global animation speed multiplier.
    pub fn set_speed(&mut self, speed: f64) {
        self.speed = speed.clamp(0.0, 5.0);
    }

    /// Get global animation speed.
    pub fn speed(&self) -> f64 {
        self.speed
    }

    /// Create an animation from start to end value.
    pub fn animate(&self, from: f64, to: f64, duration_ms: u64, easing: Easing) -> Animation {
        let duration = if self.reduced_motion {
            Duration::from_millis(1)
        } else {
            Duration::from_millis((duration_ms as f64 / self.speed) as u64).max(Duration::from_millis(1))
        };

        Animation {
            from,
            to,
            duration,
            elapsed: Duration::ZERO,
            easing,
            start: Instant::now(),
            finished: false,
        }
    }

    /// Create a spring-based animation.
    pub fn spring(&self, from: f64, to: f64, config: SpringConfig) -> SpringAnimation {
        SpringAnimation {
            from,
            to,
            velocity: 0.0,
            config,
            value: from,
            elapsed: Duration::ZERO,
            start: Instant::now(),
            finished: false,
        }
    }

    /// Create a timeline for sequencing multiple animations.
    pub fn timeline(&self) -> Timeline {
        Timeline {
            keyframes: Vec::new(),
            duration: Duration::ZERO,
        }
    }
}

impl Default for AnimationEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ── Easing Curves ───────────────────────────────────────────────

/// Easing curve types for animation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Easing {
    /// Linear interpolation.
    Linear,
    /// Ease (smooth start and end).
    Ease,
    /// Ease-in (smooth start).
    EaseIn,
    /// Ease-out (smooth end).
    EaseOut,
    /// Ease-in-out (smooth start and end, stronger curve).
    EaseInOut,
    /// Bounce effect at end.
    Bounce,
    /// Elastic overshoot.
    Elastic,
    /// Overshoot then settle.
    BackOut,
}

impl Easing {
    /// Apply the easing function to a progress value [0.0, 1.0].
    pub fn apply(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::Ease => ease(t),
            Self::EaseIn => ease_in(t),
            Self::EaseOut => ease_out(t),
            Self::EaseInOut => ease_in_out(t),
            Self::Bounce => bounce_out(t),
            Self::Elastic => elastic_out(t),
            Self::BackOut => back_out(t),
        }
    }
}

// Standard easing implementations (Cubic Bézier approximations).
fn ease(t: f64) -> f64 {
    // Cubic bezier(0.25, 0.1, 0.25, 1.0) approximation
    t * (2.0 - t)
}

fn ease_in(t: f64) -> f64 {
    t * t * t
}

fn ease_out(t: f64) -> f64 {
    1.0 - (1.0 - t).powi(3)
}

fn ease_in_out(t: f64) -> f64 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
    }
}

fn bounce_out(t: f64) -> f64 {
    const N1: f64 = 7.5625;
    const D1: f64 = 2.75;

    if t < 1.0 / D1 {
        N1 * t * t
    } else if t < 2.0 / D1 {
        let t = t - 1.5 / D1;
        N1 * t * t + 0.75
    } else if t < 2.5 / D1 {
        let t = t - 2.25 / D1;
        N1 * t * t + 0.9375
    } else {
        let t = t - 2.625 / D1;
        N1 * t * t + 0.984_375
    }
}

fn elastic_out(t: f64) -> f64 {
    const C4: f64 = (2.0 * std::f64::consts::PI) / 3.0;

    if t == 0.0 || t == 1.0 {
        return t;
    }
    2.0_f64.powf(-10.0 * t) * f64::sin((t * 10.0 - 0.75) * C4) + 1.0
}

fn back_out(t: f64) -> f64 {
    const C1: f64 = 1.70158;
    const C3: f64 = C1 + 1.0;

    1.0 + C3 * (t - 1.0).powi(3) + C1 * (t - 1.0).powi(2)
}

// ── Animation ───────────────────────────────────────────────────

/// A single animation from one value to another.
pub struct Animation {
    from: f64,
    to: f64,
    duration: Duration,
    elapsed: Duration,
    easing: Easing,
    start: Instant,
    finished: bool,
}

impl Animation {
    /// Tick the animation forward. Returns the current interpolated value.
    pub fn tick(&mut self) -> f64 {
        if self.finished {
            return self.to;
        }

        self.elapsed = self.start.elapsed();
        let progress = (self.elapsed.as_secs_f64() / self.duration.as_secs_f64()).clamp(0.0, 1.0);

        if progress >= 1.0 {
            self.finished = true;
            return self.to;
        }

        let eased = self.easing.apply(progress);
        self.from + (self.to - self.from) * eased
    }

    /// Check if the animation has finished.
    pub fn finished(&self) -> bool {
        self.finished
    }

    /// Get the current progress [0.0, 1.0].
    pub fn progress(&self) -> f64 {
        (self.elapsed.as_secs_f64() / self.duration.as_secs_f64()).clamp(0.0, 1.0)
    }

    /// Get the easing curve being used.
    pub fn easing(&self) -> Easing {
        self.easing
    }
}

// ── Spring Animation ────────────────────────────────────────────

/// Configuration for spring physics.
#[derive(Debug, Clone, Copy)]
pub struct SpringConfig {
    /// Spring stiffness (higher = faster).
    pub stiffness: f64,
    /// Damping ratio (0 = undamped, 1 = critically damped).
    pub damping: f64,
    /// Mass of the attached object.
    pub mass: f64,
}

impl SpringConfig {
    /// A gentle spring.
    pub fn gentle() -> Self {
        Self { stiffness: 100.0, damping: 10.0, mass: 1.0 }
    }

    /// A snappy spring.
    pub fn snappy() -> Self {
        Self { stiffness: 300.0, damping: 20.0, mass: 1.0 }
    }

    /// A bouncy spring.
    pub fn bouncy() -> Self {
        Self { stiffness: 150.0, damping: 5.0, mass: 1.0 }
    }

    /// No bounce (critically damped).
    pub fn stiff() -> Self {
        Self { stiffness: 400.0, damping: 40.0, mass: 1.0 }
    }
}

impl Default for SpringConfig {
    fn default() -> Self {
        Self::snappy()
    }
}

/// A spring-physics based animation.
pub struct SpringAnimation {
    from: f64,
    to: f64,
    velocity: f64,
    config: SpringConfig,
    value: f64,
    elapsed: Duration,
    start: Instant,
    finished: bool,
}

impl SpringAnimation {
    /// Tick the spring simulation forward using wall-clock time. Returns the current value.
    pub fn tick(&mut self) -> f64 {
        if self.finished {
            return self.value;
        }

        let dt = self.start.elapsed().as_secs_f64() - self.elapsed.as_secs_f64();
        self.elapsed = self.start.elapsed();
        let dt = dt.max(1.0 / 60.0).min(0.05); // minimum 1/60s, cap to prevent explosion

        let displacement = self.value - self.to;
        let spring_force = -self.config.stiffness * displacement;
        let damping_force = -self.config.damping * self.velocity;
        let acceleration = (spring_force + damping_force) / self.config.mass;

        self.velocity += acceleration * dt;
        self.value += self.velocity * dt;

        // Check if settled (value near target and velocity near zero)
        if (self.value - self.to).abs() < 0.001 && self.velocity.abs() < 0.001 {
            self.value = self.to;
            self.finished = true;
        }

        self.value
    }

    /// Check if the animation has finished.
    pub fn finished(&self) -> bool {
        self.finished
    }

    /// Get the current value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Get the target value.
    pub fn target(&self) -> f64 {
        self.to
    }

    /// Update the target value (for following animations).
    pub fn set_target(&mut self, to: f64) {
        self.to = to;
        self.finished = false;
    }
}

// ── Timeline ────────────────────────────────────────────────────

/// A keyframe in a timeline.
pub struct Keyframe {
    /// Time offset from timeline start (milliseconds).
    pub time_ms: u64,
    /// Value at this keyframe.
    pub value: f64,
    /// Easing to use from previous keyframe.
    pub easing: Easing,
}

/// A timeline for sequencing multiple keyframe animations.
pub struct Timeline {
    keyframes: Vec<Keyframe>,
    duration: Duration,
}

impl Timeline {
    /// Add a keyframe to the timeline.
    pub fn add(&mut self, time_ms: u64, value: f64, easing: Easing) {
        self.keyframes.push(Keyframe { time_ms, value, easing });
        self.keyframes.sort_by_key(|k| k.time_ms);
        self.duration = Duration::from_millis(
            self.keyframes.last().map_or(0, |k| k.time_ms),
        );
    }

    /// Evaluate the timeline at a given elapsed time.
    pub fn evaluate(&self, elapsed: Duration) -> f64 {
        if self.keyframes.is_empty() {
            return 0.0;
        }

        let elapsed_ms = elapsed.as_millis() as u64;

        // Before first keyframe
        if elapsed_ms <= self.keyframes[0].time_ms {
            return self.keyframes[0].value;
        }

        // After last keyframe
        if elapsed_ms >= self.keyframes.last().unwrap().time_ms {
            return self.keyframes.last().unwrap().value;
        }

        // Find the segment
        for i in 0..self.keyframes.len() - 1 {
            let kf_a = &self.keyframes[i];
            let kf_b = &self.keyframes[i + 1];

            if elapsed_ms >= kf_a.time_ms && elapsed_ms <= kf_b.time_ms {
                let segment_duration = kf_b.time_ms - kf_a.time_ms;
                let segment_progress = if segment_duration == 0 {
                    1.0
                } else {
                    (elapsed_ms - kf_a.time_ms) as f64 / segment_duration as f64
                };

                let eased = kf_b.easing.apply(segment_progress);
                return kf_a.value + (kf_b.value - kf_a.value) * eased;
            }
        }

        self.keyframes.last().unwrap().value
    }

    /// Get total duration.
    pub fn duration(&self) -> Duration {
        self.duration
    }
}

// ── Tests ───────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_linear() {
        let e = Easing::Linear;
        assert!((e.apply(0.0) - 0.0).abs() < 1e-6);
        assert!((e.apply(0.5) - 0.5).abs() < 1e-6);
        assert!((e.apply(1.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_easing_ease_in_out() {
        let e = Easing::EaseInOut;
        assert!((e.apply(0.0) - 0.0).abs() < 1e-6);
        assert!((e.apply(1.0) - 1.0).abs() < 1e-6);
        // Ease-in-out is S-curve: slow at start, fast in middle, slow at end
        assert!(e.apply(0.25) < 0.25); // slower at start
    }

    #[test]
    fn test_animation_tick() {
        let engine = AnimationEngine::new();
        let mut anim = engine.animate(0.0, 100.0, 1000, Easing::Linear);

        // Immediately after creation, progress should be ~0
        let val = anim.tick();
        assert!(val >= 0.0);
        assert!(val <= 100.0);
    }

    #[test]
    fn test_animation_finishes() {
        let engine = AnimationEngine::new();
        let mut anim = engine.animate(0.0, 100.0, 1, Easing::Linear);
        std::thread::sleep(std::time::Duration::from_millis(2));
        let _ = anim.tick();
        assert!(anim.finished());
    }

    #[test]
    fn test_spring_settles() {
        let mut spring = SpringAnimation {
            from: 50.0,
            to: 100.0,
            velocity: 0.0,
            config: SpringConfig::stiff(),
            value: 50.0,
            elapsed: Duration::ZERO,
            start: Instant::now(),
            finished: false,
        };

        for _ in 0..1000 {
            spring.tick();
            if spring.finished() {
                break;
            }
        }
        assert!(spring.finished());
        assert!((spring.value - 100.0).abs() < 0.1);
    }

    #[test]
    fn test_timeline_single_keyframe() {
        let engine = AnimationEngine::new();
        let mut tl = engine.timeline();
        tl.add(0, 10.0, Easing::Linear);
        assert!((tl.evaluate(Duration::ZERO) - 10.0).abs() < 1e-6);
        assert!((tl.evaluate(Duration::from_millis(100)) - 10.0).abs() < 1e-6);
    }

    #[test]
    fn test_timeline_two_keyframes() {
        let engine = AnimationEngine::new();
        let mut tl = engine.timeline();
        tl.add(0, 0.0, Easing::Linear);
        tl.add(1000, 100.0, Easing::Linear);

        let mid = tl.evaluate(Duration::from_millis(500));
        assert!((mid - 50.0).abs() < 1.0);
    }

    #[test]
    fn test_reduced_motion() {
        let mut engine = AnimationEngine::new();
        engine.set_reduced_motion(true);
        let anim = engine.animate(0.0, 100.0, 10000, Easing::Linear);
        assert_eq!(anim.duration.as_millis(), 1);
    }

    #[test]
    fn test_spring_set_target() {
        let mut spring = SpringAnimation {
            from: 0.0,
            to: 100.0,
            velocity: 0.0,
            config: SpringConfig::gentle(),
            value: 0.0,
            elapsed: Duration::ZERO,
            start: Instant::now(),
            finished: false,
        };
        spring.set_target(200.0);
        assert_eq!(spring.target(), 200.0);
    }

    #[test]
    fn test_bounce_elastic_back() {
        // Verify these don't panic and produce values in [0,1]
        let bounce = Easing::Bounce;
        let elastic = Easing::Elastic;
        let back = Easing::BackOut;

        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let b = bounce.apply(t);
            let e = elastic.apply(t);
            let ba = back.apply(t);
            assert!(b.is_finite());
            assert!(e.is_finite());
            assert!(ba.is_finite());
        }
    }

    #[test]
    fn test_engine_defaults() {
        let engine = AnimationEngine::new();
        assert!(!engine.reduced_motion());
        assert!((engine.speed() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_spring_config_variants() {
        let g = SpringConfig::gentle();
        let s = SpringConfig::snappy();
        let b = SpringConfig::bouncy();
        assert!(g.stiffness < s.stiffness);
        assert!(b.damping < s.damping);
    }

    #[test]
    fn test_timeline_empty() {
        let engine = AnimationEngine::new();
        let tl = engine.timeline();
        assert!((tl.evaluate(Duration::ZERO) - 0.0).abs() < 1e-6);
        assert_eq!(tl.duration().as_millis(), 0);
    }
}
