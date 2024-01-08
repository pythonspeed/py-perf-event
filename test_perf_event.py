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
