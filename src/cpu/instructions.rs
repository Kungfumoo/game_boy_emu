use super::{
    CPU,
    registers::{to8_bit, to16_bit, RegisterChange},
    flags::{
        FlagChange,
        is_half_carry_add, is_half_carry_subtract,
        is_carry_add_16, is_half_carry_add_16,
        is_carry_add, is_carry_subtract
    },
    memory::{MemoryChange, MemoryEdit},
    util::{
        add16_bit, sub16_bit,
        add8_bit, sub8_bit
    }
};

pub struct StateChange {
    pub byte_length: i16,
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
            cpu.registers.hl(),
            cpu.registers.bc()
        ),
        0x0A => ld_from_absolute( //LD A, [BC]
            RegisterChange {
                a: Some(cpu.memory[cpu.registers.bc() as usize]),
                ..RegisterChange::default()
            }
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
        0x0F => { //RRCA (rotate register A right)
            let a = cpu.registers.a.rotate_right(1);

            rotate_register(
                RegisterChange {
                    a: Some(a),
                    ..RegisterChange::default()
                },
                (a & 0x80) == 0x80 //check leftmost bit
            )
        },
        0x10 => StateChange { //STOP - TODO: something about switching between power modes on GBC cpu
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
        0x17 => { //RLA (rotate register A left through the carry)
            //carry works effectively as the 1st bit on the right so need to emulate this...
            let mut a = (cpu.registers.a as u16) << 1; //convert to 16bit and shift to left by 1

            if cpu.flags.carry {
                a += 1; //+1 will set the new rightmost bit
            }

            //bit 9 of the 16bit int will be the new carry
            let set_carry = (a & 0x100) == 0x100;

            if set_carry {
                a -= 0x100; //unset bit 9 so we can cast back to 8 bits
            }

            rotate_register(
                RegisterChange {
                    a: Some(a as u8),
                    ..RegisterChange::default()
                },
                set_carry
            )
        },
        0x18 => { //JR e8
            let pc = cpu.registers.program_counter;

            #[allow(overflowing_literals)]
            let modifier = cpu.memory[(pc + 1) as usize] as i8;

            relative_jmp(modifier)
        },
        0x19 => add_to_hl( //ADD HL, DE
            cpu.registers.hl(),
            cpu.registers.de()
        ),
        0x1A => ld_from_absolute( //LD A, [DE]
            RegisterChange {
                a: Some(cpu.memory[cpu.registers.de() as usize]),
                ..RegisterChange::default()
            }
        ),
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
        0x1F => { //RRA (rotate register A right through the carry)
            let mut a = cpu.registers.a;
            let set_carry = (a & 0x1) == 0x1; //if rightmost is 1 then it will carry

            a = a >> 1;

            if cpu.flags.carry {
                a += 0x80; //0x80 will set the new leftmost bit
            }

            rotate_register(
                RegisterChange {
                    a: Some(a),
                    ..RegisterChange::default()
                },
                set_carry
            )
        },
        0x20 => { //JR NZ, e8
            if cpu.flags.zero {
                return no_relative_jmp();
            }

            let pc = cpu.registers.program_counter;

            #[allow(overflowing_literals)]
            let modifier = cpu.memory[(pc + 1) as usize] as i8;

            relative_jmp(modifier)
        },
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
        0x27 => { //DAA
            //https://forums.nesdev.org/viewtopic.php?p=196282#p196282
            let mut a = cpu.registers.a;
            let mut set_carry = false;

            if !cpu.flags.subtract { //addition
                if cpu.flags.carry || a > 0x99 {
                    a += 0x60;
                    set_carry = true;
                }

                if cpu.flags.half_carry || (a & 0x0F) > 0x09 {
                    a += 0x06;
                }
            } else { //subtraction
                if cpu.flags.carry {
                    a -= 0x60;
                }

                if cpu.flags.half_carry {
                    a -= 0x06;
                }
            }

            StateChange {
                byte_length: 1,
                t_states: 4,
                memory: MemoryChange::default(),
                flags: FlagChange {
                    carry: Some(set_carry),
                    half_carry: Some(false),
                    zero: Some(a == 0),
                    ..FlagChange::default()
                },
                register: RegisterChange {
                    a: Some(a),
                    ..RegisterChange::default()
                }
            }
        },
        0x28 => { //JR Z, e8
            if !cpu.flags.zero {
                return no_relative_jmp();
            }

            let pc = cpu.registers.program_counter;

            #[allow(overflowing_literals)]
            let modifier = cpu.memory[(pc + 1) as usize] as i8;

            relative_jmp(modifier)
        },
        0x29 => add_to_hl( //ADD HL, HL
            cpu.registers.hl(),
            cpu.registers.hl()
        ),
        0x2A => { //LD A, (HL+)
            let (h, l) = to8_bit(add16_bit(cpu.registers.hl(), 1));

            ld_from_absolute(RegisterChange {
                a: Some(cpu.memory[cpu.registers.hl() as usize]),
                h: Some(h),
                l: Some(l),
                ..RegisterChange::default()
            })
        },
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
        0x2F => StateChange { //CPL
            byte_length: 1,
            t_states: 4,
            flags: FlagChange {
                subtract: Some(true),
                half_carry: Some(true),
                ..FlagChange::default()
            },
            register: RegisterChange {
                a: Some(!cpu.registers.a),
                ..RegisterChange::default()
            },
            memory: MemoryChange::default()
        },
        0x30 => { //JR NC, e8
            if cpu.flags.carry {
                return no_relative_jmp();
            }

            let pc = cpu.registers.program_counter;

            #[allow(overflowing_literals)]
            let modifier = cpu.memory[(pc + 1) as usize] as i8;

            relative_jmp(modifier)
        },
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
        0x37 => StateChange { //SCF (Set Carry Flag)
            byte_length: 1,
            t_states: 4,
            flags: FlagChange {
                subtract: Some(false),
                half_carry: Some(false),
                carry: Some(true),
                ..FlagChange::default()
            },
            register: RegisterChange::default(),
            memory: MemoryChange::default()
        },
        0x38 => { //JR C, e8
            if !cpu.flags.carry {
                return no_relative_jmp();
            }

            let pc = cpu.registers.program_counter;

            #[allow(overflowing_literals)]
            let modifier = cpu.memory[(pc + 1) as usize] as i8;

            relative_jmp(modifier)
        },
        0x39 => add_to_hl( //ADD HL, SP
            cpu.registers.hl(),
            cpu.registers.stack_pointer
        ),
        0x3A => { //LD A, (HL-)
            let (h, l) = to8_bit(sub16_bit(cpu.registers.hl(), 1));

            ld_from_absolute(RegisterChange {
                a: Some(cpu.memory[cpu.registers.hl() as usize]),
                h: Some(h),
                l: Some(l),
                ..RegisterChange::default()
            })
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
        0x3F => StateChange { //CCF (Complement/Invert Carry Flag)
            byte_length: 1,
            t_states: 4,
            flags: FlagChange {
                subtract: Some(false),
                half_carry: Some(false),
                carry: Some(!cpu.flags.carry),
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
        0x46 => ld_from_absolute(RegisterChange { //LD B, [HL]
            b: Some(cpu.memory[cpu.registers.hl() as usize]),
            ..RegisterChange::default()
        }),
        0x47 => ld_register_to_register(RegisterChange { //LD B, A
            b: Option::Some(cpu.registers.a),
            ..RegisterChange::default()
        }),
        0x48 => ld_register_to_register(RegisterChange { //LD C, B
            c: Option::Some(cpu.registers.b),
            ..RegisterChange::default()
        }),
        0x49 => ld_register_to_register(RegisterChange { //LD C, C
            c: Option::Some(cpu.registers.c),
            ..RegisterChange::default()
        }),
        0x4A => ld_register_to_register(RegisterChange { //LD C, D
            c: Option::Some(cpu.registers.d),
            ..RegisterChange::default()
        }),
        0x4B => ld_register_to_register(RegisterChange { //LD C, E
            c: Option::Some(cpu.registers.e),
            ..RegisterChange::default()
        }),
        0x4C => ld_register_to_register(RegisterChange { //LD C, H
            c: Option::Some(cpu.registers.h),
            ..RegisterChange::default()
        }),
        0x4D => ld_register_to_register(RegisterChange { //LD C, L
            c: Option::Some(cpu.registers.l),
            ..RegisterChange::default()
        }),
        0x4E => ld_from_absolute(RegisterChange { //LD C, [HL]
            c: Some(cpu.memory[cpu.registers.hl() as usize]),
            ..RegisterChange::default()
        }),
        0x4F => ld_register_to_register(RegisterChange { //LD C, A
            c: Option::Some(cpu.registers.a),
            ..RegisterChange::default()
        }),
        0x50 => ld_register_to_register(RegisterChange { //LD D, B
            d: Option::Some(cpu.registers.b),
            ..RegisterChange::default()
        }),
        0x51 => ld_register_to_register(RegisterChange { //LD D, C
            d: Option::Some(cpu.registers.c),
            ..RegisterChange::default()
        }),
        0x52 => ld_register_to_register(RegisterChange { //LD D, D
            d: Option::Some(cpu.registers.d),
            ..RegisterChange::default()
        }),
        0x53 => ld_register_to_register(RegisterChange { //LD D, E
            d: Option::Some(cpu.registers.e),
            ..RegisterChange::default()
        }),
        0x54 => ld_register_to_register(RegisterChange { //LD D, H
            d: Option::Some(cpu.registers.h),
            ..RegisterChange::default()
        }),
        0x55 => ld_register_to_register(RegisterChange { //LD D, L
            d: Option::Some(cpu.registers.l),
            ..RegisterChange::default()
        }),
        0x56 => ld_from_absolute(RegisterChange { //LD D, [HL]
            d: Some(cpu.memory[cpu.registers.hl() as usize]),
            ..RegisterChange::default()
        }),
        0x57 => ld_register_to_register(RegisterChange { //LD D, A
            d: Option::Some(cpu.registers.a),
            ..RegisterChange::default()
        }),
        0x58 => ld_register_to_register(RegisterChange { //LD E, B
            e: Option::Some(cpu.registers.b),
            ..RegisterChange::default()
        }),
        0x59 => ld_register_to_register(RegisterChange { //LD E, C
            e: Option::Some(cpu.registers.c),
            ..RegisterChange::default()
        }),
        0x5A => ld_register_to_register(RegisterChange { //LD E, D
            e: Option::Some(cpu.registers.d),
            ..RegisterChange::default()
        }),
        0x5B => ld_register_to_register(RegisterChange { //LD E, E
            e: Option::Some(cpu.registers.e),
            ..RegisterChange::default()
        }),
        0x5C => ld_register_to_register(RegisterChange { //LD E, H
            e: Option::Some(cpu.registers.h),
            ..RegisterChange::default()
        }),
        0x5D => ld_register_to_register(RegisterChange { //LD E, L
            e: Option::Some(cpu.registers.l),
            ..RegisterChange::default()
        }),
        0x5E => ld_from_absolute(RegisterChange { //LD E, [HL]
            e: Some(cpu.memory[cpu.registers.hl() as usize]),
            ..RegisterChange::default()
        }),
        0x5F => ld_register_to_register(RegisterChange { //LD E, A
            e: Option::Some(cpu.registers.a),
            ..RegisterChange::default()
        }),
        0x60 => ld_register_to_register(RegisterChange { //LD H, B
            h: Option::Some(cpu.registers.b),
            ..RegisterChange::default()
        }),
        0x61 => ld_register_to_register(RegisterChange { //LD H, C
            h: Option::Some(cpu.registers.c),
            ..RegisterChange::default()
        }),
        0x62 => ld_register_to_register(RegisterChange { //LD H, D
            h: Option::Some(cpu.registers.d),
            ..RegisterChange::default()
        }),
        0x63 => ld_register_to_register(RegisterChange { //LD H, E
            h: Option::Some(cpu.registers.e),
            ..RegisterChange::default()
        }),
        0x64 => ld_register_to_register(RegisterChange { //LD H, H
            h: Option::Some(cpu.registers.h),
            ..RegisterChange::default()
        }),
        0x65 => ld_register_to_register(RegisterChange { //LD H, L
            h: Option::Some(cpu.registers.l),
            ..RegisterChange::default()
        }),
        0x66 => ld_from_absolute(RegisterChange { //LD H, [HL]
            h: Some(cpu.memory[cpu.registers.hl() as usize]),
            ..RegisterChange::default()
        }),
        0x67 => ld_register_to_register(RegisterChange { //LD H, A
            h: Option::Some(cpu.registers.a),
            ..RegisterChange::default()
        }),
        0x68 => ld_register_to_register(RegisterChange { //LD L, B
            l: Option::Some(cpu.registers.b),
            ..RegisterChange::default()
        }),
        0x69 => ld_register_to_register(RegisterChange { //LD L, C
            l: Option::Some(cpu.registers.c),
            ..RegisterChange::default()
        }),
        0x6A => ld_register_to_register(RegisterChange { //LD L, D
            l: Option::Some(cpu.registers.d),
            ..RegisterChange::default()
        }),
        0x6B => ld_register_to_register(RegisterChange { //LD L, E
            l: Option::Some(cpu.registers.e),
            ..RegisterChange::default()
        }),
        0x6C => ld_register_to_register(RegisterChange { //LD L, H
            l: Option::Some(cpu.registers.h),
            ..RegisterChange::default()
        }),
        0x6D => ld_register_to_register(RegisterChange { //LD L, L
            l: Option::Some(cpu.registers.l),
            ..RegisterChange::default()
        }),
        0x6E => ld_from_absolute(RegisterChange { //LD L, [HL]
            l: Some(cpu.memory[cpu.registers.hl() as usize]),
            ..RegisterChange::default()
        }),
        0x6F => ld_register_to_register(RegisterChange { //LD L, A
            l: Option::Some(cpu.registers.a),
            ..RegisterChange::default()
        }),
        0x70 => ld_to_absolute(MemoryChange { //LD [HL], B
            changes: vec![
                MemoryEdit {
                    key: cpu.registers.hl(),
                    value: cpu.registers.b
                }
            ]
        }),
        0x71 => ld_to_absolute(MemoryChange { //LD [HL], C
            changes: vec![
                MemoryEdit {
                    key: cpu.registers.hl(),
                    value: cpu.registers.c
                }
            ]
        }),
        0x72 => ld_to_absolute(MemoryChange { //LD [HL], D
            changes: vec![
                MemoryEdit {
                    key: cpu.registers.hl(),
                    value: cpu.registers.d
                }
            ]
        }),
        0x73 => ld_to_absolute(MemoryChange { //LD [HL], E
            changes: vec![
                MemoryEdit {
                    key: cpu.registers.hl(),
                    value: cpu.registers.e
                }
            ]
        }),
        0x74 => ld_to_absolute(MemoryChange { //LD [HL], H
            changes: vec![
                MemoryEdit {
                    key: cpu.registers.hl(),
                    value: cpu.registers.h
                }
            ]
        }),
        0x75 => ld_to_absolute(MemoryChange { //LD [HL], L
            changes: vec![
                MemoryEdit {
                    key: cpu.registers.hl(),
                    value: cpu.registers.l
                }
            ]
        }),
        0x76 => nop(), //HALT //TODO: is this the same as nop?
        0x77 => ld_to_absolute(MemoryChange { //LD [HL], A
            changes: vec![
                MemoryEdit {
                    key: cpu.registers.hl(),
                    value: cpu.registers.a
                }
            ]
        }),
        0x78 => ld_register_to_register(RegisterChange { //LD A, B
            a: Option::Some(cpu.registers.b),
            ..RegisterChange::default()
        }),
        0x79 => ld_register_to_register(RegisterChange { //LD A, C
            a: Option::Some(cpu.registers.c),
            ..RegisterChange::default()
        }),
        0x7A => ld_register_to_register(RegisterChange { //LD A, D
            a: Option::Some(cpu.registers.d),
            ..RegisterChange::default()
        }),
        0x7B => ld_register_to_register(RegisterChange { //LD A, E
            a: Option::Some(cpu.registers.e),
            ..RegisterChange::default()
        }),
        0x7C => ld_register_to_register(RegisterChange { //LD A, H
            a: Option::Some(cpu.registers.h),
            ..RegisterChange::default()
        }),
        0x7D => ld_register_to_register(RegisterChange { //LD A, L
            a: Option::Some(cpu.registers.l),
            ..RegisterChange::default()
        }),
        0x7E => ld_from_absolute(RegisterChange { //LD A, [HL]
            a: Some(cpu.memory[cpu.registers.hl() as usize]),
            ..RegisterChange::default()
        }),
        0x7F => ld_register_to_register(RegisterChange { //LD A, A
            a: Option::Some(cpu.registers.a),
            ..RegisterChange::default()
        }),
        0x80 => add_to_a( //ADD A, B
            cpu.registers.a,
            cpu.registers.b
        ),
        0x81 => add_to_a( //ADD A, C
            cpu.registers.a,
            cpu.registers.c
        ),
        0x82 => add_to_a( //ADD A, D
            cpu.registers.a,
            cpu.registers.d
        ),
        0x83 => add_to_a( //ADD A, E
            cpu.registers.a,
            cpu.registers.e
        ),
        0x84 => add_to_a( //ADD A, H
            cpu.registers.a,
            cpu.registers.h
        ),
        0x85 => add_to_a( //ADD A, L
            cpu.registers.a,
            cpu.registers.l
        ),
        0x86 => StateChange { //ADD A, [HL]
            t_states: 8,
            ..add_to_a(
                cpu.registers.a,
                cpu.memory[cpu.registers.hl() as usize]
            )
        },
        0x87 => add_to_a( //ADD A, A
            cpu.registers.a,
            cpu.registers.a
        ),
        0x88 => { //ADC A, B
            let mut operand = cpu.registers.b;

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            add_to_a(cpu.registers.a, operand)
        },
        0x89 => { //ADC A, C
            let mut operand = cpu.registers.c;

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            add_to_a(cpu.registers.a, operand)
        },
        0x8A => { //ADC A, D
            let mut operand = cpu.registers.d;

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            add_to_a(cpu.registers.a, operand)
        },
        0x8B => { //ADC A, E
            let mut operand = cpu.registers.e;

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            add_to_a(cpu.registers.a, operand)
        },
        0x8C => { //ADC A, H
            let mut operand = cpu.registers.h;

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            add_to_a(cpu.registers.a, operand)
        },
        0x8D => { //ADC A, L
            let mut operand = cpu.registers.l;

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            add_to_a(cpu.registers.a, operand)
        },
        0x8E => { //ADC A, [HL]
            let mut operand = cpu.memory[cpu.registers.hl() as usize];

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            StateChange {
                t_states: 8,
                ..add_to_a(
                    cpu.registers.a,
                    operand
                )
            }
        },
        0x8F => { //ADC A, A
            let mut operand = cpu.registers.a;

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            add_to_a(cpu.registers.a, operand)
        },
        0x90 => sub_from_a( //SUB A, B
            cpu.registers.a,
            cpu.registers.b
        ),
        0x91 => sub_from_a( //SUB A, C
            cpu.registers.a,
            cpu.registers.c
        ),
        0x92 => sub_from_a( //SUB A, D
            cpu.registers.a,
            cpu.registers.d
        ),
        0x93 => sub_from_a( //SUB A, E
            cpu.registers.a,
            cpu.registers.e
        ),
        0x94 => sub_from_a( //SUB A, H
            cpu.registers.a,
            cpu.registers.h
        ),
        0x95 => sub_from_a( //SUB A, L
            cpu.registers.a,
            cpu.registers.l
        ),
        0x96 => StateChange { //SUB A, [HL]
            t_states: 8,
            ..sub_from_a(
                cpu.registers.a,
                cpu.memory[cpu.registers.hl() as usize]
            )
        },
        0x97 => sub_from_a( //SUB A, A
            cpu.registers.a,
            cpu.registers.a
        ),
        0x98 => { //SBC A, B
            let mut operand = cpu.registers.b;

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            sub_from_a(cpu.registers.a, operand)
        },
        0x99 => { //SBC A, C
            let mut operand = cpu.registers.c;

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            sub_from_a(cpu.registers.a, operand)
        },
        0x9A => { //SBC A, D
            let mut operand = cpu.registers.d;

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            sub_from_a(cpu.registers.a, operand)
        },
        0x9B => { //SBC A, E
            let mut operand = cpu.registers.e;

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            sub_from_a(cpu.registers.a, operand)
        },
        0x9C => { //SBC A, H
            let mut operand = cpu.registers.h;

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            sub_from_a(cpu.registers.a, operand)
        },
        0x9D => { //SBC A, L
            let mut operand = cpu.registers.l;

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            sub_from_a(cpu.registers.a, operand)
        },
        0x9E => { //SBC A, [HL]
            let mut operand = cpu.memory[cpu.registers.hl() as usize];

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            StateChange {
                t_states: 8,
                ..sub_from_a(
                    cpu.registers.a,
                    operand
                )
            }
        },
        0x9F => { //SBC A, A
            let mut operand = cpu.registers.a;

            if cpu.flags.carry {
                operand = add8_bit(operand, 1);
            }

            sub_from_a(cpu.registers.a, operand)
        },
        0xA0 => and_to_a( //AND A, B
            cpu.registers.a,
            cpu.registers.b
        ),
        0xA1 => and_to_a( //AND A, C
            cpu.registers.a,
            cpu.registers.c
        ),
        0xA2 => and_to_a( //AND A, D
            cpu.registers.a,
            cpu.registers.d
        ),
        0xA3 => and_to_a( //AND A, E
            cpu.registers.a,
            cpu.registers.e
        ),
        0xA4 => and_to_a( //AND A, H
            cpu.registers.a,
            cpu.registers.h
        ),
        0xA5 => and_to_a( //AND A, L
            cpu.registers.a,
            cpu.registers.l
        ),
        0xA6 => StateChange { //AND A, [HL]
            t_states: 8,
            ..and_to_a(
                cpu.registers.a,
                cpu.memory[cpu.registers.hl() as usize]
            )
        },
        0xA7 => and_to_a( //AND A, A
            cpu.registers.a,
            cpu.registers.a
        ),
        0xA8 => xor_to_a( //XOR A, B
            cpu.registers.a,
            cpu.registers.b
        ),
        0xA9 => xor_to_a( //XOR A, C
            cpu.registers.a,
            cpu.registers.c
        ),
        0xAA => xor_to_a( //XOR A, D
            cpu.registers.a,
            cpu.registers.d
        ),
        0xAB => xor_to_a( //XOR A, E
            cpu.registers.a,
            cpu.registers.e
        ),
        0xAC => xor_to_a( //XOR A, H
            cpu.registers.a,
            cpu.registers.h
        ),
        0xAD => xor_to_a( //XOR A, L
            cpu.registers.a,
            cpu.registers.l
        ),
        0xAE => StateChange { //XOR A, [HL]
            t_states: 8,
            ..xor_to_a(
                cpu.registers.a,
                cpu.memory[cpu.registers.hl() as usize]
            )
        },
        0xAF => xor_to_a( //XOR A, A
            cpu.registers.a,
            cpu.registers.a
        ),
        0xB0 => or_to_a( //OR A, B
            cpu.registers.a,
            cpu.registers.b
        ),
        0xB1 => or_to_a( //OR A, C
            cpu.registers.a,
            cpu.registers.c
        ),
        0xB2 => or_to_a( //OR A, D
            cpu.registers.a,
            cpu.registers.d
        ),
        0xB3 => or_to_a( //OR A, E
            cpu.registers.a,
            cpu.registers.e
        ),
        0xB4 => or_to_a( //OR A, H
            cpu.registers.a,
            cpu.registers.h
        ),
        0xB5 => or_to_a( //OR A, L
            cpu.registers.a,
            cpu.registers.l
        ),
        0xB6 => StateChange { //OR A, [HL]
            t_states: 8,
            ..or_to_a(
                cpu.registers.a,
                cpu.memory[cpu.registers.hl() as usize]
            )
        },
        0xB7 => or_to_a( //OR A, A
            cpu.registers.a,
            cpu.registers.a
        ),
        0xB8 => cp_to_a( //CP A, B
            cpu.registers.a,
            cpu.registers.b
        ),
        0xB9 => cp_to_a( //CP A, C
            cpu.registers.a,
            cpu.registers.c
        ),
        0xBA => cp_to_a( //CP A, D
            cpu.registers.a,
            cpu.registers.d
        ),
        0xBB => cp_to_a( //CP A, E
            cpu.registers.a,
            cpu.registers.e
        ),
        0xBC => cp_to_a( //CP A, H
            cpu.registers.a,
            cpu.registers.h
        ),
        0xBD => cp_to_a( //CP A, L
            cpu.registers.a,
            cpu.registers.l
        ),
        0xBE => StateChange { //CP A, [HL]
            t_states: 8,
            ..cp_to_a(
                cpu.registers.a,
                cpu.memory[cpu.registers.hl() as usize]
            )
        },
        0xBF => cp_to_a( //CP A, A
            cpu.registers.a,
            cpu.registers.a
        ),
        0xC0 => { //RET NZ
            if cpu.flags.zero {
                return no_ret();
            }

            StateChange {
                t_states: 20,
                ..ret(cpu)
            }
        },
        0xC3 => { //JP a16
            let addr = to16_bit(
                cpu.memory[(cpu.registers.program_counter + 1) as usize],
                cpu.memory[(cpu.registers.program_counter + 2) as usize]
            );

            absolute_jmp(addr)
        },
        0xC8 => { //RET Z
            if !cpu.flags.zero {
                return no_ret();
            }

            StateChange {
                t_states: 20,
                ..ret(cpu)
            }
        },
        0xC9 => ret(cpu), //RET
        0xCD => { //CALL a16
            let addr = to16_bit(
                cpu.memory[(cpu.registers.program_counter + 1) as usize],
                cpu.memory[(cpu.registers.program_counter + 2) as usize]
            );

            call(cpu, addr)
        },
        0xD0 => { //RET NC
            if cpu.flags.carry {
                return no_ret();
            }

            StateChange {
                t_states: 20,
                ..ret(cpu)
            }
        },
        0xD8 => { //RET C
            if !cpu.flags.carry {
                return no_ret();
            }

            StateChange {
                t_states: 20,
                ..ret(cpu)
            }
        },
        _ => StateChange {
            byte_length: 0,
            t_states: 0,
            flags: FlagChange::default(),
            register: RegisterChange::default(),
            memory: MemoryChange::default()
        }
    }
}

fn no_ret() -> StateChange {
    StateChange {
        byte_length: 1,
        t_states: 8,
        flags: FlagChange::default(),
        memory: MemoryChange::default(),
        register: RegisterChange::default()
    }
}

//return from subroutine. JP back to the addr that is in the stack
fn ret(cpu: &CPU) -> StateChange {
    let lsb = cpu.memory[cpu.registers.stack_pointer as usize];
    let msb = cpu.memory[(cpu.registers.stack_pointer + 1) as usize];
    let new_addr = to16_bit(msb, lsb);

    StateChange {
        byte_length: 1,
        t_states: 16,
        flags: FlagChange::default(),
        register: RegisterChange {
            pc: Some(new_addr),
            sp: Some(cpu.registers.stack_pointer + 2),
            ..RegisterChange::default()
        },
        memory: MemoryChange::default()
    }
}

//calls a subroutine. JP to the new addr and pushes the old address to the stack
fn call(cpu: &CPU, new_addr: u16) -> StateChange {
    let current_address = cpu.registers.program_counter + 3;
    let (msb, lsb) = to8_bit(current_address);

    StateChange {
        byte_length: 3,
        t_states: 24,
        flags: FlagChange::default(),
        register: RegisterChange {
            pc: Some(new_addr),
            sp: Some(cpu.registers.stack_pointer - 2),
            ..RegisterChange::default()
        },
        memory: MemoryChange {
            changes: vec![
                MemoryEdit {
                    key: cpu.registers.stack_pointer - 1,
                    value: msb
                },
                MemoryEdit {
                    key: cpu.registers.stack_pointer - 2,
                    value: lsb
                }
            ]
        }
    }
}

fn absolute_jmp(address: u16) -> StateChange {
    StateChange {
        byte_length: 3,
        t_states: 16,
        register: RegisterChange {
            pc: Some(address),
            ..RegisterChange::default()
        },
        flags: FlagChange::default(),
        memory: MemoryChange::default()
    }
}

fn no_absolute_jmp() -> StateChange {
    StateChange {
        byte_length: 3,
        t_states: 12,
        flags: FlagChange::default(),
        register: RegisterChange::default(),
        memory: MemoryChange::default()
    }
}

fn no_relative_jmp() -> StateChange {
    StateChange {
        byte_length: 2,
        t_states: 8,
        flags: FlagChange::default(),
        register: RegisterChange::default(),
        memory: MemoryChange::default()
    }
}

fn relative_jmp(modifier: i8) -> StateChange {
    StateChange {
        byte_length: (2 + modifier).into(),
        t_states: 12,
        flags: FlagChange::default(),
        register: RegisterChange::default(),
        memory: MemoryChange::default()
    }
}

fn cp_to_a(a_value: u8, operand: u8) -> StateChange {
    let new_value = sub8_bit(a_value, operand);

    StateChange {
        byte_length: 1,
        t_states: 4,
        flags: FlagChange {
            subtract: Some(true),
            carry: Some(is_carry_subtract(a_value, operand)),
            half_carry: Some(is_half_carry_subtract(a_value, operand)),
            zero: Some(new_value == 0)
        },
        register: RegisterChange::default(),
        memory: MemoryChange::default()
    }
}

fn or_to_a(a_value: u8, operand: u8) -> StateChange {
    let new_value = a_value | operand;

    StateChange {
        byte_length: 1,
        t_states: 4,
        flags: FlagChange {
            subtract: Some(false),
            carry: Some(false),
            half_carry: Some(false),
            zero: Some(new_value == 0)
        },
        register: RegisterChange {
            a: Some(new_value),
            ..RegisterChange::default()
        },
        memory: MemoryChange::default()
    }
}

fn xor_to_a(a_value: u8, operand: u8) -> StateChange {
    let new_value = a_value ^ operand;

    StateChange {
        byte_length: 1,
        t_states: 4,
        flags: FlagChange {
            subtract: Some(false),
            carry: Some(false),
            half_carry: Some(false),
            zero: Some(new_value == 0)
        },
        register: RegisterChange {
            a: Some(new_value),
            ..RegisterChange::default()
        },
        memory: MemoryChange::default()
    }
}

fn and_to_a(a_value: u8, operand: u8) -> StateChange {
    let new_value = a_value & operand;

    StateChange {
        byte_length: 1,
        t_states: 4,
        flags: FlagChange {
            subtract: Some(false),
            carry: Some(false),
            half_carry: Some(true),
            zero: Some(new_value == 0)
        },
        register: RegisterChange {
            a: Some(new_value),
            ..RegisterChange::default()
        },
        memory: MemoryChange::default()
    }
}

fn add_to_a(a_value: u8, operand: u8) -> StateChange {
    let new_value = add8_bit(a_value, operand);

    StateChange {
        byte_length: 1,
        t_states: 4,
        flags: FlagChange {
            subtract: Some(false),
            carry: Some(is_carry_add(a_value, operand)),
            half_carry: Some(is_half_carry_add(a_value, operand)),
            zero: Some(new_value == 0)
        },
        register: RegisterChange {
            a: Some(new_value),
            ..RegisterChange::default()
        },
        memory: MemoryChange::default()
    }
}

fn sub_from_a(a_value: u8, operand: u8) -> StateChange {
    let new_value = sub8_bit(a_value, operand);

    StateChange {
        byte_length: 1,
        t_states: 4,
        flags: FlagChange {
            subtract: Some(true),
            carry: Some(is_carry_subtract(a_value, operand)),
            half_carry: Some(is_half_carry_subtract(a_value, operand)),
            zero: Some(new_value == 0)
        },
        register: RegisterChange {
            a: Some(new_value),
            ..RegisterChange::default()
        },
        memory: MemoryChange::default()
    }
}

fn add_to_hl(hl_value: u16, operand: u16) -> StateChange {
    StateChange {
        byte_length: 1,
        t_states: 8,
        flags: FlagChange {
            subtract: Some(false),
            carry: Some(is_carry_add_16(hl_value, operand)),
            half_carry: Some(is_half_carry_add_16(hl_value, operand)),
            ..FlagChange::default()
        },
        memory: MemoryChange::default(),
        register: {
            let (h, l) = to8_bit(add16_bit(hl_value, operand));

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

fn ld_from_absolute(change: RegisterChange) -> StateChange {
    StateChange {
        byte_length: 1,
        t_states: 8,
        flags: FlagChange::default(),
        register: change,
        memory: MemoryChange::default()
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