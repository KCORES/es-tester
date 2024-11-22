use std::error::Error;
use super::info::{CpuInfo, CpuVendor, CpuGeneration};

pub fn detect_cpu() -> Result<CpuInfo, Box<dyn Error>> {
    Ok(CpuInfo {
        vendor: CpuVendor::Intel,
        generation: CpuGeneration::XeonV2,
        model_name: String::from("Intel Xeon Platinum 2nd Gen"),
    })
} 
