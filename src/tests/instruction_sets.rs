use core::arch::asm;

#[derive(Debug)]
pub struct InstructionSet {
    pub name: String,
    pub description: String,
    pub test_function: fn() -> bool,
}

// 测试ADD指令
unsafe fn test_add() -> bool {
    let mut result: u32;
    let a: u32 = 10;
    let b: u32 = 20;
    
    asm!(
        "mov eax, {0:e}",
        "add eax, {1:e}",
        "mov {2:e}, eax",
        in(reg) a,
        in(reg) b,
        out(reg) result,
    );
    
    result == (a + b)
}

// 测试SUB指令
unsafe fn test_sub() -> bool {
    let mut result: u32;
    let a: u32 = 30;
    let b: u32 = 20;
    
    asm!(
        "mov eax, {0:e}",
        "sub eax, {1:e}",
        "mov {2:e}, eax",
        in(reg) a,
        in(reg) b,
        out(reg) result,
    );
    
    result == (a - b)
}

// 测试AND指令
unsafe fn test_and() -> bool {
    let mut result: u32;
    let a: u32 = 0xFF00;
    let b: u32 = 0x0FF0;
    
    asm!(
        "mov eax, {0:e}",
        "and eax, {1:e}",
        "mov {2:e}, eax",
        in(reg) a,
        in(reg) b,
        out(reg) result,
    );
    
    result == (a & b)
}

pub fn get_instruction_sets() -> Vec<InstructionSet> {
    vec![
        InstructionSet {
            name: String::from("ADD"),
            description: String::from("Basic integer addition"),
            test_function: || unsafe { test_add() },
        },
        InstructionSet {
            name: String::from("SUB"),
            description: String::from("Basic integer subtraction"),
            test_function: || unsafe { test_sub() },
        },
        InstructionSet {
            name: String::from("AND"),
            description: String::from("Logical AND operation"),
            test_function: || unsafe { test_and() },
        },
    ]
} 
