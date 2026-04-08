"""
Access to a subset of Linux's perf_event_open() API covering CPU hardware counters.
"""

from . import _lib

_re_export = [
    "Cache",
    "CacheId",
    "CacheOp",
    "CacheResult",
    "Measure",
    "Hardware",
    "Raw",
    "Read",
]
for name in _re_export:
    globals()[name] = getattr(_lib, name)


class PartialRead(Exception):
    """
    CPU counters didn't run for the whole time, so counter measurements may be inaccurately low.
    """

    def __init__(self, read: _lib.Read):
        super().__init__(
            f"Enabled {read.time_enabled_ns} ns, but only measured for {read.time_running_ns} ns. Measurements were {read.measurements}"
        )
        self.read = read


def measure(events, a_callable, *args, **kwargs) -> list[int]:
    """
    Call a function or other callable with given arguments, measuring the given
    events, and returning ``list`` with their respective results.

    Will raise ``PartialRead`` if CPU counters will only enabled part of the
    time.
    """
    read = _lib.measure(events, a_callable, *args, **kwargs)
    if read.time_running_ns < read.time_enabled_ns:
        raise PartialRead(read)
    return read.measurements


__all__ = _re_export
