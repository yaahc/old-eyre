use crate::{BoxError, ErrReport};
use std::{backtrace::BacktraceStatus, fmt, fmt::Write as _};
use workingtitle::Indented;

impl fmt::Debug for ErrReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = &self.0.error;

        if f.alternate() {
            return fmt::Debug::fmt(error, f);
        }

        let errors = self.0.chain().rev().enumerate();

        writeln!(f)?;

        for (n, error) in errors {
            write!(Indented::numbered(f, n), "{}", error)?;
            writeln!(f)?;
        }

        write!(f, "\n\n{}", self.0.context.span_backtrace)?;

        let backtrace = &self.0.context.backtrace;
        if let BacktraceStatus::Captured = backtrace.status() {
            write!(f, "\n\n{}", backtrace)?;
        }

        Ok(())
    }
}

impl fmt::Display for ErrReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.error)?;

        if f.alternate() {
            for cause in self.0.chain().skip(1) {
                write!(f, ": {}", cause)?;
            }
        }

        Ok(())
    }
}

impl<E> From<E> for ErrReport
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn from(err: E) -> Self {
        ErrReport(Box::new(workingtitle::ErrorReporter::from(BoxError(
            Box::new(err),
        ))))
    }
}
