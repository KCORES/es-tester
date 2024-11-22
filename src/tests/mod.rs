pub mod instruction_sets;
pub mod executor;

use std::error::Error;
use crate::cpu::CpuInfo;
use crate::TestConfig;

pub struct TestResult {
    pub instruction_set: String,
    pub success: bool,
    pub execution_time: std::time::Duration,
    pub error_message: Option<String>,
}

pub struct CpuTester {
    cpu_info: CpuInfo,
    test_results: Vec<TestResult>,
    config: TestConfig,
}

impl CpuTester {
    pub fn new(cpu_info: CpuInfo, config: TestConfig) -> Self {
        Self {
            cpu_info,
            test_results: Vec::new(),
            config,
        }
    }

    pub fn run_tests(&mut self) -> Result<(), Box<dyn Error>> {
        // TODO: Implement test execution logic
        Ok(())
    }

    pub fn generate_report(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Implement report generation
        Ok(())
    }

    pub fn save_results(&self) -> Result<(), Box<dyn Error>> {
        // TODO: Implement results saving
        Ok(())
    }
} 
