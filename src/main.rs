use std::error::Error;

mod cpu;
mod tests;
mod config;
mod report;

#[derive(Debug, Clone, Copy)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Console,
    Json,
    Csv,
}

pub struct TestConfig {
    timeout: std::time::Duration,
    parallel: bool,
    log_level: LogLevel,
    output_format: OutputFormat,
}

fn setup_logger() -> Result<(), Box<dyn Error>> {
    // TODO: Initialize logging system
    Ok(())
}

fn load_config() -> Result<TestConfig, Box<dyn Error>> {
    // TODO: Load configuration from file or defaults
    Ok(TestConfig {
        timeout: std::time::Duration::from_secs(30),
        parallel: false,
        log_level: LogLevel::Info,
        output_format: OutputFormat::Console,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize logger
    setup_logger()?;

    // Load configuration
    let config = load_config()?;

    // Detect CPU
    let cpu_info = cpu::detection::detect_cpu()?;

    // Initialize tester
    let mut tester = tests::CpuTester::new(cpu_info, config);

    // Run tests
    tester.run_tests()?;

    // Generate and display report
    tester.generate_report()?;

    // Save results
    tester.save_results()?;

    Ok(())
}
