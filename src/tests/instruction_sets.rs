pub struct InstructionSet {
    pub name: String,
    pub description: String,
    pub test_function: fn() -> bool,
}

pub fn get_instruction_sets() -> Vec<InstructionSet> {
    // TODO: Implement instruction sets list
    Vec::new()
} 
