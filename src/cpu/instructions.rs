use std::num::Wrapping;

use super::{
    CPU,
    registers::{to8_bit, to16_bit, RegisterChange},
    flags::{
        FlagChange,
        is_half_carry_add, is_half_carry_subtract,
        is_carry_add_16, is_half_carry_add_16
    },
    memory::{MemoryChange, MemoryEdit}
};

pub struct StateChange {
    pub byte_length: u8,
    pub t_states: u8,
    pub flags: FlagChange,
    pub register: RegisterChange,
    pub memory: MemoryChange
}

//How to interpret instruction comments:
//INC A = Increment the value in register A
//INC (A) = Increment the value at the memory address that the A register contains.
pub fn execute(cpu: &CPU, op_code: u8) -> StateChange {
    match op_code {
        0x00 => nop(),
        0x01 => ld16_immediate({ //LD BC, u16
            let pc = cpu.registers.program_counter;

            RegisterChange {
                b: Some(cpu.memory[add16_bit(pc, 1) as usize]),
                c: Some(cpu.memory[add16_bit(pc, 2) as usize]),
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
            let bc = add16_bit(cpu.registers.bc(), 1);
            let (b, c) = to8_bit(bc);

            RegisterChange {
                b: Option::Some(b),
                c: Option::Some(c),
                ..RegisterChange::default() //NOTE: it does NOT set the status registers, not a bug.
            }
        }),
        0x04 => { //INC B
            let value = add8_bit(cpu.registers.b, 1);

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
            let value = sub8_bit(cpu.registers.b, 1);

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

                Some(cpu.memory[add16_bit(pc, 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x07 => { //RLCA (rotate register A left)
            let a = cpu.registers.a.rotate_left(1);

            rotate_register(
                RegisterChange {
                    a: Some(a),
                    ..RegisterChange::default()
                },
                (a & 0x01) == 0x01 //check rightmost bit
            )
        },
        0x08 => StateChange { //LD [n16], SP (load stack pointer into memory)
            byte_length: 3,
            t_states: 20,
            flags: FlagChange::default(),
            register: RegisterChange::default(),
            memory: {
                let pc = cpu.registers.program_counter;
                let addr = to16_bit(
                    cpu.memory[add16_bit(pc, 1) as usize],
                    cpu.memory[add16_bit(pc, 2) as usize]
                );

                let (left, right) = to8_bit(cpu.registers.stack_pointer);

                MemoryChange {
                    changes: vec![
                        MemoryEdit {
                            key: addr,
                            value: left
                        },
                        MemoryEdit {
                            key: add16_bit(addr, 1),
                            value: right
                        }
                    ]
                }
            }
        },
        0x09 => add_to_hl( //ADD HL, BC
            cpu.registers.bc(),
            cpu
        ),
        0x0B => dec16_bit({ //DEC BC
            let bc = sub16_bit(cpu.registers.bc(), 1);
            let (b, c) = to8_bit(bc);

            RegisterChange {
                b: Option::Some(b),
                c: Option::Some(c),
                ..RegisterChange::default()
            }
        }),
        0x0C => { //INC C
            let value = add8_bit(cpu.registers.c, 1);

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
            let value = sub8_bit(cpu.registers.c, 1);

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

                Some(cpu.memory[add16_bit(pc, 1) as usize])
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
                d: Some(cpu.memory[add16_bit(pc, 1) as usize]),
                e: Some(cpu.memory[add16_bit(pc, 2) as usize]),
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
            let de = add16_bit(cpu.registers.de(), 1);
            let (d, e) = to8_bit(de);

            RegisterChange {
                d: Option::Some(d),
                e: Option::Some(e),
                ..RegisterChange::default()
            }
        }),
        0x14 => { //INC D
            let value = add8_bit(cpu.registers.d, 1);

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
            let value = sub8_bit(cpu.registers.d, 1);

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

                Some(cpu.memory[add16_bit(pc, 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x1B => dec16_bit({ //DEC DE
            let de = sub16_bit(cpu.registers.de(), 1);
            let (d, e) = to8_bit(de);

            RegisterChange {
                d: Option::Some(d),
                e: Option::Some(e),
                ..RegisterChange::default()
            }
        }),
        0x1C => { //INC E
            let value = add8_bit(cpu.registers.e, 1);

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
            let value = sub8_bit(cpu.registers.e, 1);

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

                Some(cpu.memory[add16_bit(pc, 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x21 => ld16_immediate({ //LD HL, u16
            let pc = cpu.registers.program_counter;

            RegisterChange {
                h: Some(cpu.memory[add16_bit(pc, 1) as usize]),
                l: Some(cpu.memory[add16_bit(pc, 2) as usize]),
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

            let (h, l) = to8_bit(add16_bit(addr, 1));

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
            let hl = add16_bit(cpu.registers.hl(), 1);
            let (h, l) = to8_bit(hl);

            RegisterChange {
                h: Option::Some(h),
                l: Option::Some(l),
                ..RegisterChange::default()
            }
        }),
        0x24 => { //INC H
            let value = add8_bit(cpu.registers.h, 1);

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
            let value = sub8_bit(cpu.registers.h, 1);

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

                Some(cpu.memory[add16_bit(pc, 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x2B => dec16_bit({ //DEC HL
            let hl = sub16_bit(cpu.registers.hl(), 1);
            let (h, l) = to8_bit(hl);

            RegisterChange {
                h: Option::Some(h),
                l: Option::Some(l),
                ..RegisterChange::default()
            }
        }),
        0x2C => { //INC L
            let value = add8_bit(cpu.registers.l, 1);

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
            let value = sub8_bit(cpu.registers.l, 1);

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

                Some(cpu.memory[add16_bit(pc, 1) as usize])
            },
            ..RegisterChange::default()
        }),
        0x31 => ld16_immediate({ //LD SP, u16
            let pc = cpu.registers.program_counter;

            RegisterChange {
                sp: Option::Some(to16_bit(
                    cpu.memory[add16_bit(pc, 1) as usize],
                    cpu.memory[add16_bit(pc, 2) as usize]
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

            let (h, l) = to8_bit(
                sub16_bit(addr, 1)
            );

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
            let sp = add16_bit(cpu.registers.stack_pointer, 1);

            RegisterChange {
                sp: Option::Some(sp),
                ..RegisterChange::default()
            }
        }),
        0x34 => { //INC (HL)
            let addr = cpu.registers.hl();
            let value = cpu.memory[addr as usize];
            let result = add8_bit(value, 1);

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
            let result = sub8_bit(value, 1);

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
            let arg = cpu.memory[add16_bit(pc, 1) as usize];

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
        0x3B => dec16_bit({ //DEC SP
            let sp = sub16_bit(cpu.registers.stack_pointer, 1);

            RegisterChange {
                sp: Option::Some(sp),
                ..RegisterChange::default()
            }
        }),
        0x3C => { //INC A
            let value = add8_bit(cpu.registers.a, 1);

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
            let value = sub8_bit(cpu.registers.a, 1);

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

                Some(cpu.memory[add16_bit(pc, 1) as usize])
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

fn add_to_hl(operand: u16, cpu: &CPU) -> StateChange {
    let hl = cpu.registers.hl();

    StateChange {
        byte_length: 1,
        t_states: 8,
        flags: FlagChange {
            subtract: Some(false),
            carry: Some(is_carry_add_16(hl, operand)),
            half_carry: Some(is_half_carry_add_16(hl, operand)),
            ..FlagChange::default()
        },
        memory: MemoryChange::default(),
        register: {
            let (h, l) = to8_bit(add16_bit(hl, operand));

            RegisterChange {
                h: Some(h),
                l: Some(l),
                ..RegisterChange::default()
            }
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

fn dec16_bit(change: RegisterChange) -> StateChange {
    StateChange {
        byte_length: 1,
        t_states: 8,
        flags: FlagChange::default(),
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

fn rotate_register(change: RegisterChange, set_carry: bool) -> StateChange {
    StateChange {
        byte_length: 1,
        t_states: 4,
        flags: FlagChange {
            carry: Some(set_carry),
            ..FlagChange::reset()
        },
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

//Below adders and subtracters make use of Wrapping to safely handle overflows without the program crashing
fn add8_bit(a: u8, b: u8) -> u8 {
    let a = Wrapping(a);
    let b = Wrapping(b);

    (a + b).0
}

fn add16_bit(a: u16, b: u16) -> u16 {
    let a = Wrapping(a);
    let b = Wrapping(b);

    (a + b).0
}

fn sub8_bit(a: u8, b: u8) -> u8 {
    let a = Wrapping(a);
    let b = Wrapping(b);

    (a - b).0
}

fn sub16_bit(a: u16, b: u16) -> u16 {
    let a = Wrapping(a);
    let b = Wrapping(b);

    (a - b).0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add8_bit() {
        let result = add8_bit(0xFF, 0x03);

        assert_eq!(0x02, result);
    }

    #[test]
    fn test_add16_bit() {
        let result = add16_bit(0xFFFF, 0x03);

        assert_eq!(0x02, result);
    }

    #[test]
    fn test_sub8_bit() {
        let result = sub8_bit(0x00, 0x01);

        assert_eq!(0xFF, result);
    }

    #[test]
    fn test_sub16_bit() {
        let result = sub16_bit(0x00, 0x01);

        assert_eq!(0xFFFF, result);
    }
}