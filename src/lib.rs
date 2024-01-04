//! A Python wrapper for perf-event2.
use perf_event::events;
use pyo3::prelude::*;

#[derive(Clone)]
#[pyclass]
pub struct CacheResult(events::CacheResult);

macro_rules! expose_consts {
    ($klass:ident, $($attr:ident),+) => {
        #[pymethods]
        impl $klass {
            $(
                #[classattr]
                const $attr : Self = Self(events::$klass::$attr);
            )*
        }
    }
}

expose_consts!(CacheResult, ACCESS, MISS);

#[derive(Clone)]
#[pyclass]
pub struct CacheId(events::CacheId);

expose_consts!(CacheId, L1D, L1I, LL, DTLB, ITLB, BPU, NODE);

#[derive(Clone)]
#[pyclass]
pub struct CacheOp(events::CacheOp);

expose_consts!(CacheOp, READ, WRITE, PREFETCH);

#[derive(Clone)]
#[pyclass]
pub struct Cache {
    which: CacheId,
    operation: CacheOp,
    result: CacheResult,
}

impl From<Cache> for events::Cache {
    fn from(value: Cache) -> Self {
        Self {
            which: value.which.0,
            operation: value.operation.0,
            result: value.result.0,
        }
    }
}

#[derive(Clone)]
#[pyclass]
pub struct Hardware(events::Hardware);

expose_consts!(Hardware, CPU_CYCLES, INSTRUCTIONS, CACHE_REFERENCES, CACHE_MISSES, BRANCH_INSTRUCTIONS, BRANCH_MISSES, BUS_CYCLES, STALLED_CYCLES_FRONTEND, STALLED_CYCLES_BACKEND, REF_CPU_CYCLES);

#[pyfunction]
fn start_profiling(events: Vec<&PyAny>) -> PyResult<()> {
    let mut group = perf_event::Group::new()?;
    for event in events {
        if let Ok(hw) = event.extract::<Hardware>() {
            group.add(&perf_event::Builder::new(hw.0))?;
            continue;
        }
        let cache: Cache = event.extract()?;
        let cache: events::Cache = cache.into();
        group.add(&perf_event::Builder::new(cache))?;
    }
    group.enable()?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_perf_event(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CacheId>()?;
    m.add_class::<CacheOp>()?;
    m.add_class::<CacheResult>()?;
    m.add_class::<Cache>()?;
    m.add_class::<Hardware>()?;
    m.add_function(wrap_pyfunction!(start_profiling, m)?)?;
    Ok(())
}
