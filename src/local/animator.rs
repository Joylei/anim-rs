use super::timeline::{Timeline, TimelineEx};
use crate::{
    core::timeline::Timeline as CoreTimeline,
    timeline::{Builder, Options, Status, TimelineId},
    Animatable,
};
use parking_lot::{Mutex, RwLock, RwLockUpgradableReadGuard};
use std::{collections::HashMap, rc::Rc, time::Instant};

thread_local! {
    /// thread local manager
    ///
    /// one iced ui thread will have one manager
    static MANAGER: Manager = Manager::new();
}

/// build a thread-local based [`Timeline`], which attaches to current thread once created
pub fn timeline<T>(opts: Options<T>) -> Timeline<T>
where
    T: Animatable + 'static,
{
    let timeline = opts.build();
    let shared = MANAGER.with(|m| m.shared.clone());
    let wrapper = TimelineWrapper::new(timeline, shared);
    wrapper.into()
}

/// update current thread associated [`Timeline`]s
pub fn update() {
    MANAGER.with(|m| m.update());
}

pub(crate) struct TimelineWrapper<T: Animatable> {
    id: TimelineId,
    pub(crate) inner: Rc<Mutex<Inner<T>>>,
    shared: Shared,
}

pub(crate) struct Inner<T: Animatable> {
    pub(crate) timeline: CoreTimeline<T>,
    pub(crate) cache: Option<(Instant, Status, T)>,
    scheduled: bool,
}

impl<T: Animatable> Inner<T> {
    pub(crate) fn update(&mut self, time: Instant) -> Status {
        let status = self.timeline.update(time);
        let value = self.timeline.value();
        self.cache = Some((time, status, value));
        status
    }
}

impl<T: Animatable> TimelineWrapper<T> {
    fn new(timeline: CoreTimeline<T>, shared: Shared) -> Self {
        Self {
            id: timeline.id(),
            inner: Rc::new(Mutex::new(Inner {
                timeline,
                cache: None,
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

impl<T: Animatable + 'static> TimelineEx<T> for TimelineWrapper<T> {
    #[inline]
    fn status(&self) -> Status {
        let state = &*self.inner.lock();
        if let Some((_, status, _)) = state.cache.as_ref() {
            *status
        } else {
            state.timeline.status()
        }
    }

    #[inline]
    fn value(&self) -> T {
        let state = &*self.inner.lock();
        if let Some((_, _, value)) = state.cache.as_ref() {
            value.clone()
        } else {
            state.timeline.value()
        }
    }

    #[inline]
    fn begin(&self) {
        {
            let state = &mut *self.inner.lock();
            state.timeline.begin();
            state.cache = None; //reset cache
            state.scheduled = true;
        }
        self.shared.schedule(Rc::clone(&self.inner));
    }

    #[inline]
    fn stop(&self) {
        {
            let time = Instant::now();
            let state = &mut *self.inner.lock();
            state.timeline.stop();
            state.update(time);
            state.cache = None; //reset cache
        }

        let id = self.id;
        self.shared.cancel(id);
    }

    #[inline]
    fn pause(&self) {
        {
            let time = Instant::now();
            let state = &mut *self.inner.lock();
            state.timeline.pause();
            state.update(time);
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

impl<T: Animatable> Drop for TimelineWrapper<T> {
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

impl<T: Animatable + 'static> From<TimelineWrapper<T>> for Timeline<T> {
    #[inline]
    fn from(src: TimelineWrapper<T>) -> Self {
        Timeline::new(src)
    }
}

trait TimelineControl {
    /// timeline unique id
    fn id(&self) -> TimelineId;
    /// update timeline
    fn update(&self, time: Instant) -> Status;

    /// on schedule into [`TimelineScheduler`]
    fn on_schedule(&self);

    /// on removed from [`TimelineScheduler`]
    fn on_remove(&self);
}

impl<T: Animatable> TimelineControl for Rc<Mutex<Inner<T>>> {
    #[inline]
    fn id(&self) -> TimelineId {
        let state = &*self.lock();
        state.timeline.id()
    }

    #[inline]
    fn update(&self, time: Instant) -> Status {
        let state = &mut *self.lock();
        state.update(time)
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
        let now = Instant::now();
        let state = self.0.upgradable_read();
        for (id, item) in state.iter() {
            let status = item.update(now);
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
