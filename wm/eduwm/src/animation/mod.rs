use crate::core::WindowId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnimationId(usize);

impl AnimationId {
    pub fn new(id: usize) -> Self {
        Self(id)
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AnimationState {
    Pending,
    Running,
    Paused,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationEasing {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Bounce,
    Elastic,
    BackOut,
    BackIn,
    BackInOut,
    CubicBezier(f64, f64, f64, f64),
}

impl AnimationEasing {
    pub fn evaluate(&self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Self::Linear => t,
            Self::EaseIn => t * t * t,
            Self::EaseOut => 1.0 - (1.0 - t).powi(3),
            Self::EaseInOut => 3.0 * t * t - 2.0 * t * t * t,
            Self::Bounce => {
                if t < 1.0 / 2.75 {
                    7.5625 * t * t
                } else if t < 2.0 / 2.75 {
                    let t2 = t - 1.5 / 2.75;
                    7.5625 * t2 * t2 + 0.75
                } else if t < 2.5 / 2.75 {
                    let t2 = t - 2.25 / 2.75;
                    7.5625 * t2 * t2 + 0.9375
                } else {
                    let t2 = t - 2.625 / 2.75;
                    7.5625 * t2 * t2 + 0.984375
                }
            }
            Self::Elastic => {
                if t == 0.0 || t == 1.0 {
                    return t;
                }
                let p = 0.3;
                let s = p / 4.0;
                -(2.0_f64.powf(10.0 * (t - 1.0)))
                    * ((t - 1.0 - s) * (2.0 * std::f64::consts::PI) / p).sin()
            }
            Self::BackOut => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            }
            Self::BackIn => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            Self::BackInOut => {
                let c1 = 1.70158;
                let c2 = c1 * 1.525;
                if t < 0.5 {
                    ((2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2)) / 2.0
                } else {
                    let t2 = 2.0 * t - 2.0;
                    (t2.powi(2) * ((c2 + 1.0) * t2 + c2) + 2.0) / 2.0
                }
            }
            Self::CubicBezier(x1, y1, x2, y2) => {
                let mut low = 0.0_f64;
                let mut high = 1.0_f64;
                for _ in 0..20 {
                    let mid = (low + high) * 0.5_f64;
                    let x = 3.0_f64 * (1.0_f64 - mid).powi(2) * mid * x1
                        + 3.0_f64 * (1.0_f64 - mid) * mid.powi(2) * x2
                        + mid.powi(3);
                    if x < t {
                        low = mid;
                    } else {
                        high = mid;
                    }
                }
                let u = (low + high) * 0.5_f64;
                3.0_f64 * (1.0_f64 - u).powi(2) * u * y1
                    + 3.0_f64 * (1.0_f64 - u) * u.powi(2) * y2
                    + u.powi(3)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationProperty {
    Opacity,
    Scale(f64, f64),
    Translate(f64, f64),
    Rotate(f64),
    Blur(f64),
    CornerRadius(f64),
    Color(f64, f64, f64, f64),
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnimationTarget {
    Window(WindowId),
    Workspace(usize),
    Launcher,
    Notification,
    Overview,
    Panel,
    Dock,
    Theme,
    Cursor,
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub id: AnimationId,
    pub target: AnimationTarget,
    pub property: AnimationProperty,
    pub from_value: f64,
    pub to_value: f64,
    pub duration_ms: u64,
    pub elapsed_ms: u64,
    pub easing: AnimationEasing,
    pub state: AnimationState,
    pub parallel: bool,
    pub on_complete: Option<String>,
    pub created_at: u64,
}

pub struct AnimationEngineV2 {
    pub animations: Vec<Animation>,
    next_id: usize,
    pub max_animations: usize,
    pub reduced_motion: bool,
    pub adaptive_performance: bool,
    pub gpu_accelerated: bool,
    pub global_speed_multiplier: f64,
}

impl AnimationEngineV2 {
    pub fn new() -> Self {
        Self {
            animations: Vec::new(),
            next_id: 1,
            max_animations: 100,
            reduced_motion: false,
            adaptive_performance: false,
            gpu_accelerated: true,
            global_speed_multiplier: 1.0,
        }
    }

    pub fn create_animation(
        &mut self,
        target: AnimationTarget,
        property: AnimationProperty,
        from_value: f64,
        to_value: f64,
        duration_ms: u64,
        easing: AnimationEasing,
    ) -> AnimationId {
        let id = AnimationId::new(self.next_id);
        self.next_id += 1;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        let anim = Animation {
            id,
            target,
            property,
            from_value,
            to_value,
            duration_ms,
            elapsed_ms: 0,
            easing,
            state: AnimationState::Pending,
            parallel: true,
            on_complete: None,
            created_at: now,
        };
        if self.animations.len() >= self.max_animations {
            self.animations.remove(0);
        }
        self.animations.push(anim);
        id
    }

    pub fn get_animation(&self, id: AnimationId) -> Option<&Animation> {
        self.animations.iter().find(|a| a.id == id)
    }

    pub fn get_animation_mut(&mut self, id: AnimationId) -> Option<&mut Animation> {
        self.animations.iter_mut().find(|a| a.id == id)
    }

    pub fn start_animation(&mut self, id: AnimationId) -> bool {
        if let Some(anim) = self.get_animation_mut(id) {
            if anim.state == AnimationState::Pending || anim.state == AnimationState::Paused {
                anim.state = AnimationState::Running;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn pause_animation(&mut self, id: AnimationId) -> bool {
        if let Some(anim) = self.get_animation_mut(id) {
            if anim.state == AnimationState::Running {
                anim.state = AnimationState::Paused;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn resume_animation(&mut self, id: AnimationId) -> bool {
        if let Some(anim) = self.get_animation_mut(id) {
            if anim.state == AnimationState::Paused {
                anim.state = AnimationState::Running;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn cancel_animation(&mut self, id: AnimationId) -> bool {
        if let Some(anim) = self.get_animation_mut(id) {
            anim.state = AnimationState::Cancelled;
            true
        } else {
            false
        }
    }

    pub fn complete_animation(&mut self, id: AnimationId) -> bool {
        if let Some(anim) = self.get_animation_mut(id) {
            anim.state = AnimationState::Completed;
            anim.elapsed_ms = anim.duration_ms;
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta_ms: u64) {
        if self.reduced_motion {
            for anim in &mut self.animations {
                if anim.state == AnimationState::Running {
                    anim.state = AnimationState::Completed;
                    anim.elapsed_ms = anim.duration_ms;
                }
            }
            return;
        }
        let effective_delta = (delta_ms as f64 * self.global_speed_multiplier) as u64;
        for anim in &mut self.animations {
            if anim.state == AnimationState::Running {
                anim.elapsed_ms = anim.elapsed_ms.saturating_add(effective_delta);
                if anim.elapsed_ms >= anim.duration_ms {
                    anim.elapsed_ms = anim.duration_ms;
                    anim.state = AnimationState::Completed;
                }
            }
        }
    }

    pub fn is_running(&self, id: AnimationId) -> bool {
        self.get_animation(id)
            .map(|a| a.state == AnimationState::Running)
            .unwrap_or(false)
    }

    pub fn is_completed(&self, id: AnimationId) -> bool {
        self.get_animation(id)
            .map(|a| a.state == AnimationState::Completed)
            .unwrap_or(false)
    }

    pub fn running_count(&self) -> usize {
        self.animations
            .iter()
            .filter(|a| a.state == AnimationState::Running)
            .count()
    }

    pub fn total_count(&self) -> usize {
        self.animations.len()
    }

    pub fn set_reduced_motion(&mut self, enabled: bool) -> bool {
        self.reduced_motion = enabled;
        if enabled {
            for anim in &mut self.animations {
                if anim.state == AnimationState::Running {
                    anim.state = AnimationState::Completed;
                    anim.elapsed_ms = anim.duration_ms;
                }
            }
        }
        true
    }

    pub fn get_reduced_motion(&self) -> bool {
        self.reduced_motion
    }

    pub fn set_gpu_accelerated(&mut self, enabled: bool) {
        self.gpu_accelerated = enabled;
    }

    pub fn get_gpu_accelerated(&self) -> bool {
        self.gpu_accelerated
    }

    pub fn set_adaptive_performance(&mut self, enabled: bool) {
        self.adaptive_performance = enabled;
    }

    pub fn get_adaptive_performance(&self) -> bool {
        self.adaptive_performance
    }

    pub fn set_global_speed(&mut self, speed: f64) {
        self.global_speed_multiplier = speed.max(0.1);
    }

    pub fn get_global_speed(&self) -> f64 {
        self.global_speed_multiplier
    }

    pub fn set_max_animations(&mut self, max: usize) {
        self.max_animations = max;
        while self.animations.len() > max {
            self.animations.remove(0);
        }
    }

    pub fn get_max_animations(&self) -> usize {
        self.max_animations
    }

    pub fn animations_for_target(&self, target: AnimationTarget) -> Vec<&Animation> {
        self.animations
            .iter()
            .filter(|a| a.target == target)
            .collect()
    }

    pub fn cancel_all_for_target(&mut self, target: AnimationTarget) {
        for anim in &mut self.animations {
            if anim.target == target {
                anim.state = AnimationState::Cancelled;
            }
        }
    }

    pub fn cancel_all(&mut self) {
        for anim in &mut self.animations {
            anim.state = AnimationState::Cancelled;
        }
    }

    pub fn complete_all(&mut self) {
        for anim in &mut self.animations {
            anim.state = AnimationState::Completed;
            anim.elapsed_ms = anim.duration_ms;
        }
    }

    pub fn current_value(&self, id: AnimationId) -> f64 {
        match self.animations.iter().find(|a| a.id == id) {
            None => 0.0,
            Some(anim) => match anim.state {
                AnimationState::Completed | AnimationState::Cancelled => anim.to_value,
                AnimationState::Pending => anim.from_value,
                _ => {
                    let t = if anim.duration_ms == 0 {
                        1.0
                    } else {
                        (anim.elapsed_ms as f64 / anim.duration_ms as f64).min(1.0)
                    };
                    let eased = anim.easing.evaluate(t);
                    anim.from_value + (anim.to_value - anim.from_value) * eased
                }
            },
        }
    }
}

impl Default for AnimationEngineV2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_easing_functions() {
        let linear = AnimationEasing::Linear;
        assert!((linear.evaluate(0.0) - 0.0).abs() < 1e-6);
        assert!((linear.evaluate(0.5) - 0.5).abs() < 1e-6);
        assert!((linear.evaluate(1.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_create_start_complete_animation() {
        let mut engine = AnimationEngineV2::new();
        let id = engine.create_animation(
            AnimationTarget::Window(WindowId::new()),
            AnimationProperty::Opacity,
            0.0,
            1.0,
            1000,
            AnimationEasing::Linear,
        );
        assert!(engine.get_animation(id).is_some());
        assert!(engine.start_animation(id));
        assert!(engine.is_running(id));
        assert!(engine.complete_animation(id));
        assert!(engine.is_completed(id));
    }

    #[test]
    fn test_pause_resume_animation() {
        let mut engine = AnimationEngineV2::new();
        let id = engine.create_animation(
            AnimationTarget::Panel,
            AnimationProperty::Opacity,
            0.0,
            1.0,
            500,
            AnimationEasing::EaseInOut,
        );
        engine.start_animation(id);
        assert!(engine.is_running(id));
        assert!(engine.pause_animation(id));
        assert!(!engine.is_running(id));
        assert!(engine.resume_animation(id));
        assert!(engine.is_running(id));
    }

    #[test]
    fn test_cancel_animation() {
        let mut engine = AnimationEngineV2::new();
        let id = engine.create_animation(
            AnimationTarget::Notification,
            AnimationProperty::Blur(5.0),
            0.0,
            10.0,
            300,
            AnimationEasing::EaseOut,
        );
        engine.start_animation(id);
        assert!(engine.cancel_animation(id));
        assert!(!engine.is_running(id));
    }

    #[test]
    fn test_animation_update() {
        let mut engine = AnimationEngineV2::new();
        let id = engine.create_animation(
            AnimationTarget::Dock,
            AnimationProperty::Scale(1.0, 1.5),
            0.0,
            1.0,
            100,
            AnimationEasing::Linear,
        );
        engine.start_animation(id);
        engine.update(50);
        let val = engine.current_value(id);
        assert!(val > 0.45 && val < 0.55);
        engine.update(50);
        assert!(engine.is_completed(id));
    }

    #[test]
    fn test_reduced_motion() {
        let mut engine = AnimationEngineV2::new();
        engine.set_reduced_motion(true);
        assert!(engine.get_reduced_motion());
        let id = engine.create_animation(
            AnimationTarget::Overview,
            AnimationProperty::Opacity,
            0.0,
            1.0,
            1000,
            AnimationEasing::Linear,
        );
        engine.start_animation(id);
        engine.update(1);
        assert!(engine.is_completed(id));
    }

    #[test]
    fn test_global_speed() {
        let mut engine = AnimationEngineV2::new();
        engine.set_global_speed(2.0);
        let id = engine.create_animation(
            AnimationTarget::Workspace(0),
            AnimationProperty::Opacity,
            0.0,
            1.0,
            100,
            AnimationEasing::Linear,
        );
        engine.start_animation(id);
        engine.update(25);
        let val = engine.current_value(id);
        assert!((val - 0.5).abs() < 0.05);
    }

    #[test]
    fn test_animation_target_filter() {
        let mut engine = AnimationEngineV2::new();
        let id = engine.create_animation(
            AnimationTarget::Launcher,
            AnimationProperty::Opacity,
            0.0,
            1.0,
            200,
            AnimationEasing::Linear,
        );
        let animations = engine.animations_for_target(AnimationTarget::Launcher);
        assert_eq!(animations.len(), 1);
        assert_eq!(animations[0].id, id);
        let empty = engine.animations_for_target(AnimationTarget::Cursor);
        assert!(empty.is_empty());
    }

    #[test]
    fn test_ease_in_out_symmetric() {
        let easing = AnimationEasing::EaseInOut;
        let v1 = easing.evaluate(0.25);
        let v2 = easing.evaluate(0.75);
        assert!((v1 - 1.0 + v2).abs() < 1e-6);
    }

    #[test]
    fn test_bounce_bounds() {
        let easing = AnimationEasing::Bounce;
        for i in 0..=100 {
            let t = i as f64 / 100.0;
            let v = easing.evaluate(t);
            assert!(v >= 0.0, "bounce at t={} is {}", t, v);
            assert!(v <= 1.5, "bounce at t={} is {}", t, v);
        }
        assert!((easing.evaluate(0.0) - 0.0).abs() < 1e-6);
        assert!((easing.evaluate(1.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_multiple_animations() {
        let mut engine = AnimationEngineV2::new();
        let id1 = engine.create_animation(
            AnimationTarget::Window(WindowId::new()),
            AnimationProperty::Opacity,
            0.0,
            1.0,
            100,
            AnimationEasing::Linear,
        );
        let id2 = engine.create_animation(
            AnimationTarget::Window(WindowId::new()),
            AnimationProperty::Scale(1.0, 2.0),
            0.0,
            1.0,
            200,
            AnimationEasing::EaseIn,
        );
        engine.start_animation(id1);
        engine.start_animation(id2);
        assert_eq!(engine.running_count(), 2);
        engine.cancel_all();
        assert_eq!(engine.running_count(), 0);
    }
}
