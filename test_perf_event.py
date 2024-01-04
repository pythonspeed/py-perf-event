from py_perf_event import (
    Measure,
    Hardware,
    Cache,
    CacheId,
    CacheOp,
    CacheResult,
    measure,
)


def test_measure():
    """
    The measure() API allows getting counters for a given callable, which gets
    called with the given arguments.
    """
    [instructions1] = measure([Hardware.INSTRUCTIONS], sum, range(1_000_000))
    [instructions2] = measure([Hardware.INSTRUCTIONS], sum, range(10_000_000))
    assert instructions1 > 1_000_000
    assert 7 < (instructions2 / instructions1) < 15
