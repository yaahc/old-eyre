use crate::{BoxError, ErrReport, EyreContext, Result};
use eyre_impl::{AddContext, ErrorContext};
use std::fmt;

impl EyreContext {
    pub fn note(&mut self, context: impl ContextObj) {
        self.push(EyreInfo::Note(Box::new(context)));
    }

    pub fn warn(&mut self, context: impl ContextObj) {
        self.push(EyreInfo::Warning(Box::new(context)));
    }

    pub fn suggestion(&mut self, context: impl ContextObj) {
        self.push(EyreInfo::Suggestion(Box::new(context)));
    }
}

impl ErrorContext for EyreContext {
    type ContextMember = EyreInfo;

    fn push(&mut self, context: Self::ContextMember) {
        self.context.push(context);
    }
}

impl Default for EyreContext {
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

pub trait Context<T>: private::Sealed {
    /// Wrap the error value with additional context in the form of a Note.
    fn note<C>(self, context: C) -> Result<T>
    where
        C: ContextObj;

    /// Wrap the error value with additional context in the form of a Warning.
    fn warning<C>(self, context: C) -> Result<T>
    where
        C: ContextObj;

    /// Wrap the error value with additional context in the form of a Suggestion.
    fn suggestion<C>(self, context: C) -> Result<T>
    where
        C: ContextObj;

    /// Wrap the error value with additional context in the form of a Note that is evaluated lazily
    /// only once an error does occur.
    fn with_note<C, F>(self, f: F) -> Result<T>
    where
        C: ContextObj,
        F: FnOnce() -> C;

    /// Wrap the error value with additional context in the form of a Warning that is evaluated
    /// lazily only once an error does occur.
    fn with_warning<C, F>(self, f: F) -> Result<T>
    where
        C: ContextObj,
        F: FnOnce() -> C;

    /// Wrap the error value with additional context in the form of a Suggestion that is evaluated
    /// lazily only once an error does occur.
    fn with_suggestion<C, F>(self, f: F) -> Result<T>
    where
        C: ContextObj,
        F: FnOnce() -> C;
}

impl<T, E> Context<T> for std::result::Result<T, E>
where
    E: eyre_impl::AddContext<
            EyreContext,
            WithContext = eyre_impl::ErrorReporter<BoxError, EyreContext>,
        > + Send
        + Sync
        + 'static,
{
    fn note<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|error| ErrReport::new(error.add_context(EyreInfo::Note(Box::new(context)))))
    }

    fn with_note<C, F>(self, context: F) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|error| ErrReport::new(error.add_context(EyreInfo::Note(Box::new(context())))))
    }

    fn warning<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|error| {
            ErrReport::new(error.add_context(EyreInfo::Warning(Box::new(context))))
        })
    }

    fn with_warning<C, F>(self, context: F) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|error| {
            ErrReport::new(error.add_context(EyreInfo::Warning(Box::new(context()))))
        })
    }

    fn suggestion<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|error| {
            ErrReport::new(error.add_context(EyreInfo::Suggestion(Box::new(context))))
        })
    }

    fn with_suggestion<C, F>(self, context: F) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|error| {
            ErrReport::new(error.add_context(EyreInfo::Suggestion(Box::new(context()))))
        })
    }
}

impl<T> Context<T> for Result<T> {
    fn note<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|error| error.add_context(EyreInfo::Note(Box::new(context))))
    }

    fn with_note<C, F>(self, context: F) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|error| error.add_context(EyreInfo::Note(Box::new(context()))))
    }

    fn warning<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|error| error.add_context(EyreInfo::Warning(Box::new(context))))
    }

    fn with_warning<C, F>(self, context: F) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|error| error.add_context(EyreInfo::Warning(Box::new(context()))))
    }

    fn suggestion<C>(self, context: C) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
    {
        self.map_err(|error| error.add_context(EyreInfo::Suggestion(Box::new(context))))
    }

    fn with_suggestion<C, F>(self, context: F) -> Result<T>
    where
        C: fmt::Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|error| error.add_context(EyreInfo::Suggestion(Box::new(context()))))
    }
}

impl AddContext<EyreContext> for crate::ErrReport {
    type WithContext = Self;

    fn add_context(mut self, context: EyreInfo) -> Self {
        self.0.context.push(context);
        self
    }
}

pub enum EyreInfo {
    Note(Box<dyn ContextObj>),
    Warning(Box<dyn ContextObj>),
    Suggestion(Box<dyn ContextObj>),
}

impl fmt::Display for EyreInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Note(context) => write!(f, "Note: {}", context),
            Self::Warning(context) => write!(f, "Warning: {}", context),
            Self::Suggestion(context) => write!(f, "Suggestion: {}", context),
        }
    }
}

pub(crate) mod private {
    use super::*;

    pub trait Sealed {}

    impl<T, E> Sealed for std::result::Result<T, E> where E: eyre_impl::AddContext<EyreContext> {}
}
