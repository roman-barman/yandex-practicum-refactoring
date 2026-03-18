mod app_log_error_kind;
mod app_log_journal_kind;
mod app_log_kind;
mod app_log_trace_kind;
mod log_kind;
mod log_line;
mod system_log_error_kind;
mod system_log_kind;
mod system_log_trace_kind;

use crate::parsable::Parsable;
use crate::parse::Parser;
pub(crate) use app_log_error_kind::*;
pub(crate) use app_log_journal_kind::*;
pub(crate) use app_log_kind::*;
pub(crate) use app_log_trace_kind::*;
pub(crate) use log_kind::*;
pub(crate) use log_line::*;
use std::fmt::Debug;
pub(crate) use system_log_kind::*;

/// Log line parser
pub struct LogLineParser {
    parser: std::sync::OnceLock<<LogLine as Parsable>::Parser>,
}
impl LogLineParser {
    pub fn new() -> Self {
        Self {
            parser: std::sync::OnceLock::new(),
        }
    }

    pub fn parse<'a>(&self, input: &'a str) -> Result<(&'a str, LogLine), ()> {
        self.parser
            .get_or_init(|| <LogLine as Parsable>::parser())
            .parse(input)
    }
}

impl Debug for LogLineParser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogLineParser").finish()
    }
}
