import numpy as np
from numba import njit
from py_perf_event import (
    Measure,
    Hardware,
    Cache,
    CacheId,
    CacheOp,
    CacheResult,
    Raw,
    measure,
)


def test_measure_class():
    """
    The ``Measure`` API allows getting counters for any code.
    """
    m = Measure([Hardware.INSTRUCTIONS])
    m.enable()
    sum(range(1_000_000))
    [instructions] = m.read()
    sum(range(1_000_000))
    [instructions2] = m.read()
    m.disable()
    assert 1.5 < instructions2 / instructions < 2.5


def test_measure_function():
    """
    The measure() API allows getting counters for a given callable, which gets
    called with the given arguments.
    """
    [instructions1] = measure([Hardware.INSTRUCTIONS], sum, range(1_000_000))
    [instructions2] = measure([Hardware.INSTRUCTIONS], sum, range(10_000_000))
    assert instructions1 > 1_000_000
    assert 7 < (instructions2 / instructions1) < 15


def test_cache_ops():
    """
    ``Cache`` instances get measured too.
    """
    ll_reads = Cache(CacheId.LL, CacheOp.READ, CacheResult.ACCESS)
    ll_misses = Cache(CacheId.LL, CacheOp.READ, CacheResult.MISS)

    # Random scan, not linear:
    def traverse(l):
        result = 0
        length = len(l)
        for i in range(length):
            result += l[(i * 123763537) % length]
        return result

    small_list = list(range(1_000))
    [small_reads, small_misses] = measure([ll_reads, ll_misses], traverse, small_list)
    large_list = list(range(10_000_000))
    [large_reads, large_misses] = measure([ll_reads, ll_misses], traverse, large_list)
    assert small_reads < 1000
    assert small_misses <= small_reads
    assert large_reads > 1000 * small_reads
    assert (large_misses / large_reads) > 0.2


def test_raw():
    """
    ``Raw()`` events get measured.

    TODO: This test is model-specific, only tested on i7-12700K.
    """
    # SIMD on float64:
    simd_f64 = [Raw(0x4c7), Raw(0x10c7)]

    f64_data = np.ones((1_000_000,), dtype=np.float64)
    f32_data = np.ones((1_000_000,), dtype=np.float32)

    @njit
    def double(arr):
        result = np.empty(arr.shape, dtype=arr.dtype)
        # Should auto-vectorize to SIMD;
        for i in range(len(arr)):
            result[i] = 2 * arr[i]
        return result

    double(f64_data)
    double(f32_data)

    with_f64 = sum(measure(simd_f64, double, f64_data))
    assert with_f64 > (1_000_000 / 8) * 0.5
    with_f32 = sum(measure(simd_f64, double, f32_data))
    assert with_f32 < 100
