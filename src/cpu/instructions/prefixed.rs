use crate::cpu::{
    CPU,
    flags::FlagChange,
    registers::RegisterChange,
    memory::{MemoryChange, MemoryEdit}
};

use super::StateChange;

pub fn prefixed_execute(cpu: &CPU, op_code: u8) -> StateChange {
    match op_code {
        0x00 => { //RLC B
            let b = cpu.registers.b.rotate_left(1);

            rotate_register(
                RegisterChange {
                    b: Some(b),
                    ..RegisterChange::default()
                },
                (b & 0x01) == 0x01, //check rightmost bit
                b == 0
            )
        },
        0x01 => { //RLC C
            let c = cpu.registers.c.rotate_left(1);

            rotate_register(
                RegisterChange {
                    c: Some(c),
                    ..RegisterChange::default()
                },
                (c & 0x01) == 0x01, //check rightmost bit
                c == 0
            )
        },
        0x02 => { //RLC D
            let d = cpu.registers.d.rotate_left(1);

            rotate_register(
                RegisterChange {
                    d: Some(d),
                    ..RegisterChange::default()
                },
                (d & 0x01) == 0x01, //check rightmost bit
                d == 0
            )
        },
        0x03 => { //RLC E
            let e = cpu.registers.e.rotate_left(1);

            rotate_register(
                RegisterChange {
                    e: Some(e),
                    ..RegisterChange::default()
                },
                (e & 0x01) == 0x01, //check rightmost bit
                e == 0
            )
        },
        0x04 => { //RLC H
            let h = cpu.registers.h.rotate_left(1);

            rotate_register(
                RegisterChange {
                    h: Some(h),
                    ..RegisterChange::default()
                },
                (h & 0x01) == 0x01, //check rightmost bit
                h == 0
            )
        },
        0x05 => { //RLC L
            let l = cpu.registers.l.rotate_left(1);

            rotate_register(
                RegisterChange {
                    l: Some(l),
                    ..RegisterChange::default()
                },
                (l & 0x01) == 0x01, //check rightmost bit
                l == 0
            )
        },
        0x06 => { //RLC [HL]
            let value = cpu.memory[cpu.registers.hl() as usize].rotate_left(1);

            StateChange {
                byte_length: 2,
                t_states: 16,
                ime: None,
                flags: FlagChange {
                    zero: Some(value == 0),
                    carry: Some((value & 0x01) == 0x01),
                    ..FlagChange::reset()
                },
                register: RegisterChange::default(),
                memory: MemoryChange {
                    changes: vec![
                        MemoryEdit {
                            key: cpu.registers.hl(),
                            value: value
                        }
                    ]
                }
            }
        },
        0x07 => { //RLC A
            let a = cpu.registers.a.rotate_left(1);

            rotate_register(
                RegisterChange {
                    a: Some(a),
                    ..RegisterChange::default()
                },
                (a & 0x01) == 0x01, //check rightmost bit
                a == 0
            )
        },
        0x08 => { //RRC B
            let b = cpu.registers.b.rotate_right(1);

            rotate_register(
                RegisterChange {
                    b: Some(b),
                    ..RegisterChange::default()
                },
                (b & 0x80) == 0x80, //check leftmost bit
                b == 0
            )
        },
        0x09 => { //RRC C
            let c = cpu.registers.c.rotate_right(1);

            rotate_register(
                RegisterChange {
                    c: Some(c),
                    ..RegisterChange::default()
                },
                (c & 0x80) == 0x80, //check leftmost bit
                c == 0
            )
        },
        0x0A => { //RRC D
            let d = cpu.registers.d.rotate_right(1);

            rotate_register(
                RegisterChange {
                    d: Some(d),
                    ..RegisterChange::default()
                },
                (d & 0x80) == 0x80, //check leftmost bit
                d == 0
            )
        },
        0x0B => { //RRC E
            let e = cpu.registers.e.rotate_right(1);

            rotate_register(
                RegisterChange {
                    e: Some(e),
                    ..RegisterChange::default()
                },
                (e & 0x80) == 0x80, //check leftmost bit
                e == 0
            )
        },
        0x0C => { //RRC H
            let h = cpu.registers.h.rotate_right(1);

            rotate_register(
                RegisterChange {
                    h: Some(h),
                    ..RegisterChange::default()
                },
                (h & 0x80) == 0x80, //check leftmost bit
                h == 0
            )
        },
        0x0D => { //RRC L
            let l = cpu.registers.l.rotate_right(1);

            rotate_register(
                RegisterChange {
                    l: Some(l),
                    ..RegisterChange::default()
                },
                (l & 0x80) == 0x80, //check leftmost bit
                l == 0
            )
        },
        0x0E => { //RRC [HL]
            let value = cpu.memory[cpu.registers.hl() as usize].rotate_right(1);

            StateChange {
                byte_length: 2,
                t_states: 16,
                ime: None,
                flags: FlagChange {
                    zero: Some(value == 0),
                    carry: Some((value & 0x80) == 0x80),
                    ..FlagChange::reset()
                },
                register: RegisterChange::default(),
                memory: MemoryChange {
                    changes: vec![
                        MemoryEdit {
                            key: cpu.registers.hl(),
                            value: value
                        }
                    ]
                }
            }
        },
        0x0F => { //RRC A
            let a = cpu.registers.a.rotate_right(1);

            rotate_register(
                RegisterChange {
                    a: Some(a),
                    ..RegisterChange::default()
                },
                (a & 0x80) == 0x80, //check leftmost bit
                a == 0
            )
        },
        0x10 => { //RL B
            let (result, set_carry) = rotate_left_through_carry(
                cpu.registers.b,
                cpu.flags.carry
            );

            rotate_register(
                RegisterChange {
                    b: Some(result as u8),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x11 => { //RL C
            let (result, set_carry) = rotate_left_through_carry(
                cpu.registers.c,
                cpu.flags.carry
            );

            rotate_register(
                RegisterChange {
                    c: Some(result as u8),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x12 => { //RL D
            let (result, set_carry) = rotate_left_through_carry(
                cpu.registers.d,
                cpu.flags.carry
            );

            rotate_register(
                RegisterChange {
                    d: Some(result as u8),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x13 => { //RL E
            let (result, set_carry) = rotate_left_through_carry(
                cpu.registers.e,
                cpu.flags.carry
            );

            rotate_register(
                RegisterChange {
                    e: Some(result as u8),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x14 => { //RL H
            let (result, set_carry) = rotate_left_through_carry(
                cpu.registers.h,
                cpu.flags.carry
            );

            rotate_register(
                RegisterChange {
                    h: Some(result as u8),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x15 => { //RL L
            let (result, set_carry) = rotate_left_through_carry(
                cpu.registers.l,
                cpu.flags.carry
            );

            rotate_register(
                RegisterChange {
                    l: Some(result as u8),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x16 => { //RL [HL]
            let (result, set_carry) = rotate_left_through_carry(
                cpu.memory[cpu.registers.hl() as usize],
                cpu.flags.carry
            );

            StateChange {
                byte_length: 2,
                t_states: 16,
                ime: None,
                flags: FlagChange {
                    zero: Some(result == 0),
                    carry: Some(set_carry),
                    ..FlagChange::reset()
                },
                register: RegisterChange::default(),
                memory: MemoryChange {
                    changes: vec![
                        MemoryEdit {
                            key: cpu.registers.hl(),
                            value: result
                        }
                    ]
                }
            }
        },
        0x17 => { //RL A
            let (result, set_carry) = rotate_left_through_carry(
                cpu.registers.a,
                cpu.flags.carry
            );

            rotate_register(
                RegisterChange {
                    a: Some(result as u8),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x18 => { //RR B
            let (result, set_carry) = rotate_right_through_carry(
                cpu.registers.b,
                cpu.flags.carry
            );

            rotate_register(
                RegisterChange {
                    b: Some(result as u8),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x19 => { //RR C
            let (result, set_carry) = rotate_right_through_carry(
                cpu.registers.c,
                cpu.flags.carry
            );

            rotate_register(
                RegisterChange {
                    c: Some(result as u8),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x1A => { //RR D
            let (result, set_carry) = rotate_right_through_carry(
                cpu.registers.d,
                cpu.flags.carry
            );

            rotate_register(
                RegisterChange {
                    d: Some(result as u8),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x1B => { //RR E
            let (result, set_carry) = rotate_right_through_carry(
                cpu.registers.e,
                cpu.flags.carry
            );

            rotate_register(
                RegisterChange {
                    e: Some(result as u8),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x1C => { //RR H
            let (result, set_carry) = rotate_right_through_carry(
                cpu.registers.h,
                cpu.flags.carry
            );

            rotate_register(
                RegisterChange {
                    h: Some(result as u8),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x1D => { //RR L
            let (result, set_carry) = rotate_right_through_carry(
                cpu.registers.l,
                cpu.flags.carry
            );

            rotate_register(
                RegisterChange {
                    l: Some(result as u8),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x1E => { //RR [HL]
            let (result, set_carry) = rotate_right_through_carry(
                cpu.memory[cpu.registers.hl() as usize],
                cpu.flags.carry
            );

            StateChange {
                byte_length: 2,
                t_states: 16,
                ime: None,
                flags: FlagChange {
                    zero: Some(result == 0),
                    carry: Some(set_carry),
                    ..FlagChange::reset()
                },
                register: RegisterChange::default(),
                memory: MemoryChange {
                    changes: vec![
                        MemoryEdit {
                            key: cpu.registers.hl(),
                            value: result
                        }
                    ]
                }
            }
        },
        0x1F => { //RR A
            let (result, set_carry) = rotate_right_through_carry(
                cpu.registers.a,
                cpu.flags.carry
            );

            rotate_register(
                RegisterChange {
                    a: Some(result as u8),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        _ => StateChange {
            byte_length: 0,
            t_states: 0,
            ime: Option::None,
            flags: FlagChange::default(),
            register: RegisterChange::default(),
            memory: MemoryChange::default()
        }
    }
}

fn rotate_right_through_carry(value: u8, carry: bool) -> (u8, bool) {
    let set_carry = (value & 0x01) == 0x01;
    let mut value = value >> 1;

    if carry {
        value += 0x80; //+0x80 will set the leftmost bit
    }

    (value, set_carry)
}

fn rotate_left_through_carry(value: u8, carry: bool) -> (u8, bool) {
    let set_carry = (value & 0x80) == 0x80;
    let mut value = value << 1;

    if carry {
        value += 0x01; //+1 will set the rightmost bit
    }

    (value, set_carry)
}

fn rotate_register(change: RegisterChange, set_carry: bool, set_zero: bool) -> StateChange {
    StateChange {
        byte_length: 2,
        t_states: 8,
        ime: Option::None,
        flags: FlagChange {
            carry: Some(set_carry),
            zero: Some(set_zero),
            ..FlagChange::reset()
        },
        register: change,
        memory: MemoryChange::default()
    }
}