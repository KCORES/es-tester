use core::arch::asm;

#[derive(Debug)]
pub struct InstructionSet {
    pub name: String,
    pub description: String,
    pub test_function: fn() -> bool,
}

// 算术运算指令测试
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

unsafe fn test_mul() -> bool {
    let mut result_low: u32;
    let mut result_high: u32;
    let a: u32 = 100;
    let b: u32 = 200;
    
    asm!(
        "mov eax, {0:e}",
        "mul {1:e}",
        "mov {2:e}, eax",
        "mov {3:e}, edx",
        in(reg) a,
        in(reg) b,
        out(reg) result_low,
        out(reg) result_high,
    );
    
    let expected: u64 = (a as u64) * (b as u64);
    result_low as u64 == (expected & 0xFFFFFFFF) && 
    result_high as u64 == (expected >> 32)
}

unsafe fn test_div() -> bool {
    let mut quotient: u32;
    let mut remainder: u32;
    let dividend: u32 = 100;
    let divisor: u32 = 3;
    
    asm!(
        "mov eax, {0:e}",
        "xor edx, edx",
        "div {1:e}",
        "mov {2:e}, eax",
        "mov {3:e}, edx",
        in(reg) dividend,
        in(reg) divisor,
        out(reg) quotient,
        out(reg) remainder,
    );
    
    quotient == (dividend / divisor) && remainder == (dividend % divisor)
}

// 逻辑运算指令测试
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

unsafe fn test_or() -> bool {
    let mut result: u32;
    let a: u32 = 0xFF00;
    let b: u32 = 0x0FF0;
    
    asm!(
        "mov eax, {0:e}",
        "or eax, {1:e}",
        "mov {2:e}, eax",
        in(reg) a,
        in(reg) b,
        out(reg) result,
    );
    
    result == (a | b)
}

unsafe fn test_xor() -> bool {
    let mut result: u32;
    let a: u32 = 0xFF00;
    let b: u32 = 0x0FF0;
    
    asm!(
        "mov eax, {0:e}",
        "xor eax, {1:e}",
        "mov {2:e}, eax",
        in(reg) a,
        in(reg) b,
        out(reg) result,
    );
    
    result == (a ^ b)
}

// 移位指令测试
unsafe fn test_shl() -> bool {
    let mut result: u32;
    let a: u32 = 0x1;
    let count: u8 = 4;
    
    asm!(
        "mov eax, {0:e}",
        "mov cl, {1}",
        "shl eax, cl",
        "mov {2:e}, eax",
        in(reg) a,
        in(reg_byte) count,
        out(reg) result,
    );
    
    result == (a << count)
}

// 比较指令测试
unsafe fn test_cmp() -> bool {
    let mut flags: u64;
    let a: u32 = 20;
    let b: u32 = 20;
    
    asm!(
        "mov eax, {0:e}",
        "cmp eax, {1:e}",
        "pushfq",
        "pop {2}",
        in(reg) a,
        in(reg) b,
        out(reg) flags,
    );
    
    // 检查ZF标志位（相等时ZF=1）
    (flags & 0x40) != 0
}

unsafe fn test_test() -> bool {
    let mut flags: u64;
    let a: u32 = 0xFF00;
    let b: u32 = 0x0F00;
    
    asm!(
        "mov eax, {0:e}",
        "test eax, {1:e}",
        "pushfq",
        "pop {2}",
        in(reg) a,
        in(reg) b,
        out(reg) flags,
    );
    
    // 检查结果是否正确（有共同的1位时ZF=0）
    (flags & 0x40) == 0
}

// 字符串操作指令测试
unsafe fn test_movs() -> bool {
    let src: [u8; 4] = [1, 2, 3, 4];
    let dst: [u8; 4] = [0; 4];
    
    asm!(
        "cld",
        "mov esi, {0:e}",
        "mov edi, {1:e}",
        "mov ecx, 4",
        "rep movsb",
        in(reg) &src,
        in(reg) &dst,
        out("ecx") _,
        out("esi") _,
        out("edi") _,
    );
    
    dst == src
}

// 标志位操作指令测试
unsafe fn test_flags() -> bool {
    let mut flags_after_stc: u64;
    let mut flags_after_clc: u64;
    
    asm!(
        "stc",
        "pushfq",
        "pop {0}",
        "clc",
        "pushfq",
        "pop {1}",
        out(reg) flags_after_stc,
        out(reg) flags_after_clc,
    );
    
    // 检查CF标志位
    (flags_after_stc & 1) == 1 && (flags_after_clc & 1) == 0
}

pub fn get_instruction_sets() -> Vec<InstructionSet> {
    let sets = vec![
        // 算术运算指令
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
            name: String::from("MUL"),
            description: String::from("Unsigned multiplication"),
            test_function: || unsafe { test_mul() },
        },
        InstructionSet {
            name: String::from("DIV"),
            description: String::from("Unsigned division"),
            test_function: || unsafe { test_div() },
        },
        // 逻辑运算指令
        InstructionSet {
            name: String::from("AND"),
            description: String::from("Logical AND operation"),
            test_function: || unsafe { test_and() },
        },
        InstructionSet {
            name: String::from("OR"),
            description: String::from("Logical OR operation"),
            test_function: || unsafe { test_or() },
        },
        InstructionSet {
            name: String::from("XOR"),
            description: String::from("Logical XOR operation"),
            test_function: || unsafe { test_xor() },
        },
        // 移位指令
        InstructionSet {
            name: String::from("SHL"),
            description: String::from("Shift left"),
            test_function: || unsafe { test_shl() },
        },
        // 比较指令
        InstructionSet {
            name: String::from("CMP"),
            description: String::from("Compare two values"),
            test_function: || unsafe { test_cmp() },
        },
        // 测试指令
        InstructionSet {
            name: String::from("TEST"),
            description: String::from("Logical compare"),
            test_function: || unsafe { test_test() },
        },
        // 字符串操作指令
        InstructionSet {
            name: String::from("MOVS"),
            description: String::from("Move string"),
            test_function: || unsafe { test_movs() },
        },
        // 标志位操作指令
        InstructionSet {
            name: String::from("FLAGS"),
            description: String::from("Flags operations (STC/CLC)"),
            test_function: || unsafe { test_flags() },
        },
    ];
    sets
} 
