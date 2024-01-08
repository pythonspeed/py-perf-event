# Python interface to CPU counters on Linux

Or, a minimal wrapper around the `py-perf-event2` Rust crate.

Example:

```python
from py_perf_event import measure, Hardware

[instructions1] = measure([Hardware.INSTRUCTIONS], sum, range(1_000_000))
print(instructions1)
[instructions2] = measure([Hardware.INSTRUCTIONS], sum, range(10_000_000))
print(instructions2)
assert instructions1 > 1_000_000
assert 7 < (instructions2 / instructions1) < 15
```

See [the tests](https://github.com/pythonspeed/py-perf-event/blob/main/test_perf_event.py) for more examples.
Until more docs are available, you can see the underlying Rust libraries docs for the [`Hardware`](https://docs.rs/perf-event2/latest/perf_event/events/struct.Hardware.html) and [`CacheId`](https://docs.rs/perf-event2/latest/perf_event/events/struct.CacheId.html)/[`CacheOp`](https://docs.rs/perf-event2/latest/perf_event/events/struct.CacheOp.html)/[`CacheResult`](https://docs.rs/perf-event2/latest/perf_event/events/struct.CacheResult.html) structs to see what options are exposed.
