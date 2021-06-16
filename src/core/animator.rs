use crate::timeline::{Status, TimelineId};
use std::time::Instant;

/// represents timeline
pub trait TimelineControl {
    /// timeline unique id
    fn id(&self) -> TimelineId;
    /// update timeline
    fn update(&mut self, time: Instant) -> Status;

    /// on schedule into [`TimelineScheduler`]
    fn on_schedule(&mut self);

    /// on removed from [`TimelineScheduler`]
    fn on_remove(&mut self);
}

/// timeline scheduler
pub trait TimelineScheduler {
    /// represents timeline
    type Timeline: TimelineControl;
    /// enqueue timeline
    fn schedule(&mut self, timeline: Self::Timeline);
    /// remove timeline
    fn cancel(&mut self, id: TimelineId) -> bool;
}
