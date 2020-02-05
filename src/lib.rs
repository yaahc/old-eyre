#![feature(backtrace)]
pub use context::ContextExt;
use context::ContextObject;
pub use err as format_err;
use std::{backtrace::Backtrace, fmt};

mod context;
mod macros;
mod report;

#[derive(Debug)]
pub struct BoxError(Box<dyn std::error::Error + Send + Sync + 'static>);

pub struct ErrReport(pub(crate) Box<eyre_impl::ErrorReporter<BoxError, RootCauseFirst>>);

pub struct RootCauseFirst {
    pub(crate) context: Vec<ContextObject>,
    backtrace: Backtrace,
    // pub(crate) span_backtrace: tracing_error::SpanTrace,
}

impl std::error::Error for BoxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

impl fmt::Display for BoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl ErrReport {
    fn new(reporter: eyre_impl::ErrorReporter<BoxError, RootCauseFirst>) -> Self {
        ErrReport(Box::new(reporter))
    }
}
pub type Result<T> = std::result::Result<T, ErrReport>;

#[doc(hidden)]
pub mod private {
    pub use adhocerr::{err, wrap};
    pub use core::result::Result::Err;
}
