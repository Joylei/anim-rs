// anim
//
// An animation library, works nicely with Iced and the others
// Copyright: 2021, Joylei <leingliu@gmail.com>
// License: MIT

/*!
# anim

This is an animation library, works nicely with [Iced](https://github.com/hecrj/iced) and the others.

## Showcase

<center>

![Color&Opacity Animation Example](./images/color-example.gif)

![Size Animation Example](./images/size-example.gif)

![Raindrop Splash Animation](./images/animated-splash.gif)

</center>

## How to install?

Include `anim` in your `Cargo.toml` dependencies:

```toml
[dependencies]
anim = "0.1"
```

Note: `anim` turns on `iced-backend` feature by default. You need to disable default features if you do not work with `iced`.

```toml
[dependencies]
anim = { version="0.1", default-features = false }
```

## How to use?

There are 3 important concepts in `anim`:
- `Animatable`
Types derived from `Animatable` means that its values can be calculated based on timing progress, with which you can create `Animation` objects.

- `Animation`
The `Animation` generates values based on its timing progress. You can construct a big `Animation`  from small ones.

- `Timeline`
With `Timeline` you can control your animations' lifetime.

---

For simple scenarios, you just need `Options`.

```rust
use anim::{Options, Timeline, Animation, easing};
```

Then, build and start your animation:

```rust
use std::time::Duration;
use anim::{Options, Timeline, Animation, easing};

let mut timeline = Options::new(20,100).easing(easing::bounce_ease())
    .duration(Duration::from_millis(300))
    .begin_animation();

loop {
    let status = timeline.update();
    if status.is_completed() {
       break;
    }
    println!("animated value: {}", timeline.value());
}
```

For complex scenarios, please look at [examples](./examples/) to gain some ideas.


*/

#![warn(missing_docs)]

mod core;
/// iced animation backend
#[cfg(feature = "iced-backend")]
mod iced;
/// thread local based timeline
#[cfg(feature = "local")]
pub mod local;

// reexports
pub use crate::core::*;
#[cfg(feature = "iced-backend")]
pub use crate::iced::*;
