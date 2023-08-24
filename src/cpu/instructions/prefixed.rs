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