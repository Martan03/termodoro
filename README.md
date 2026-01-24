# termodoro

Pomodoro TUI implementation in Rust 🦀

## Table of Contents

{{ mdcon }}

## Installation

Currently there's no other way of installing then building it yourself. You
need to have Rust Toolchain installed (see
[rust installation page](https://www.rust-lang.org/tools/install)). When you
have the Rust Toochain, you can build the project with `cargo`:

```bash
cargo build -r
```

After it's done compiling, the binary will be `target/release/termodoro`.

## Usage

To start the TUI app with an interval picker you can run:

```bash
./termodoro
```

If you want to skip the need for picking the interval or you want a custom one,
you can run:

```bash
./termodoro -w <focus_time> -r <rest_time> -l <long_rest_time> -i <long_rest_rate>
```

## The science of focus

The Pomodoro Technique is often debated - some people love it, while others
don't believe in it and fear that it breaks their "flow". But the method isn't
just a random timer, it is designed with our biology in mind.

### The brain's natural pulse

Our brains have many natural cycles. The Circadian rhythm is the best known,
functioning over a roughly 24-hour period and most notably governing our
sleep-wake cycle. Our brains also operate on shorter pulses called Ultradian
rhythms, which have periods shorter than a day, but longer than an hour. There
are many Ultradian rhythms, such as for blood circulation and blinking, but for
our interest the Basic Rest-Activity Cycle (BRAC) is the most notable.

### Basic Rest-Activity Cycle

The BRAC is an Ultradian rhythm of approximately 90-120 minutes. During the
first half of the cycle, our brainwaves are faster, making us more alert and
focused. In the final 20 minutes of the cycle, our brainwaves slow down and
our body starts to feel more tired. This is a result of it preparing for the
start of the next cycle.

### Why pomodoro fits?

If you work without a break through the full 90-minute cycle, you hit a
so-called cognitive cliff. Our brains begin to fatigue, which destroys our
focus. The most common Pomodoro split is 25/5 (25 minutes of focus followed by
a short, 5 minute break). After four sessions (totaling 100 minutes, 115
minutes including breaks), you have completed one full Ultradian cycle. To
allow our brain to reset, Pomodoro includes a long break (usually 15 minutes),
which aligns with the brain's natural recovery phase.

### Psychological aspect

I am no psychologist, but one thing the Pomodoro technique does well is
Time-boxing.

A time-box is a boundary that tells your brain exactly how long it needs to
focus. This is significantly more effective than looking at the massive goal
(such as "study all weekend"). The clear end point let's you begin so much
easier.

This principle is core to building habits as well. If you want to start a
reading habit, for example, the best way is to set an easily achievable
boundary, like "read one page a day". The worst-case scenario is that you read
only one page, but often you want to finish the interesting part so you keep
reading. Initially it might be difficult to do, but once the friction is gone,
you often overachieve your goal and read multiple pages.

### Conclusion

A lot of people misinterpret the meaning of Pomodoro. It isn't meant to
interrupt your focus, it's meant to ensure your brain actually gets enough time
to recover to stay productive for longer.

Personally, I don't use Pomodoro for everything. I find it most effective when
I'm struggling to start a task or maintain focus. For example, when I'm
programming, I often enter a flow state where a timer might actually be a
distraction. On the other hand, when I have a lot of studying to do, Pomodoro
comes in handy. It helps me overcome the initial friction of starting and makes
sure my brain stays fresh for longer, which translates to me learning for
longer period.

At the end of the day, this technique is really subjective and requires
you setting it up so that it fits your needs, so definitely spend some time
tuning the intervals until you find the rhythm that fits your workflow!

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [termodoro](https://github.com/Martan03/termodoro)
- **Author website:** [martan03.github.io](https://martan03.github.io)
