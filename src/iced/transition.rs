use crate::timeline::Status;
use std::time::Duration;

/// default transition duration
pub const DEFAULT_TRANSITION_DURATION: Duration = Duration::from_millis(400);

/// fade transition
pub mod fade;
/// fly transition
pub mod fly;
/// slide transition
pub mod slide;

/// transition controller
pub trait Transition {
    /// update the transition, so the animation can be evaluated to the next state
    fn update(&mut self);
    /// start transition
    fn begin(&mut self);
    /// stop transition
    fn stop(&mut self);
    /// animation status()
    fn status(&self) -> Status;
    /// indicate the visibility of your target element
    fn visible(&self) -> bool;
}
