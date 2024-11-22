#[derive(Debug, Clone, Copy)]
pub enum CpuVendor {
    Intel,
    AMD,
    Unknown,
}

#[derive(Debug, Clone, Copy)]
pub enum CpuGeneration {
    // Intel
    XeonV2,
    Unknown,
}

#[derive(Debug)]
pub struct CpuInfo {
    pub vendor: CpuVendor,
    pub generation: CpuGeneration,
    pub model_name: String,
} 
