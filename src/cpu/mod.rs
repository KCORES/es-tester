pub mod detection;
pub mod info;

pub use detection::detect_cpu;
pub use info::{CpuInfo, CpuVendor, CpuGeneration};
