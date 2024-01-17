//! Get CPU performance counters on Linux.
use perf_event::events;
use pyo3::{prelude::*, types::{PyTuple, PyDict}};

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

#[derive(Clone, Copy)]
#[pyclass]
pub struct CacheResult(events::CacheResult);

expose_consts!(CacheResult, ACCESS, MISS);

#[derive(Clone, Copy)]
#[pyclass]
pub struct CacheId(events::CacheId);

expose_consts!(CacheId, L1D, L1I, LL, DTLB, ITLB, BPU, NODE);

#[derive(Clone, Copy)]
#[pyclass]
pub struct CacheOp(events::CacheOp);

expose_consts!(CacheOp, READ, WRITE, PREFETCH);

/// Read a counter covering caches.
#[derive(Clone, Copy)]
#[pyclass]
pub struct Cache {
    which: CacheId,
    operation: CacheOp,
    result: CacheResult,
}

#[pymethods]
impl Cache {
    #[new]
    fn new(which: CacheId, operation: CacheOp, result: CacheResult) -> Self {
        Self {
            which, operation, result
        }
    }
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

/// Read a counter from the CPU.
#[derive(Clone, Copy)]
#[pyclass]
pub struct Hardware(events::Hardware);

expose_consts!(Hardware, CPU_CYCLES, INSTRUCTIONS, CACHE_REFERENCES, CACHE_MISSES, BRANCH_INSTRUCTIONS, BRANCH_MISSES, BUS_CYCLES, STALLED_CYCLES_FRONTEND, STALLED_CYCLES_BACKEND, REF_CPU_CYCLES);

/// A raw, model-specific CPU counter.
#[derive(Clone, Copy)]
#[pyclass]
pub struct Raw(events::Raw);

#[pymethods]
impl Raw {
    #[new]
    fn new(config: u64) -> Self {
        Raw(events::Raw::new(config))
    }
}

/// Start gathering counter information, given a list of Hardware or Cache
/// instances.
#[pyclass]
struct Measure {
    counters: Vec<perf_event::Counter>,
    group: perf_event::Group,
}

#[pymethods]
impl Measure {
    #[new]
    fn new(events: Vec<&PyAny>) -> PyResult<Self> {
        let mut counters = vec![];
        let mut group = perf_event::Group::new()?;
        for event in events {
            if let Ok(hw) = event.extract::<Hardware>() {
                counters.push(group.add(&perf_event::Builder::new(hw.0))?);
                continue;
            }
            if let Ok(raw) = event.extract::<Raw>() {
                counters.push(group.add(&perf_event::Builder::new(raw.0))?);
                continue;
            }
            let cache: Cache = event.extract()?;
            let cache: events::Cache = cache.into();
            counters.push(group.add(&perf_event::Builder::new(cache))?);
        }
        Ok(Self {
            counters, group
        })
    }

    fn enable(&mut self) -> PyResult<()> {
        self.group.enable()?;
        Ok(())
    }

    fn disable(&mut self) -> PyResult<()> {
        self.group.disable()?;
        Ok(())
    }

    fn read(&mut self) -> PyResult<Vec<u64>> {
        let data = self.group.read()?;
        let mut result = vec![];
        for counter in &self.counters {
            result.push(data[&counter]);
        }
        Ok(result)
    }
}

/// Measure the given events for ``callable(*args, **kwargs)``.
#[pyfunction]
#[pyo3(signature = (events, callable, *args, **kwargs))]
fn measure(events: Vec<&PyAny>, callable: &PyAny, args: &PyTuple, kwargs: Option<&PyDict>) -> PyResult<Vec<u64>> {
    let mut measure = Measure::new(events)?;
    measure.enable()?;
    callable.call(args, kwargs)?;
    let result = measure.read()?;
    measure.disable()?;
    Ok(result)
}

/// Get CPU performance counters on Linux.
#[pymodule]
fn py_perf_event(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CacheId>()?;
    m.add_class::<CacheOp>()?;
    m.add_class::<CacheResult>()?;
    m.add_class::<Cache>()?;
    m.add_class::<Hardware>()?;
    m.add_class::<Raw>()?;
    m.add_class::<Measure>()?;
    m.add_function(wrap_pyfunction!(measure, m)?)?;
    Ok(())
}
