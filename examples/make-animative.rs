use anim::{
    timeline::{Builder, Options, Status},
    Animatable, Animation, Timeline,
};
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
struct MyTimelineModel {
    a: f32, //animatable
    b: i64, //animatable
}

// make it animatable, do not forget to derive Clone
impl Animatable for MyTimelineModel {
    fn animate(&self, to: &Self, time: f64) -> Self {
        let a = self.a.animate(&to.a, time);
        let b = self.b.animate(&to.b, time);
        MyTimelineModel { a, b }
    }
}

// once it's animatable, you can use it with anim::timeline::Options;

fn main() {
    let from = MyTimelineModel { a: 0.0, b: 32 };
    let to = MyTimelineModel { a: 100.0, b: 100 };
    let mut timeline: Timeline<_> = Options::new(from, to)
        .duration(Duration::from_secs(2))
        .times(1)
        .into();

    println!("start animation");
    timeline.begin();

    loop {
        let status = timeline.update();
        if status == Status::Completed {
            break;
        }
        let value = timeline.value();
        println!("animated: {:?}", value);
    }
}
