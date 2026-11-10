#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[cfg(target_os = "linux")]
pub mod catalog;

#[cfg(all(target_os = "linux", feature = "qt-probe"))]
pub mod qtbridge;
