use crate::{BoxError, ErrReport, Result, RootCauseFirst};
use std::fmt;
use workingtitle::{ErrorContext, IntoErrorReporter};

impl RootCauseFirst {
    pub fn note(&mut self, context: impl ContextObj) {
        self.push(ContextObject::Note(Box::new(context)));
    }

    pub fn warn(&mut self, context: impl ContextObj) {
        self.push(ContextObject::Warn(Box::new(context)));
    }
}

impl ErrorContext<ContextObject> for RootCauseFirst {
    fn push(&mut self, context: ContextObject) {
        self.context.push(context);
    }
}

impl Default for RootCauseFirst {
    fn default() -> Self {
        Self {
            context: Vec::new(),
            backtrace: std::backtrace::Backtrace::capture(),
            span_backtrace: tracing_error::SpanTrace::capture(),
        }
    }
}

pub trait ContextObj: fmt::Display + Send + Sync + 'static {}

impl<T> ContextObj for T where T: fmt::Display + Send + Sync + 'static {}

pub trait ContextExt<T, E>: private::Sealed {
    /// Wrap the error value with additional context.
    fn note<C>(self, context: C) -> Result<T>
    where
        C: ContextObj;

    /// Wrap the error value with additional context that is evaluated lazily
    /// only once an error does occur.
    fn with_note<C, F>(self, f: F) -> Result<T>
    where
        C: ContextObj,
        F: FnOnce() -> C;
}

impl<T, E> ContextExt<T, E> for std::result::Result<T, E>
where
    E: workingtitle::IntoErrorReporter<BoxError, RootCauseFirst, ContextObject>
        + Send
        + Sync
        + 'static,
{
    fn note<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|error| {
            ErrReport::new(error.ext_context(ContextObject::Note(Box::new(context))))
        })
    }

    fn with_note<C, F>(self, context: F) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|error| {
            ErrReport::new(error.ext_context(ContextObject::Note(Box::new(context()))))
        })
    }
}

impl IntoErrorReporter<BoxError, RootCauseFirst, ContextObject> for crate::ErrReport {
    fn ext_context(
        mut self,
        context: ContextObject,
    ) -> workingtitle::ErrorReporter<BoxError, RootCauseFirst> {
        self.0.context.push(context);
        *self.0
    }
}

pub enum ContextObject {
    Note(Box<dyn ContextObj>),
    Warn(Box<dyn ContextObj>),
}

// impl ErrorFormatter for crate::context::RootCauseFirst {
//     fn fmt_error<'a>(
//         &self,
//         error: &'a (dyn std::error::Error + 'static),
//         f: &mut fmt::Formatter,
//     ) -> fmt::Result {
//         let errors = Chain::new(error).rev().enumerate();

//         writeln!(f)?;

//         for (n, error) in errors {
//             write!(Indented::numbered(f, n), "{}", error)?;
//             writeln!(f)?;
//         }

//         write!(f, "\n\n{}", self.span_backtrace)?;

//         let backtrace = &self.backtrace;
//         if let BacktraceStatus::Captured = backtrace.status() {
//             let mut backtrace = backtrace.to_string();
//             if backtrace.starts_with("stack backtrace:") {
//                 // Capitalize to match "Caused by:"
//                 backtrace.replace_range(0..7, "Stack B");
//             }
//             backtrace.truncate(backtrace.trim_end().len());
//             write!(f, "\n\n{}", backtrace)?;
//         }

//         Ok(())
//     }
// }

pub(crate) mod private {
    use super::*;

    pub trait Sealed {}

    impl<T, E> Sealed for std::result::Result<T, E> where
        E: workingtitle::IntoErrorReporter<BoxError, RootCauseFirst, ContextObject>
    {
    }
    impl<T> Sealed for Option<T> {}
}
