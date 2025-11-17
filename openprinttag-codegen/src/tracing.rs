#![allow(unused_macros, unused_imports)]

#[cfg(feature = "tracing")]
pub(crate) use tracing::{debug, error, info, trace, warn};

#[cfg(not(feature = "tracing"))]
macro_rules! log_debug {
    ($($t:tt)*) => {};
}
#[cfg(not(feature = "tracing"))]
macro_rules! log_trace {
    ($($t:tt)*) => {};
}
#[cfg(not(feature = "tracing"))]
macro_rules! log_info {
    ($($t:tt)*) => {};
}
#[cfg(not(feature = "tracing"))]
macro_rules! log_warn {
    ($($t:tt)*) => {};
}
#[cfg(not(feature = "tracing"))]
macro_rules! log_error {
    ($($t:tt)*) => {};
}

#[cfg(not(feature = "tracing"))]
pub(crate) use {
    log_debug as debug, log_error as error, log_info as info, log_trace as trace, log_warn as warn,
};
