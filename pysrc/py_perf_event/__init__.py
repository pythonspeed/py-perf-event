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


def measure(events, a_callable, *args, **kwargs) -> list[int]:
    """
    Call a function or other callable with given arguments, measuring the given
    events, and returning ``list`` with their respective results.

    Will raise ``PartialRead`` if CPU counters will only enabled part of the
    time.
    """
    return _lib.measure(events, a_callable, *args, **kwargs).measurements


__all__ = _re_export
