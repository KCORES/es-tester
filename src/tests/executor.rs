use std::error::Error;
use std::time::Instant;
use crate::tests::instruction_sets::InstructionSet;

pub fn execute_test(instruction_set: &InstructionSet) -> Result<(bool, std::time::Duration), Box<dyn Error>> {
    let start = Instant::now();
    let success = (instruction_set.test_function)();
    let duration = start.elapsed();
    
    Ok((success, duration))
} 
