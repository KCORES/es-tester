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
        let instruction_sets = instruction_sets::get_instruction_sets();
        
        for inst_set in instruction_sets {
            // First print what we're about to test
            println!("Testing {}...", inst_set.name);
            
            // Execute the test
            let (success, duration) = executor::execute_test(&inst_set)?;
            
            // Store the result
            self.test_results.push(TestResult {
                instruction_set: inst_set.name.clone(),
                success,
                execution_time: duration,
                error_message: if success { None } else { Some("Test failed".to_string()) },
            });
            
            // Print the result after completion
            println!(
                "Result for {}: {} (took {:?})",
                inst_set.name,
                if success { "PASS" } else { "FAIL" },
                duration
            );
            println!("----------------------------------------");
        }
        
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
