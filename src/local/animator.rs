use super::timeline::{Timeline, TimelineEx};
use crate::{
    core::timeline::Timeline as CoreTimeline,
    timeline::{Boxed, Status, TimelineId},
};
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use std::{collections::HashMap, rc::Rc};

thread_local! {
    /// thread local manager
    ///
    /// one iced ui thread will have one manager
    static MANAGER: Manager = Manager::new();
}

/// build a thread-local based [`Timeline`], which attaches to current thread once created
pub fn timeline<F, T>(animation: F) -> Timeline<T>
where
    F: Into<Boxed<T>> + 'static,
    T: 'static,
{
    let timeline: CoreTimeline<_> = CoreTimeline::new(animation);
    let shared = MANAGER.with(|m| m.shared.clone());
    let wrapper = TimelineWrapper::new(timeline, shared);
    wrapper.into()
}

/// update current thread associated [`Timeline`]s
///
/// you should call it only in one place
pub fn update() {
    MANAGER.with(|m| m.update());
}

pub(crate) struct TimelineWrapper<T> {
    id: TimelineId,
    pub(crate) inner: Rc<Mutex<Inner<T>>>,
    shared: Shared,
}

pub(crate) struct Inner<T> {
    pub(crate) timeline: CoreTimeline<T>,
    scheduled: bool,
}

impl<T> TimelineWrapper<T> {
    fn new(timeline: CoreTimeline<T>, shared: Shared) -> Self {
        Self {
            id: timeline.id(),
            inner: Rc::new(Mutex::new(Inner {
                timeline,
                scheduled: false,
            })),
            shared,
        }
    }

    fn scheduled(&self) -> bool {
        let state = self.inner.lock();
        state.scheduled
    }
}

impl<T: 'static> TimelineEx<T> for TimelineWrapper<T> {
    #[inline]
    fn status(&self) -> Status {
        let state = &*self.inner.lock();
        state.timeline.status()
    }

    #[inline]
    fn value(&self) -> T {
        let state = &*self.inner.lock();
        state.timeline.value()
    }

    #[inline]
    fn begin(&self) {
        {
            let state = &mut *self.inner.lock();
            state.timeline.begin();
        }
        self.shared.schedule(Rc::clone(&self.inner));
    }

    #[inline]
    fn stop(&self) {
        {
            let state = &mut *self.inner.lock();
            state.timeline.stop();
        }

        let id = self.id;
        self.shared.cancel(id);
    }

    #[inline]
    fn pause(&self) {
        {
            let state = &mut *self.inner.lock();
            state.timeline.pause();
        }
        let id = self.id;
        self.shared.cancel(id);
    }

    #[inline]
    fn resume(&self) {
        {
            let state = &mut *self.inner.lock();
            state.timeline.resume();
            state.scheduled = true;
        }
        self.shared.schedule(Rc::clone(&self.inner));
    }
}

impl<T> Drop for TimelineWrapper<T> {
    fn drop(&mut self) {
        let id = self.id;
        let scheduled = self.scheduled();
        dbg!(Rc::strong_count(&self.inner));
        if scheduled && Rc::strong_count(&self.inner) == 2 {
            //eprintln!("drop TimelineWrapper");
            self.shared.cancel(id);
        }
    }
}

impl<T: 'static> From<TimelineWrapper<T>> for Timeline<T> {
    #[inline]
    fn from(src: TimelineWrapper<T>) -> Self {
        Timeline::new(src)
    }
}

trait TimelineControl {
    /// timeline unique id
    fn id(&self) -> TimelineId;
    /// update timeline
    fn update(&self) -> Status;

    /// on schedule into [`TimelineScheduler`]
    fn on_schedule(&self);

    /// on removed from [`TimelineScheduler`]
    fn on_remove(&self);
}

impl<T> TimelineControl for Rc<Mutex<Inner<T>>> {
    #[inline]
    fn id(&self) -> TimelineId {
        let state = &*self.lock();
        state.timeline.id()
    }

    #[inline]
    fn update(&self) -> Status {
        let state = &mut *self.lock();
        state.timeline.update()
    }

    #[inline]
    fn on_schedule(&self) {
        let state = &mut *self.lock();
        state.scheduled = true;
    }

    #[inline]
    fn on_remove(&self) {
        let state = &mut *self.lock();
        state.scheduled = false;
    }
}

#[derive(Clone)]
struct Shared(Rc<RwLock<HashMap<TimelineId, Box<dyn TimelineControl + 'static>>>>);

impl Shared {
    fn update(&self) {
        let mut holder = Vec::new();
        let state = self.0.upgradable_read();
        for (id, item) in state.iter() {
            let status = item.update();
            if status == Status::Completed || status == Status::Paused {
                holder.push(*id);
            }
        }
        if !holder.is_empty() {
            let mut state = RwLockUpgradableReadGuard::upgrade(state);
            for id in holder {
                state.remove(&id);
            }
        }
    }

    fn schedule(&self, timeline: impl TimelineControl + 'static) {
        let id = timeline.id();
        let state = self.0.upgradable_read();
        if !state.contains_key(&id) {
            timeline.on_schedule();
            let mut state = RwLockUpgradableReadGuard::upgrade(state);
            state.insert(id, Box::new(timeline));
        }
    }

    fn cancel(&self, id: TimelineId) -> bool {
        let res = {
            let state = self.0.upgradable_read();
            if state.contains_key(&id) {
                let mut state = RwLockUpgradableReadGuard::upgrade(state);
                state.remove(&id)
            } else {
                None
            }
        };
        if let Some(ref item) = res {
            item.on_remove();
        }
        res.is_some()
    }
}

struct Manager {
    shared: Shared,
}

impl Manager {
    fn new() -> Self {
        Self {
            shared: Shared(Rc::new(RwLock::new(Default::default()))),
        }
    }

    #[inline]
    fn update(&self) {
        self.shared.update();
    }
}
