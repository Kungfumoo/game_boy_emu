use super::{
    CPU,
    registers::{to8_bit, to16_bit},
    flags::{is_half_carry_add, is_half_carry_subtract}
};

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
    pub sp: Option<u16>,
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
            sp: Option::None,
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
        0x01 => ld16_immediate({ //LD BC, u16
            let pc = cpu.registers.program_counter;

            RegisterChange {
                b: Some(cpu.memory[(pc + 1) as usize]),
                c: Some(cpu.memory[(pc + 2) as usize]),
                ..RegisterChange::default()
            }
        }),
        0x02 => ld_to_absolute(MemoryChange { //LD (BC), A
            changes: Vec::from([
                MemoryEdit {
                    key: cpu.registers.bc(),
                    value: cpu.registers.a
                }
            ])
        }),
        0x03 => inc16_bit({ //INC BC
            let bc = cpu.registers.bc() + 1;
            let (b, c) = to8_bit(bc);

            RegisterChange {
                b: Option::Some(b),
                c: Option::Some(c),
                ..RegisterChange::default()
            }
        }),
        0x04 => { //INC B
            let value = cpu.registers.b + 1;

            inc8_bit(
                RegisterChange {
                    b: Option::Some(value),
                    ..RegisterChange::default()
                },
                value == 0,
                is_half_carry_add(cpu.registers.b, 1)
            )
        },
        0x05 => { //DEC B
            let value = cpu.registers.b - 1;

            dec8_bit(
                RegisterChange {
                    b: Option::Some(value),
                    ..RegisterChange::default()
                },
                value == 0,
                is_half_carry_subtract(cpu.registers.b, 1)
            )
        },
        0x06 => ld_immediate(RegisterChange { //LD B, u8
            b: {
                let pc = cpu.registers.program_counter;

                Some(cpu.memory[(pc + 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x0C => { //INC C
            let value = cpu.registers.c + 1;

            inc8_bit(
                RegisterChange {
                    c: Option::Some(value),
                    ..RegisterChange::default()
                },
                value == 0,
                is_half_carry_add(cpu.registers.c, 1)
            )
        },
        0x0D => { //DEC C
            let value = cpu.registers.c - 1;

            dec8_bit(
                RegisterChange {
                    c: Option::Some(value),
                    ..RegisterChange::default()
                },
                value == 0,
                is_half_carry_subtract(cpu.registers.c, 1)
            )
        },
        0x0E => ld_immediate(RegisterChange { //LD C, u8
            c: {
                let pc = cpu.registers.program_counter;

                Some(cpu.memory[(pc + 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x10 => StateChange {
            byte_length: 2,
            ..nop()
        },
        0x11 => ld16_immediate({ //LD DE, u16
            let pc = cpu.registers.program_counter;

            RegisterChange {
                d: Some(cpu.memory[(pc + 1) as usize]),
                e: Some(cpu.memory[(pc + 2) as usize]),
                ..RegisterChange::default()
            }
        }),
        0x12 => ld_to_absolute(MemoryChange { //LD (DE), A
            changes: Vec::from([
                MemoryEdit {
                    key: cpu.registers.de(),
                    value: cpu.registers.a
                }
            ])
        }),
        0x13 => inc16_bit({ //INC DE
            let de = cpu.registers.de() + 1;
            let (d, e) = to8_bit(de);

            RegisterChange {
                d: Option::Some(d),
                e: Option::Some(e),
                ..RegisterChange::default()
            }
        }),
        0x14 => { //INC D
            let value = cpu.registers.d + 1;

            inc8_bit(
                RegisterChange {
                    d: Option::Some(value),
                    ..RegisterChange::default()
                },
                value == 0,
                is_half_carry_add(cpu.registers.d, 1)
            )
        },
        0x15 => { //DEC D
            let value = cpu.registers.d - 1;

            dec8_bit(
                RegisterChange {
                    d: Option::Some(value),
                    ..RegisterChange::default()
                },
                value == 0,
                is_half_carry_subtract(cpu.registers.d, 1)
            )
        },
        0x16 => ld_immediate(RegisterChange { //LD D, u8
            d: {
                let pc = cpu.registers.program_counter;

                Some(cpu.memory[(pc + 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x1C => { //INC E
            let value = cpu.registers.e + 1;

            inc8_bit(
                RegisterChange {
                    e: Option::Some(value),
                    ..RegisterChange::default()
                },
                value == 0,
                is_half_carry_add(cpu.registers.e, 1)
            )
        },
        0x1D => { //DEC E
            let value = cpu.registers.e - 1;

            dec8_bit(
                RegisterChange {
                    e: Option::Some(value),
                    ..RegisterChange::default()
                },
                value == 0,
                is_half_carry_subtract(cpu.registers.e, 1)
            )
        },
        0x1E => ld_immediate(RegisterChange { //LD E, u8
            e: {
                let pc = cpu.registers.program_counter;

                Some(cpu.memory[(pc + 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x21 => ld16_immediate({ //LD HL, u16
            let pc = cpu.registers.program_counter;

            RegisterChange {
                h: Some(cpu.memory[(pc + 1) as usize]),
                l: Some(cpu.memory[(pc + 2) as usize]),
                ..RegisterChange::default()
            }
        }),
        0x22 => {  //LD (HL+), A
            let addr = cpu.registers.hl();
            let change = ld_to_absolute(MemoryChange {
                changes: Vec::from([
                    MemoryEdit {
                        key: addr,
                        value: cpu.registers.a
                    }
                ])
            });

            let (h, l) = to8_bit(addr + 1);

            StateChange {
                register: RegisterChange {
                    h: Option::Some(h),
                    l: Option::Some(l),
                    ..change.register
                },
                ..change
            }
        },
        0x23 => inc16_bit({ //INC HL
            let hl = cpu.registers.hl() + 1;
            let (h, l) = to8_bit(hl);

            RegisterChange {
                h: Option::Some(h),
                l: Option::Some(l),
                ..RegisterChange::default()
            }
        }),
        0x24 => { //INC H
            let value = cpu.registers.h + 1;

            inc8_bit(
                RegisterChange {
                    h: Option::Some(value),
                    ..RegisterChange::default()
                },
                value == 0,
                is_half_carry_add(cpu.registers.h, 1)
            )
        },
        0x25 => { //DEC H
            let value = cpu.registers.h - 1;

            dec8_bit(
                RegisterChange {
                    h: Option::Some(value),
                    ..RegisterChange::default()
                },
                value == 0,
                is_half_carry_subtract(cpu.registers.h, 1)
            )
        },
        0x26 => ld_immediate(RegisterChange { //LD H, u8
            h: {
                let pc = cpu.registers.program_counter;

                Some(cpu.memory[(pc + 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x2C => { //INC L
            let value = cpu.registers.l + 1;

            inc8_bit(
                RegisterChange {
                    l: Option::Some(value),
                    ..RegisterChange::default()
                },
                value == 0,
                is_half_carry_add(cpu.registers.l, 1)
            )
        },
        0x2D => { //DEC L
            let value = cpu.registers.l - 1;

            dec8_bit(
                RegisterChange {
                    l: Option::Some(value),
                    ..RegisterChange::default()
                },
                value == 0,
                is_half_carry_subtract(cpu.registers.l, 1)
            )
        },
        0x2E => ld_immediate(RegisterChange { //LD L, u8
            l: {
                let pc = cpu.registers.program_counter;

                Some(cpu.memory[(pc + 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x31 => ld16_immediate({ //LD SP, u16
            let pc = cpu.registers.program_counter;

            RegisterChange {
                sp: Option::Some(to16_bit(
                    cpu.memory[(pc + 1) as usize],
                    cpu.memory[(pc + 2) as usize]
                )),
                ..RegisterChange::default()
            }
        }),
        0x32 => {  //LD (HL-), A
            let addr = cpu.registers.hl();
            let change = ld_to_absolute(MemoryChange {
                changes: Vec::from([
                    MemoryEdit {
                        key: addr,
                        value: cpu.registers.a
                    }
                ])
            });

            let (h, l) = to8_bit(addr - 1);

            StateChange {
                register: RegisterChange {
                    h: Option::Some(h),
                    l: Option::Some(l),
                    ..change.register
                },
                ..change
            }
        },
        0x33 => inc16_bit({ //INC SP
            let sp = cpu.registers.stack_pointer + 1;

            RegisterChange {
                sp: Option::Some(sp),
                ..RegisterChange::default()
            }
        }),
        0x34 => { //INC (HL)
            let addr = cpu.registers.hl();
            let value = cpu.memory[addr as usize];
            let result = value + 1;

            inc_absolute(
                MemoryChange {
                    changes: Vec::from([
                        MemoryEdit {
                            key: addr,
                            value: result
                        }
                    ])
                },
                result == 0,
                is_half_carry_add(value, 1)
            )
        },
        0x35 => { //DEC (HL)
            let addr = cpu.registers.hl();
            let value = cpu.memory[addr as usize];
            let result = value - 1;

            dec_absolute(
                MemoryChange {
                    changes: Vec::from([
                        MemoryEdit {
                            key: addr,
                            value: result
                        }
                    ])
                },
                result == 0,
                is_half_carry_subtract(value, 1)
            )
        },
        0x36 => { //LD (HL), u8
            let pc = cpu.registers.program_counter;
            let arg = cpu.memory[(pc + 1) as usize];

            StateChange {
                byte_length: 2,
                t_states: 12,
                flags: FlagChange::default(),
                register: RegisterChange::default(),
                memory: MemoryChange {
                    changes: Vec::from([
                        MemoryEdit {
                            key: cpu.registers.hl(),
                            value: arg
                        }
                    ])
                }
            }
        },
        0x3C => { //INC A
            let value = cpu.registers.a + 1;

            inc8_bit(
                RegisterChange {
                    a: Option::Some(value),
                    ..RegisterChange::default()
                },
                value == 0,
                is_half_carry_add(cpu.registers.a, 1)
            )
        },
        0x3D => { //DEC A
            let value = cpu.registers.a - 1;

            dec8_bit(
                RegisterChange {
                    a: Option::Some(value),
                    ..RegisterChange::default()
                },
                value == 0,
                is_half_carry_subtract(cpu.registers.a, 1)
            )
        },
        0x3E => ld_immediate(RegisterChange { //LD A, u8
            a: {
                let pc = cpu.registers.program_counter;

                Some(cpu.memory[(pc + 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x37 => StateChange {
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
        },
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
        0x76 => nop(), //TODO: is this the same as nop?
        _ => StateChange {
            byte_length: 0,
            t_states: 0,
            flags: FlagChange::default(),
            register: RegisterChange::default(),
            memory: MemoryChange::default()
        }
    }
}

fn dec_absolute(change: MemoryChange, set_zero: bool, set_half_carry: bool) -> StateChange {
    StateChange {
        byte_length: 1,
        t_states: 12,
        flags: FlagChange {
            zero: Option::Some(set_zero),
            subtract: Option::Some(true),
            half_carry: Option::Some(set_half_carry),
            ..FlagChange::default()
        },
        register: RegisterChange::default(),
        memory: change
    }
}

fn dec8_bit(change: RegisterChange, set_zero: bool, set_half_carry: bool) -> StateChange {
    StateChange {
        byte_length: 1,
        t_states: 4,
        flags: FlagChange {
            zero: Option::Some(set_zero),
            subtract: Option::Some(true),
            half_carry: Option::Some(set_half_carry),
            ..FlagChange::default()
        },
        register: change,
        memory: MemoryChange::default()
    }
}

fn inc_absolute(change: MemoryChange, set_zero: bool, set_half_carry: bool) -> StateChange {
    StateChange {
        byte_length: 1,
        t_states: 12,
        flags: FlagChange {
            zero: Option::Some(set_zero),
            subtract: Option::Some(false),
            half_carry: Option::Some(set_half_carry),
            ..FlagChange::default()
        },
        register: RegisterChange::default(),
        memory: change
    }
}

fn inc8_bit(change: RegisterChange, set_zero: bool, set_half_carry: bool) -> StateChange {
    StateChange {
        byte_length: 1,
        t_states: 4,
        flags: FlagChange {
            zero: Option::Some(set_zero),
            subtract: Option::Some(false),
            half_carry: Option::Some(set_half_carry),
            ..FlagChange::default()
        },
        register: change,
        memory: MemoryChange::default()
    }
}

fn inc16_bit(change: RegisterChange) -> StateChange {
    StateChange {
        byte_length: 1,
        t_states: 8,
        flags: FlagChange::default(),
        register: change,
        memory: MemoryChange::default()
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

fn ld16_immediate(change: RegisterChange) -> StateChange {
    StateChange {
        byte_length: 3,
        t_states: 12,
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
