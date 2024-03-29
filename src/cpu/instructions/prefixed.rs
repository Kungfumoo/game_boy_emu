use crate::cpu::{
    CPU,
    flags::FlagChange,
    registers::RegisterChange,
    memory::{MemoryChange, MemoryEdit}, util::BINARY_BASE
};

use super::StateChange;

pub fn prefixed_execute(cpu: &CPU, op_code: u8) -> StateChange {
    match op_code {
        0x00 => { //RLC B
            let b = cpu.registers.b.rotate_left(1);

            rotate_shift_register(
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

            rotate_shift_register(
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

            rotate_shift_register(
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

            rotate_shift_register(
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

            rotate_shift_register(
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

            rotate_shift_register(
                RegisterChange {
                    l: Some(l),
                    ..RegisterChange::default()
                },
                (l & 0x01) == 0x01, //check rightmost bit
                l == 0
            )
        },
        0x06 => { //RLC [HL]
            let result = cpu.memory[cpu.registers.hl() as usize].rotate_left(1);
            let set_carry = (result & 0x01) == 0x01;

            rotate_shift_absolute(
                MemoryChange {
                    changes: vec![
                        MemoryEdit {
                            key: cpu.registers.hl(),
                            value: result
                        }
                    ]
                },
                set_carry,
                result == 0
            )
        },
        0x07 => { //RLC A
            let a = cpu.registers.a.rotate_left(1);

            rotate_shift_register(
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

            rotate_shift_register(
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

            rotate_shift_register(
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

            rotate_shift_register(
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

            rotate_shift_register(
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

            rotate_shift_register(
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

            rotate_shift_register(
                RegisterChange {
                    l: Some(l),
                    ..RegisterChange::default()
                },
                (l & 0x80) == 0x80, //check leftmost bit
                l == 0
            )
        },
        0x0E => { //RRC [HL]
            let result = cpu.memory[cpu.registers.hl() as usize].rotate_right(1);
            let set_carry = (result & 0x80) == 0x80;

            rotate_shift_absolute(
                MemoryChange {
                    changes: vec![
                        MemoryEdit {
                            key: cpu.registers.hl(),
                            value: result
                        }
                    ]
                },
                set_carry,
                result == 0
            )
        },
        0x0F => { //RRC A
            let a = cpu.registers.a.rotate_right(1);

            rotate_shift_register(
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

            rotate_shift_register(
                RegisterChange {
                    b: Some(result),
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

            rotate_shift_register(
                RegisterChange {
                    c: Some(result),
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

            rotate_shift_register(
                RegisterChange {
                    d: Some(result),
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

            rotate_shift_register(
                RegisterChange {
                    e: Some(result),
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

            rotate_shift_register(
                RegisterChange {
                    h: Some(result),
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

            rotate_shift_register(
                RegisterChange {
                    l: Some(result),
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

            rotate_shift_absolute(
                MemoryChange {
                    changes: vec![
                        MemoryEdit {
                            key: cpu.registers.hl(),
                            value: result
                        }
                    ]
                },
                set_carry,
                result == 0
            )
        },
        0x17 => { //RL A
            let (result, set_carry) = rotate_left_through_carry(
                cpu.registers.a,
                cpu.flags.carry
            );

            rotate_shift_register(
                RegisterChange {
                    a: Some(result),
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

            rotate_shift_register(
                RegisterChange {
                    b: Some(result),
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

            rotate_shift_register(
                RegisterChange {
                    c: Some(result),
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

            rotate_shift_register(
                RegisterChange {
                    d: Some(result),
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

            rotate_shift_register(
                RegisterChange {
                    e: Some(result),
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

            rotate_shift_register(
                RegisterChange {
                    h: Some(result),
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

            rotate_shift_register(
                RegisterChange {
                    l: Some(result),
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

            rotate_shift_absolute(
                MemoryChange {
                    changes: vec![
                        MemoryEdit {
                            key: cpu.registers.hl(),
                            value: result
                        }
                    ]
                },
                set_carry,
                result == 0
            )
        },
        0x1F => { //RR A
            let (result, set_carry) = rotate_right_through_carry(
                cpu.registers.a,
                cpu.flags.carry
            );

            rotate_shift_register(
                RegisterChange {
                    a: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x20 => { //SLA B
            let (result, set_carry) = shift_left_arithmetically(
                cpu.registers.b
            );

            rotate_shift_register(
                RegisterChange {
                    b: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x21 => { //SLA C
            let (result, set_carry) = shift_left_arithmetically(
                cpu.registers.c
            );

            rotate_shift_register(
                RegisterChange {
                    c: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x22 => { //SLA D
            let (result, set_carry) = shift_left_arithmetically(
                cpu.registers.d
            );

            rotate_shift_register(
                RegisterChange {
                    d: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x23 => { //SLA E
            let (result, set_carry) = shift_left_arithmetically(
                cpu.registers.e
            );

            rotate_shift_register(
                RegisterChange {
                    e: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x24 => { //SLA H
            let (result, set_carry) = shift_left_arithmetically(
                cpu.registers.h
            );

            rotate_shift_register(
                RegisterChange {
                    h: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x25 => { //SLA L
            let (result, set_carry) = shift_left_arithmetically(
                cpu.registers.l
            );

            rotate_shift_register(
                RegisterChange {
                    l: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x26 => { //SLA [HL]
            let (result, set_carry) = shift_left_arithmetically(
                cpu.memory[cpu.registers.hl() as usize]
            );

            rotate_shift_absolute(
                MemoryChange {
                    changes: vec![
                        MemoryEdit {
                            key: cpu.registers.hl(),
                            value: result
                        }
                    ]
                },
                set_carry,
                result == 0
            )
        },
        0x27 => { //SLA A
            let (result, set_carry) = shift_left_arithmetically(
                cpu.registers.a
            );

            rotate_shift_register(
                RegisterChange {
                    a: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x28 => { //SRA B
            let (result, set_carry) = shift_right_arithmetically(
                cpu.registers.b
            );

            rotate_shift_register(
                RegisterChange {
                    b: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x29 => { //SRA C
            let (result, set_carry) = shift_right_arithmetically(
                cpu.registers.c
            );

            rotate_shift_register(
                RegisterChange {
                    c: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x2A => { //SRA D
            let (result, set_carry) = shift_right_arithmetically(
                cpu.registers.d
            );

            rotate_shift_register(
                RegisterChange {
                    d: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x2B => { //SRA E
            let (result, set_carry) = shift_right_arithmetically(
                cpu.registers.e
            );

            rotate_shift_register(
                RegisterChange {
                    e: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x2C => { //SRA H
            let (result, set_carry) = shift_right_arithmetically(
                cpu.registers.h
            );

            rotate_shift_register(
                RegisterChange {
                    h: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x2D => { //SRA L
            let (result, set_carry) = shift_right_arithmetically(
                cpu.registers.l
            );

            rotate_shift_register(
                RegisterChange {
                    l: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x2E => { //SRA [HL]
            let (result, set_carry) = shift_right_arithmetically(
                cpu.memory[cpu.registers.hl() as usize]
            );

            rotate_shift_absolute(
                MemoryChange {
                    changes: vec![
                        MemoryEdit {
                            key: cpu.registers.hl(),
                            value: result
                        }
                    ]
                },
                set_carry,
                result == 0
            )
        },
        0x2F => { //SRA A
            let (result, set_carry) = shift_right_arithmetically(
                cpu.registers.a
            );

            rotate_shift_register(
                RegisterChange {
                    a: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x30 => { //SWAP B
            let result = swap(cpu.registers.b);

            swap_register(
                RegisterChange {
                    b: Some(result),
                    ..RegisterChange::default()
                },
                result == 0
            )
        },
        0x31 => { //SWAP C
            let result = swap(cpu.registers.c);

            swap_register(
                RegisterChange {
                    c: Some(result),
                    ..RegisterChange::default()
                },
                result == 0
            )
        },
        0x32 => { //SWAP D
            let result = swap(cpu.registers.d);

            swap_register(
                RegisterChange {
                    d: Some(result),
                    ..RegisterChange::default()
                },
                result == 0
            )
        },
        0x33 => { //SWAP E
            let result = swap(cpu.registers.e);

            swap_register(
                RegisterChange {
                    e: Some(result),
                    ..RegisterChange::default()
                },
                result == 0
            )
        },
        0x34 => { //SWAP H
            let result = swap(cpu.registers.h);

            swap_register(
                RegisterChange {
                    h: Some(result),
                    ..RegisterChange::default()
                },
                result == 0
            )
        },
        0x35 => { //SWAP L
            let result = swap(cpu.registers.l);

            swap_register(
                RegisterChange {
                    l: Some(result),
                    ..RegisterChange::default()
                },
                result == 0
            )
        },
        0x36 => { //SWAP [HL]
            let result = swap(cpu.memory[cpu.registers.hl() as usize]);

            swap_absolute(
                MemoryChange {
                    changes: vec![
                        MemoryEdit {
                            key: cpu.registers.hl(),
                            value: result
                        }
                    ]
                },
                result == 0
            )
        },
        0x37 => { //SWAP A
            let result = swap(cpu.registers.a);

            swap_register(
                RegisterChange {
                    a: Some(result),
                    ..RegisterChange::default()
                },
                result == 0
            )
        },
        0x38 => { //SRL B
            let (result, set_carry) = shift_right_logically(
                cpu.registers.b
            );

            rotate_shift_register(
                RegisterChange {
                    b: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x39 => { //SRL C
            let (result, set_carry) = shift_right_logically(
                cpu.registers.c
            );

            rotate_shift_register(
                RegisterChange {
                    c: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x3A => { //SRL D
            let (result, set_carry) = shift_right_logically(
                cpu.registers.d
            );

            rotate_shift_register(
                RegisterChange {
                    d: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x3B => { //SRL E
            let (result, set_carry) = shift_right_logically(
                cpu.registers.e
            );

            rotate_shift_register(
                RegisterChange {
                    e: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x3C => { //SRL H
            let (result, set_carry) = shift_right_logically(
                cpu.registers.h
            );

            rotate_shift_register(
                RegisterChange {
                    h: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x3D => { //SRL L
            let (result, set_carry) = shift_right_logically(
                cpu.registers.l
            );

            rotate_shift_register(
                RegisterChange {
                    l: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        0x3E => { //SRL [HL]
            let (result, set_carry) = shift_right_logically(
                cpu.memory[cpu.registers.hl() as usize]
            );

            rotate_shift_absolute(
                MemoryChange {
                    changes: vec![
                        MemoryEdit {
                            key: cpu.registers.hl(),
                            value: result
                        }
                    ]
                },
                set_carry,
                result == 0
            )
        },
        0x3F => { //SRL A
            let (result, set_carry) = shift_right_logically(
                cpu.registers.a
            );

            rotate_shift_register(
                RegisterChange {
                    a: Some(result),
                    ..RegisterChange::default()
                },
                set_carry,
                result == 0
            )
        },
        //BIT
        0x40..=0x45 => bit_test_register(cpu, op_code, 0),
        0x46 => bit_test_absolute_hl(cpu, 0),
        0x47 => bit_test_register(cpu, op_code, 0),
        0x48..=0x4D => bit_test_register(cpu, op_code, 1),
        0x4E => bit_test_absolute_hl(cpu, 1),
        0x4F => bit_test_register(cpu, op_code, 1),
        0x50..=0x55 => bit_test_register(cpu, op_code, 2),
        0x56 => bit_test_absolute_hl(cpu, 2),
        0x57 => bit_test_register(cpu, op_code, 2),
        0x58..=0x5D => bit_test_register(cpu, op_code, 3),
        0x5E => bit_test_absolute_hl(cpu, 3),
        0x5F => bit_test_register(cpu, op_code, 3),
        0x60..=0x65 => bit_test_register(cpu, op_code, 4),
        0x66 => bit_test_absolute_hl(cpu, 4),
        0x67 => bit_test_register(cpu, op_code, 4),
        0x68..=0x6D => bit_test_register(cpu, op_code, 5),
        0x6E => bit_test_absolute_hl(cpu, 5),
        0x6F => bit_test_register(cpu, op_code, 5),
        0x70..=0x75 => bit_test_register(cpu, op_code, 6),
        0x76 => bit_test_absolute_hl(cpu, 6),
        0x77 => bit_test_register(cpu, op_code, 6),
        0x78..=0x7D => bit_test_register(cpu, op_code, 7),
        0x7E => bit_test_absolute_hl(cpu, 7),
        0x7F => bit_test_register(cpu, op_code, 7),
        //RES
        0x80..=0x85 => reset_bit_register(cpu, op_code, 0),
        0x86 => reset_bit_absolute_hl(cpu, 0),
        0x87 => reset_bit_register(cpu, op_code, 0),
        0x88..=0x8D => reset_bit_register(cpu, op_code, 1),
        0x8E => reset_bit_absolute_hl(cpu, 1),
        0x8F => reset_bit_register(cpu, op_code, 1),
        0x90..=0x95 => reset_bit_register(cpu, op_code, 2),
        0x96 => reset_bit_absolute_hl(cpu, 2),
        0x97 => reset_bit_register(cpu, op_code, 2),
        0x98..=0x9D => reset_bit_register(cpu, op_code, 3),
        0x9E => reset_bit_absolute_hl(cpu, 3),
        0x9F => reset_bit_register(cpu, op_code, 3),
        0xA0..=0xA5 => reset_bit_register(cpu, op_code, 4),
        0xA6 => reset_bit_absolute_hl(cpu, 4),
        0xA7 => reset_bit_register(cpu, op_code, 4),
        0xA8..=0xAD => reset_bit_register(cpu, op_code, 5),
        0xAE => reset_bit_absolute_hl(cpu, 5),
        0xAF => reset_bit_register(cpu, op_code, 5),
        0xB0..=0xB5 => reset_bit_register(cpu, op_code, 6),
        0xB6 => reset_bit_absolute_hl(cpu, 6),
        0xB7 => reset_bit_register(cpu, op_code, 6),
        0xB8..=0xBD => reset_bit_register(cpu, op_code, 7),
        0xBE => reset_bit_absolute_hl(cpu, 7),
        0xBF => reset_bit_register(cpu, op_code, 7),
        //SET
        0xC0..=0xC5 => set_bit_register(cpu, op_code, 0),
        0xC6 => set_bit_absolute_hl(cpu, 0),
        0xC7 => set_bit_register(cpu, op_code, 0),
        0xC8..=0xCD => set_bit_register(cpu, op_code, 1),
        0xCE => set_bit_absolute_hl(cpu, 1),
        0xCF => set_bit_register(cpu, op_code, 1),
        0xD0..=0xD5 => set_bit_register(cpu, op_code, 2),
        0xD6 => set_bit_absolute_hl(cpu, 2),
        0xD7 => set_bit_register(cpu, op_code, 2),
        0xD8..=0xDD => set_bit_register(cpu, op_code, 3),
        0xDE => set_bit_absolute_hl(cpu, 3),
        0xDF => set_bit_register(cpu, op_code, 3),
        0xE0..=0xE5 => set_bit_register(cpu, op_code, 4),
        0xE6 => set_bit_absolute_hl(cpu, 4),
        0xE7 => set_bit_register(cpu, op_code, 4),
        0xE8..=0xED => set_bit_register(cpu, op_code, 5),
        0xEE => set_bit_absolute_hl(cpu, 5),
        0xEF => set_bit_register(cpu, op_code, 5),
        0xF0..=0xF5 => set_bit_register(cpu, op_code, 6),
        0xF6 => set_bit_absolute_hl(cpu, 6),
        0xF7 => set_bit_register(cpu, op_code, 6),
        0xF8..=0xFD => set_bit_register(cpu, op_code, 7),
        0xFE => set_bit_absolute_hl(cpu, 7),
        0xFF => set_bit_register(cpu, op_code, 7)
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

//same as a logical left shift
fn shift_left_arithmetically(value: u8) -> (u8, bool) {
    let set_carry = (value & 0x80) == 0x80;
    let value = value << 1;

    (value, set_carry)
}

//MSB is shifted and the new MSB is set to the pre-shift MSB value
fn shift_right_arithmetically(value: u8) -> (u8, bool) {
    let set_carry = (value & 0x01) == 0x01;
    let value = (value >> 1) + (value & 0x80);

    (value, set_carry)
}

fn shift_right_logically(value: u8) -> (u8, bool) {
    let set_carry = (value & 0x01) == 0x01;
    let value = value >> 1;

    (value, set_carry)
}

//swap upper 4 bits and lower 4 bits
fn swap(value: u8) -> u8 {
    (value << 4) ^ (value >> 4)
}

fn set_bit_register(cpu: &CPU, op_code: u8, bit_index: u8) -> StateChange {
    let test = BINARY_BASE.pow(bit_index as u32);
    let mut value = cpu.registers.from_opcode_index(op_code);

    if (value & test) != test { //the bit is not set, set it
        value += test;
    }

    StateChange {
        t_states: 8,
        ime: None,
        flags: FlagChange::default(),
        register: RegisterChange::create_from_opcode(
            op_code,
            Some(value)
        ),
        memory: MemoryChange::default()
    }
}

fn set_bit_absolute_hl(cpu: &CPU, bit_index: u8) -> StateChange {
    let test = BINARY_BASE.pow(bit_index as u32);
    let mut value = cpu.memory[cpu.registers.hl() as usize];

    if (value & test) != test { //the bit is not set, set it
        value += test;
    }

    StateChange {
        t_states: 16,
        ime: None,
        flags: FlagChange::default(),
        register: RegisterChange::default(),
        memory: MemoryChange {
            changes: vec![
                MemoryEdit {
                    key: cpu.registers.hl(),
                    value
                }
            ]
        }
    }
}

fn reset_bit_register(cpu: &CPU, op_code: u8, bit_index: u8) -> StateChange {
    let test = BINARY_BASE.pow(bit_index as u32);
    let mut value = cpu.registers.from_opcode_index(op_code);

    if (value & test) == test { //the bit is set, reset it
        value -= test;
    }

    StateChange {
        t_states: 8,
        ime: None,
        flags: FlagChange::default(),
        register: RegisterChange::create_from_opcode(
            op_code,
            Some(value)
        ),
        memory: MemoryChange::default()
    }
}

fn reset_bit_absolute_hl(cpu: &CPU, bit_index: u8) -> StateChange {
    let test = BINARY_BASE.pow(bit_index as u32);
    let mut value = cpu.memory[cpu.registers.hl() as usize];

    if (value & test) == test { //the bit is set, reset it
        value -= test;
    }

    StateChange {
        t_states: 16,
        ime: None,
        flags: FlagChange::default(),
        register: RegisterChange::default(),
        memory: MemoryChange {
            changes: vec![
                MemoryEdit {
                    key: cpu.registers.hl(),
                    value
                }
            ]
        }
    }
}

fn bit_test_register(cpu: &CPU, op_code: u8, bit_index: u8) -> StateChange {
    let test = BINARY_BASE.pow(bit_index as u32);
    let value = cpu.registers.from_opcode_index(op_code);
    let set_zero = value & test != test;

    StateChange {
        t_states: 8,
        ime: None,
        flags: FlagChange {
            zero: Some(set_zero),
            subtract: Some(false),
            half_carry: Some(true),
            carry: None
        },
        register: RegisterChange::default(),
        memory: MemoryChange::default()
    }
}

fn bit_test_absolute_hl(cpu: &CPU, bit_index: u8) -> StateChange {
    let test = BINARY_BASE.pow(bit_index as u32);
    let value = cpu.memory[cpu.registers.hl() as usize];
    let set_zero = value & test != test;

    StateChange {
        t_states: 12,
        ime: None,
        flags: FlagChange {
            zero: Some(set_zero),
            subtract: Some(false),
            half_carry: Some(true),
            carry: None
        },
        register: RegisterChange::default(),
        memory: MemoryChange::default()
    }
}

fn swap_absolute(change: MemoryChange, set_zero: bool) -> StateChange {
    StateChange {
        t_states: 16,
        ime: Option::None,
        flags: FlagChange {
            zero: Some(set_zero),
            ..FlagChange::reset()
        },
        register: RegisterChange::default(),
        memory: change
    }
}

fn swap_register(change: RegisterChange, set_zero: bool) -> StateChange {
    StateChange {
        t_states: 8,
        ime: Option::None,
        flags: FlagChange {
            zero: Some(set_zero),
            ..FlagChange::reset()
        },
        register: change,
        memory: MemoryChange::default()
    }
}

fn rotate_shift_absolute(change: MemoryChange, set_carry: bool, set_zero: bool) -> StateChange {
    StateChange {
        t_states: 16,
        ime: Option::None,
        flags: FlagChange {
            carry: Some(set_carry),
            zero: Some(set_zero),
            ..FlagChange::reset()
        },
        register: RegisterChange::default(),
        memory: change
    }
}

fn rotate_shift_register(change: RegisterChange, set_carry: bool, set_zero: bool) -> StateChange {
    StateChange {
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