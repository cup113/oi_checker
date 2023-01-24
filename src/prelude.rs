//! oi_checker inner preludes

pub use std::collections::HashMap;
pub use std::fmt::Display;
pub use std::fs::{self, File};
pub use std::io;
pub use std::path::PathBuf;
pub use std::process::{Command, Stdio};
pub use std::sync::mpsc;
pub use std::thread;
pub use std::time::{Duration, Instant};

pub use crate::checker_error::{CheckerError, CheckerResult, Stage};
pub use crate::util::{IgnoreResult, LINE_END};
