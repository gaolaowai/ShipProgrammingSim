use nanoserde::{DeBin, DeJson, SerBin, SerJson};



type register = u8;

#[derive(Debug, DeJson, SerJson, SerBin, DeBin)]
enum Instructions {
    NOP,
    PUSH,
    POP,
    // JUMP instruction
    JMP(u8),
    JEQ(u8),
    JLT(u8),
    JGT(u8),
    JLE(u8),
    JGE(u8),
    
    LOAD(u8),
    SAVE(u8),
    
    // Integer ops
    ADDI(register,u8),
    SUBI(register,u8),
    MULI(register,u8),
    DIVI(register,u8),

    // float ops
    ADDF(register,u8),
    SUBF(register,u8),
    MULF(register,u8),
    DIVF(register,u8)
}

enum DATA {
    INT(u32),
    FLOAT(f32)
}
struct CPU {
    result_register : DATA,
    a_register : DATA,
    b_register : DATA,
    c_register : DATA,
    d_register : DATA,

    program_counter : u32
}