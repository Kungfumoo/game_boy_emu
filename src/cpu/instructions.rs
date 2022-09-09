use super::CPU;

pub struct StateChange {
    pub byte_length: u8,
    pub t_states: u8,
    pub flags: FlagChange,
    pub register: RegisterChange,
    pub memory: MemoryChange
}

pub struct MemoryEdit {
    pub key: u16,
    pub value: u8
}

pub struct MemoryChange {
    pub changes: Vec<MemoryEdit>
}

impl MemoryChange {
    fn default() -> MemoryChange {
        MemoryChange { changes: Vec::new() }
    }
}

pub struct RegisterChange {
    pub a: Option<u8>,
    pub b: Option<u8>,
    pub c: Option<u8>,
    pub d: Option<u8>,
    pub e: Option<u8>,
    pub f: Option<u8>,
    pub h: Option<u8>,
    pub l: Option<u8>
}

impl RegisterChange {
    fn default() -> RegisterChange {
        RegisterChange {
            a: Option::None,
            b: Option::None,
            c: Option::None,
            d: Option::None,
            e: Option::None,
            f: Option::None,
            h: Option::None,
            l: Option::None
        }
    }
}

pub struct FlagChange {
    pub zero: Option<bool>,
    pub subtract: Option<bool>,
    pub half_carry: Option<bool>,
    pub carry: Option<bool>
}

impl FlagChange {
    fn default() -> FlagChange {
        FlagChange {
            zero: Option::None,
            subtract: Option::None,
            half_carry: Option::None,
            carry: Option::None
        }
    }
}

pub fn execute(cpu: &CPU, op_code: u8) -> StateChange {
    match op_code {
        0x00 => nop(),
        0x02 => ld_to_absolute(MemoryChange { //LD (BC), A
            changes: Vec::from([
                MemoryEdit {
                    key: cpu.registers.bc(),
                    value: cpu.registers.a
                }
            ])
        }),
        0x06 => ld_immediate(RegisterChange { //LD B, u8
            b: {
                let pc = cpu.registers.program_counter;

                Some(cpu.memory[(pc + 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x0E => ld_immediate(RegisterChange { //LD C, u8
            c: {
                let pc = cpu.registers.program_counter;

                Some(cpu.memory[(pc + 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x10 => stop(),
        0x3E => ld_immediate(RegisterChange { //LD A, u8
            a: {
                let pc = cpu.registers.program_counter;

                Some(cpu.memory[(pc + 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x37 => scf(),
        0x40 => ld_register_to_register(RegisterChange { //LD B, B
            b: Option::Some(cpu.registers.b),
            ..RegisterChange::default()
        }),
        0x41 => ld_register_to_register(RegisterChange { //LD B, C
            b: Option::Some(cpu.registers.c),
            ..RegisterChange::default()
        }),
        0x42 => ld_register_to_register(RegisterChange { //LD B, D
            b: Option::Some(cpu.registers.d),
            ..RegisterChange::default()
        }),
        0x43 => ld_register_to_register(RegisterChange { //LD B, E
            b: Option::Some(cpu.registers.e),
            ..RegisterChange::default()
        }),
        0x44 => ld_register_to_register(RegisterChange { //LD B, H
            b: Option::Some(cpu.registers.h),
            ..RegisterChange::default()
        }),
        0x45 => ld_register_to_register(RegisterChange { //LD B, L
            b: Option::Some(cpu.registers.l),
            ..RegisterChange::default()
        }),
        0x47 => ld_register_to_register(RegisterChange { //LD B, A
            b: Option::Some(cpu.registers.a),
            ..RegisterChange::default()
        }),
        0x76 => halt(),
        _ => StateChange {
            byte_length: 0,
            t_states: 0,
            flags: FlagChange::default(),
            register: RegisterChange::default(),
            memory: MemoryChange::default()
        }
    }
}

fn ld_to_absolute(change: MemoryChange) -> StateChange {
    StateChange {
        byte_length: 1,
        t_states: 8,
        flags: FlagChange::default(),
        register: RegisterChange::default(),
        memory: change
    }
}

fn ld_immediate(change: RegisterChange) -> StateChange {
    StateChange {
        byte_length: 2,
        t_states: 8,
        flags: FlagChange::default(),
        register: change,
        memory: MemoryChange::default()
    }
}

fn ld_register_to_register(change: RegisterChange) -> StateChange {
    StateChange {
        byte_length: 1,
        t_states: 4,
        flags: FlagChange::default(),
        register: change,
        memory: MemoryChange::default()
    }
}

fn halt() -> StateChange {
    nop() //TODO: is this the same as nop?
}

fn scf() -> StateChange {
    StateChange {
        byte_length: 1,
        t_states: 4,
        flags: FlagChange {
            carry: Some(true),
            subtract: Some(false),
            half_carry: Some(false),
            ..FlagChange::default()
        },
        register: RegisterChange::default(),
        memory: MemoryChange::default()
    }
}

fn stop() -> StateChange {
    StateChange {
        byte_length: 2,
        ..nop()
    }
}

fn nop() -> StateChange {
    //do nothing
    StateChange {
        byte_length: 1,
        t_states: 4,
        flags: FlagChange::default(),
        register: RegisterChange::default(),
        memory: MemoryChange::default()
    }
}
