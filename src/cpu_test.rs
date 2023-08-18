use std::collections::HashMap;

use super::CPU;
use crate::cpu::{flags::is_half_carry_subtract, registers::to16_bit};

const PROGRAM_COUNTER: u16 = 0;

fn prepare_cpu() -> CPU {
    let mut cpu = CPU::new();
    cpu.registers.program_counter = PROGRAM_COUNTER;

    cpu
}

fn get_register(cpu: &mut CPU, index: i32) -> Option<&mut u8> {
    match index {
        0x00 => Option::Some(&mut cpu.registers.b),
        0x01 => Option::Some(&mut cpu.registers.c),
        0x02 => Option::Some(&mut cpu.registers.d),
        0x03 => Option::Some(&mut cpu.registers.e),
        0x04 => Option::Some(&mut cpu.registers.h),
        0x05 => Option::Some(&mut cpu.registers.l),
        0x07 => Option::Some(&mut cpu.registers.a),
        _ => Option::None
    }
}

#[test]
fn test_0x00() { //nop
    let mut cpu = prepare_cpu();

    cpu.execute(0x00);

    assert_eq!(1, cpu.registers.program_counter);
}

#[test]
fn test_0x01() { //LD BC, u16
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x01, Option::Some(vec![0xA0, 0x01])); //load BC with A001

    assert_eq!(3, cpu.registers.program_counter);
    assert_eq!(cpu.registers.bc(), 0xA001);
}

#[test]
fn test_0x02() { //LD (BC), A
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x3E, Option::Some(vec![100])); //load A with 100
    cpu.execute_with_args(0x01, Option::Some(vec![0xA0, 0x01])); //load BC with A001
    cpu.execute(0x02);

    assert_eq!(6, cpu.registers.program_counter);
    assert_eq!(100, cpu.registers.a);
    assert_eq!(cpu.registers.bc(), 0xA001);
    assert_eq!(cpu.memory[0xA001], 100);
}

#[test]
fn test_0x03() { //INC BC
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x01, Option::Some(vec![0xA0, 0x01])); //load BC with A001
    cpu.execute(0x03);

    assert_eq!(4, cpu.registers.program_counter);
    assert_eq!(cpu.registers.bc(), 0xA002);
}

#[test]
fn test_0x04() { //INC B
    let mut cpu = prepare_cpu();

    cpu.flags.subtract = true;
    cpu.registers.b = 5;
    cpu.execute(0x04);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(6, cpu.registers.b);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.half_carry);

    cpu.registers.b = 255;
    cpu.execute(0x04);

    assert_eq!(0, cpu.registers.b);
    assert!(cpu.flags.zero);
}

#[test]
fn test_0x05() { //DEC B
    let mut cpu = prepare_cpu();

    cpu.registers.b = 1;
    cpu.execute(0x05);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0, cpu.registers.b);
    assert!(cpu.flags.zero);
    assert!(cpu.flags.subtract);
}

#[test]
fn test_0x06() { //LD B, u8
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x06, Option::Some(vec![0x0A]));

    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(0x0A, cpu.registers.b);
}

#[test]
fn test_0x07() { //RLCA
    let mut cpu = prepare_cpu();

    cpu.registers.a = 0x85;
    cpu.execute(0x07);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x0B, cpu.registers.a);
    assert!(cpu.flags.carry);

    cpu.registers.a = 0x7F;
    cpu.execute(0x07);

    assert_eq!(0xFE, cpu.registers.a);
    assert!(!cpu.flags.carry);
}

#[test]
fn test_0x08() { //LD [n16], SP
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0xC001;
    cpu.execute_with_args(0x08, Some(vec![0xA1, 0xFF]));

    assert_eq!(3, cpu.registers.program_counter);
    assert_eq!(0xC0, cpu.memory[0xA1FF]);
    assert_eq!(0x01, cpu.memory[0xA1FF + 1]);
}

#[test]
fn test_0x09() { //ADD HL, BC
    let mut cpu = prepare_cpu();

    cpu.flags.subtract = true;

    cpu.registers.b = 0xA0;
    cpu.registers.c = 0xAF;
    cpu.registers.h = 0x00;
    cpu.registers.l = 0x01;

    cpu.execute(0x09);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0xA0, cpu.registers.h);
    assert_eq!(0xB0, cpu.registers.l);
    assert!(!cpu.flags.subtract);
}

#[test]
#[allow(non_snake_case)]
fn test_0x0A() { //LD A, [BC]
    let mut cpu = prepare_cpu();

    cpu.registers.b = 0xA0;
    cpu.registers.c = 0xAF;
    cpu.memory[0xA0AF] = 200;

    cpu.execute(0x0A);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(200, cpu.registers.a);
}

#[test]
#[allow(non_snake_case)]
fn test_0x0B() { //DEC BC
    let mut cpu = prepare_cpu();

    cpu.registers.b = 0xA0;
    cpu.registers.c = 0xAF;

    cpu.execute(0x0B);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0xA0AE, cpu.registers.bc());
}

#[test]
#[allow(non_snake_case)]
fn test_0x0C() { //INC C
    let mut cpu = prepare_cpu();

    cpu.registers.c = 0x0A;

    cpu.execute(0x0C);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x0B, cpu.registers.c);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);

    cpu.registers.c = 0xFF;

    cpu.execute(0x0C);

    assert_eq!(0x00, cpu.registers.c);
    assert!(cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(cpu.flags.half_carry); //11111111 + 00000001 = 1 gets carried past 4th bit

    cpu.registers.c = 0x0F;

    cpu.execute(0x0C);

    assert_eq!(0x10, cpu.registers.c);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(cpu.flags.half_carry); //00001111 + 00000001 = 1 gets carried past 4th bit
}

#[test]
#[allow(non_snake_case)]
fn test_0x0D() { //DEC C
    let mut cpu = prepare_cpu();

    cpu.registers.c = 0x0A;

    cpu.execute(0x0D);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x09, cpu.registers.c);
    assert!(!cpu.flags.zero);
    assert!(cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);

    cpu.registers.c = 0x01;

    cpu.execute(0x0D);

    assert_eq!(0x00, cpu.registers.c);
    assert!(cpu.flags.zero);
    assert!(cpu.flags.subtract);
    assert!(!cpu.flags.half_carry); //11111111 + 00000001 = 1 gets carried past 4th bit

    cpu.registers.c = 0x10;

    cpu.execute(0x0D);

    assert_eq!(0x0F, cpu.registers.c);
    assert!(!cpu.flags.zero);
    assert!(cpu.flags.subtract);
    assert!(cpu.flags.half_carry); //0001000 - 00000001 = 1 gets carried past 4th bit
}

#[test]
#[allow(non_snake_case)]
fn test_0x0E() { //LD C, u8
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x0E, Some(vec![0x0A]));

    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(0x0A, cpu.registers.c);
}

#[test]
#[allow(non_snake_case)]
fn test_0x0F() { //RRCA
    let mut cpu = prepare_cpu();

    cpu.registers.a = 0x01;

    cpu.execute(0x0F);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x80, cpu.registers.a);
    assert!(cpu.flags.carry);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);

    cpu.registers.a = 0x04;

    cpu.execute(0x0F);

    assert_eq!(0x02, cpu.registers.a);
    assert!(!cpu.flags.carry);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);
}

#[test]
fn test_0x10() { //STOP
    let mut cpu = prepare_cpu();

    cpu.execute(0x10);

    assert_eq!(2, cpu.registers.program_counter);
}

#[test]
fn test_0x11() { //LD DE, u16
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x11, Option::Some(vec![0xA5, 0x01])); //load DE with A501

    assert_eq!(3, cpu.registers.program_counter);
    assert_eq!(cpu.registers.de(), 0xA501);
}

#[test]
fn test_0x12() { //LD (DE), A
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x3E, Option::Some(vec![100])); //load A with 100
    cpu.execute_with_args(0x11, Option::Some(vec![0xA0, 0x01])); //load DE with A001
    cpu.execute(0x12);

    assert_eq!(6, cpu.registers.program_counter);
    assert_eq!(100, cpu.registers.a);
    assert_eq!(cpu.registers.de(), 0xA001);
    assert_eq!(cpu.memory[0xA001], 100);
}

#[test]
fn test_0x13() { //INC DE
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x11, Option::Some(vec![0xA0, 0x01])); //load DE with A001
    cpu.execute(0x13);

    assert_eq!(4, cpu.registers.program_counter);
    assert_eq!(cpu.registers.de(), 0xA002);
}

#[test]
fn test_0x14() { //INC D
    let mut cpu = prepare_cpu();

    cpu.flags.subtract = true;
    cpu.registers.d = 7;
    cpu.execute(0x14);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(8, cpu.registers.d);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.half_carry);

    cpu.registers.d = 255;
    cpu.execute(0x14);

    assert_eq!(0, cpu.registers.d);
    assert!(cpu.flags.zero);
}

#[test]
fn test_0x15() { //DEC D
    let mut cpu = prepare_cpu();

    cpu.registers.d = 5;
    cpu.execute(0x15);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(4, cpu.registers.d);
    assert!(!cpu.flags.zero);
    assert!(cpu.flags.subtract);
}

#[test]
fn test_0x16() { //LD D, u8
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x16, Option::Some(vec![0x0A]));

    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(0x0A, cpu.registers.d);
}

#[test]
fn test_0x17() { //RLA
    let mut cpu = prepare_cpu();

    cpu.registers.a = 0x02;
    cpu.execute(0x17);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x04, cpu.registers.a);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);
    assert!(!cpu.flags.carry);

    cpu.registers.a = 0x80;
    cpu.execute(0x17);

    assert_eq!(0x00, cpu.registers.a);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);
    assert!(cpu.flags.carry);

    cpu.execute(0x17);

    assert_eq!(0x01, cpu.registers.a);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);
    assert!(!cpu.flags.carry);
}

#[test]
fn test_0x18() { //JR e8
    let mut cpu = prepare_cpu();
    cpu.registers.program_counter = 0x05;

    cpu.execute_with_args(0x18, Some(vec![0xFE])); //-2

    assert_eq!(0x05, cpu.registers.program_counter);

    cpu.registers.program_counter = 0x05;

    cpu.execute_with_args(0x18, Some(vec![0xFD])); //-3

    assert_eq!(0x04, cpu.registers.program_counter);
}

#[test]
fn test_0x19() { //ADD HL, DE
    let mut cpu = prepare_cpu();

    cpu.flags.subtract = true;

    cpu.registers.d = 0xA0;
    cpu.registers.e = 0xAF;
    cpu.registers.h = 0x00;
    cpu.registers.l = 0x01;

    cpu.execute(0x19);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0xA0, cpu.registers.h);
    assert_eq!(0xB0, cpu.registers.l);
    assert!(!cpu.flags.subtract);
}

#[test]
#[allow(non_snake_case)]
fn test_0x1A() { //LD A, [DE]
    let mut cpu = prepare_cpu();

    cpu.registers.d = 0xA0;
    cpu.registers.e = 0xAF;
    cpu.memory[0xA0AF] = 200;

    cpu.execute(0x1A);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(200, cpu.registers.a);
}

#[test]
#[allow(non_snake_case)]
fn test_0x1B() { //DEC DE
    let mut cpu = prepare_cpu();

    cpu.registers.d = 0xA0;
    cpu.registers.e = 0xAF;

    cpu.execute(0x1B);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0xA0AE, cpu.registers.de());
}

#[test]
#[allow(non_snake_case)]
fn test_0x1C() { //INC E
    let mut cpu = prepare_cpu();

    cpu.registers.e = 0x0A;

    cpu.execute(0x1C);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x0B, cpu.registers.e);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);

    cpu.registers.e = 0xFF;

    cpu.execute(0x1C);

    assert_eq!(0x00, cpu.registers.e);
    assert!(cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(cpu.flags.half_carry); //11111111 + 00000001 = 1 gets carried past 4th bit

    cpu.registers.e = 0x0F;

    cpu.execute(0x1C);

    assert_eq!(0x10, cpu.registers.e);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(cpu.flags.half_carry); //00001111 + 00000001 = 1 gets carried past 4th bit
}

#[test]
#[allow(non_snake_case)]
fn test_0x1D() { //DEC E
    let mut cpu = prepare_cpu();

    cpu.registers.e = 0x0A;

    cpu.execute(0x1D);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x09, cpu.registers.e);
    assert!(!cpu.flags.zero);
    assert!(cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);

    cpu.registers.e = 0x01;

    cpu.execute(0x1D);

    assert_eq!(0x00, cpu.registers.e);
    assert!(cpu.flags.zero);
    assert!(cpu.flags.subtract);
    assert!(!cpu.flags.half_carry); //11111111 + 00000001 = 1 gets carried past 4th bit

    cpu.registers.e = 0x10;

    cpu.execute(0x1D);

    assert_eq!(0x0F, cpu.registers.e);
    assert!(!cpu.flags.zero);
    assert!(cpu.flags.subtract);
    assert!(cpu.flags.half_carry); //0001000 - 00000001 = 1 gets carried past 4th bit
}

#[test]
#[allow(non_snake_case)]
fn test_0x1E() { //LD E, u8
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x1E, Some(vec![0x0B]));

    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(0x0B, cpu.registers.e);
}

#[test]
#[allow(non_snake_case)]
fn test_0x1F() { //RRA
    let mut cpu = prepare_cpu();

    cpu.registers.a = 0x02;
    cpu.execute(0x1F);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x01, cpu.registers.a);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);
    assert!(!cpu.flags.carry);

    cpu.registers.a = 0x01;
    cpu.execute(0x1F);

    assert_eq!(0x00, cpu.registers.a);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);
    assert!(cpu.flags.carry);

    cpu.execute(0x1F);

    assert_eq!(0x80, cpu.registers.a);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);
    assert!(!cpu.flags.carry);
}

#[test]
fn test_0x20() { //JR NZ, e8
    let mut cpu = prepare_cpu();

    cpu.flags.zero = false;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x20, Some(vec![0xFE])); //-2

    assert_eq!(0x05, cpu.registers.program_counter);

    cpu.flags.zero = false;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x20, Some(vec![0xFD])); //-3

    assert_eq!(0x04, cpu.registers.program_counter);

    cpu.flags.zero = false;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x20, Some(vec![0x03])); //+3

    assert_eq!(0x0A, cpu.registers.program_counter);

    cpu.flags.zero = true;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x20, Some(vec![0x03])); //+3

    assert_eq!(0x07, cpu.registers.program_counter);
}

#[test]
fn test_0x21() { //LD HL, u16
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x21, Option::Some(vec![0xC0, 0x01])); //load HL with c001

    assert_eq!(3, cpu.registers.program_counter);
    assert_eq!(cpu.registers.hl(), 0xC001);
}

#[test]
fn test_0x22() { //LD (HL+), A
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x3E, Option::Some(vec![100])); //load A with 100
    cpu.execute_with_args(0x21, Option::Some(vec![0xC0, 0x01])); //load HL with c001
    cpu.execute(0x22); //load mem at (HL) with A then increment HL

    assert_eq!(6, cpu.registers.program_counter);
    assert_eq!(cpu.registers.a, 100);
    assert_eq!(cpu.registers.hl(), 0xC002);
    assert_eq!(cpu.memory[0xC001], 100);
}

#[test]
fn test_0x23() { //INC HL
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x21, Option::Some(vec![0xA0, 0x01])); //load HL with A001
    cpu.execute(0x23);

    assert_eq!(4, cpu.registers.program_counter);
    assert_eq!(cpu.registers.hl(), 0xA002);
}

#[test]
fn test_0x24() { //INC H
    let mut cpu = prepare_cpu();

    cpu.flags.subtract = true;
    cpu.registers.h = 7;
    cpu.execute(0x24);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(8, cpu.registers.h);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.zero);
    assert!(!cpu.flags.half_carry);

    cpu.registers.h = 255;
    cpu.execute(0x24);

    assert_eq!(0, cpu.registers.h);
    assert!(cpu.flags.zero);
}

#[test]
fn test_0x25() { //DEC H
    let mut cpu = prepare_cpu();

    cpu.registers.h = 5;
    cpu.execute(0x25);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(4, cpu.registers.h);
    assert!(!cpu.flags.zero);
    assert!(cpu.flags.subtract);
}

#[test]
fn test_0x26() { //LD H, u8
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x26, Option::Some(vec![0x0A]));

    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(0x0A, cpu.registers.h);
}

#[test]
fn test_0x27() { //DAA
    let mut cpu = prepare_cpu();

    cpu.registers.a = 0x07;
    cpu.execute(0x87); //7 + 7 = 14 (00001110 non bcd)

    assert_eq!(0b00001110, cpu.registers.a); //0x0E = 00001110 (14 non bcd)

    cpu.execute(0x27); //correct to bcd so 14 (0001,0100)

    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(0b00010100, cpu.registers.a); //0x14 = 00010100 (14 in BCD)
}

#[test]
fn test_0x28() { //JR Z, e8
    let mut cpu = prepare_cpu();

    cpu.flags.zero = true;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x28, Some(vec![0xFE])); //-2

    assert_eq!(0x05, cpu.registers.program_counter);

    cpu.flags.zero = true;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x28, Some(vec![0xFD])); //-3

    assert_eq!(0x04, cpu.registers.program_counter);

    cpu.flags.zero = true;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x28, Some(vec![0x03])); //+3

    assert_eq!(0x0A, cpu.registers.program_counter);

    cpu.flags.zero = false;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x28, Some(vec![0x03])); //+3

    assert_eq!(0x07, cpu.registers.program_counter);
}

#[test]
fn test_0x29() { //ADD HL, HL
    let mut cpu = prepare_cpu();

    cpu.flags.subtract = true;

    cpu.registers.h = 0x00;
    cpu.registers.l = 0x01;

    cpu.execute(0x29);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x00, cpu.registers.h);
    assert_eq!(0x02, cpu.registers.l);
    assert!(!cpu.flags.subtract);
}

#[test]
#[allow(non_snake_case)]
fn test_0x2A() { //LD A, (HL+)
    let mut cpu = prepare_cpu();

    cpu.memory[0xC001] = 0x24;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;

    cpu.execute(0x2A);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x24, cpu.registers.a);
    assert_eq!(0xC002, cpu.registers.hl());
}

#[test]
#[allow(non_snake_case)]
fn test_0x2B() { //DEC HL
    let mut cpu = prepare_cpu();

    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;

    cpu.execute(0x2B);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0xC000, cpu.registers.hl());
}

#[test]
#[allow(non_snake_case)]
fn test_0x2C() { //INC L
    let mut cpu = prepare_cpu();

    cpu.registers.l = 0x01;

    cpu.execute(0x2C);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x02, cpu.registers.l);
}

#[test]
#[allow(non_snake_case)]
fn test_0x2D() { //DEC L
    let mut cpu = prepare_cpu();

    cpu.registers.l = 0x0A;

    cpu.execute(0x2D);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x09, cpu.registers.l);
}

#[test]
#[allow(non_snake_case)]
fn test_0x2E() { //LD L, u8
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x2E, Option::Some(vec![100])); //load L with 100
    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(cpu.registers.l, 100);
}

#[test]
#[allow(non_snake_case)]
fn test_0x2F() { //CPL
    let mut cpu = prepare_cpu();

    cpu.execute(0x2F);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0xFF, cpu.registers.a);
    assert!(cpu.flags.subtract);
    assert!(cpu.flags.half_carry);

    cpu.registers.a = 0x02;
    cpu.execute(0x2F);

    assert_eq!(0xFD, cpu.registers.a);
    assert!(cpu.flags.subtract);
    assert!(cpu.flags.half_carry);
}

#[test]
fn test_0x30() { //JR NC, e8
    let mut cpu = prepare_cpu();

    cpu.flags.carry = false;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x30, Some(vec![0xFE])); //-2

    assert_eq!(0x05, cpu.registers.program_counter);

    cpu.flags.carry = false;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x30, Some(vec![0xFD])); //-3

    assert_eq!(0x04, cpu.registers.program_counter);

    cpu.flags.carry = false;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x30, Some(vec![0x03])); //+3

    assert_eq!(0x0A, cpu.registers.program_counter);

    cpu.flags.carry = true;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x30, Some(vec![0x03])); //+3

    assert_eq!(0x07, cpu.registers.program_counter);
}

#[test]
fn test_0x31() { //LD SP, u16
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x31, Option::Some(vec![0xC0, 0x01])); //load HL with c001

    assert_eq!(3, cpu.registers.program_counter);
    assert_eq!(cpu.registers.stack_pointer, 0xC001);
}

#[test]
fn test_0x32() { //LD (HL-), A
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x3E, Option::Some(vec![100])); //load A with 100
    cpu.execute_with_args(0x21, Option::Some(vec![0xC0, 0x01])); //load HL with c001
    cpu.execute(0x32); //load mem at (HL) with A then decrement HL

    assert_eq!(6, cpu.registers.program_counter);
    assert_eq!(cpu.registers.a, 100);
    assert_eq!(cpu.registers.hl(), 0xC000);
    assert_eq!(cpu.memory[0xC001], 100);
}

#[test]
fn test_0x33() { //INC SP
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x31, Option::Some(vec![0xA0, 0x01])); //load SP with A001
    cpu.execute(0x33);

    assert_eq!(4, cpu.registers.program_counter);
    assert_eq!(cpu.registers.stack_pointer, 0xA002);
}

#[test]
fn test_0x34() { //INC (HL)
    let mut cpu = prepare_cpu();

    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;
    cpu.memory[0xC001] = 0x30;

    cpu.execute(0x34);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x31, cpu.memory[0xC001]);
    assert_eq!(0xC001, cpu.registers.hl());
}

#[test]
fn test_0x35() { //DEC (HL)
    let mut cpu = prepare_cpu();

    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;
    cpu.memory[0xC001] = 0x30;

    cpu.execute(0x35);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x2F, cpu.memory[0xC001]);
    assert_eq!(0xC001, cpu.registers.hl());
}

#[test]
fn test_0x36() { //LD (HL), u8
    let mut cpu = prepare_cpu();

    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;
    cpu.execute_with_args(0x36, Some(vec![0x2F]));

    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(0x2F, cpu.memory[0xC001]);
}

#[test]
fn test_0x37() { //SCF (S)
    let mut cpu = prepare_cpu();

    cpu.flags.subtract = true;
    cpu.flags.half_carry = true;

    cpu.execute(0x37);

    assert_eq!(1, cpu.registers.program_counter);
    assert!(cpu.flags.carry);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);
}

#[test]
fn test_0x38() { //JR C, e8
    let mut cpu = prepare_cpu();

    cpu.flags.carry = true;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x38, Some(vec![0xFE])); //-2

    assert_eq!(0x05, cpu.registers.program_counter);

    cpu.flags.carry = true;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x38, Some(vec![0xFD])); //-3

    assert_eq!(0x04, cpu.registers.program_counter);

    cpu.flags.carry = true;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x38, Some(vec![0x03])); //+3

    assert_eq!(0x0A, cpu.registers.program_counter);

    cpu.flags.carry = false;
    cpu.registers.program_counter = 0x05;
    cpu.execute_with_args(0x38, Some(vec![0x03])); //+3

    assert_eq!(0x07, cpu.registers.program_counter);
}

#[test]
fn test_0x39() { //ADD HL, SP
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0xFF01;
    cpu.registers.h = 0x00;
    cpu.registers.l = 0x05;

    cpu.execute(0x39);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0xFF06, cpu.registers.hl());
}

#[test]
#[allow(non_snake_case)]
fn test_0x3A() { //LD A, (HL-)
    let mut cpu = prepare_cpu();

    cpu.memory[0xC001] = 0x24;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;

    cpu.execute(0x3A);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x24, cpu.registers.a);
    assert_eq!(0xC000, cpu.registers.hl());
}

#[test]
#[allow(non_snake_case)]
fn test_0x3B() { //DEC SP
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0xC001;

    cpu.execute(0x3B);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0xC000, cpu.registers.stack_pointer);
}

#[test]
#[allow(non_snake_case)]
fn test_0x3C() { //INC A
    let mut cpu = prepare_cpu();

    cpu.registers.a = 0x01;

    cpu.execute(0x3C);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x02, cpu.registers.a);
}

#[test]
#[allow(non_snake_case)]
fn test_0x3D() { //DEC A
    let mut cpu = prepare_cpu();

    cpu.registers.a = 0x02;

    cpu.execute(0x3D);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x01, cpu.registers.a);
}

#[test]
#[allow(non_snake_case)]
fn test_0x3E() { //LD A, u8
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x3E, Option::Some(vec![100])); //load A with 100
    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(cpu.registers.a, 100);
}

#[test]
#[allow(non_snake_case)]
fn test_0x3F() { //CCF
    let mut cpu = prepare_cpu();

    cpu.flags.carry = true;
    cpu.flags.half_carry = true;
    cpu.flags.subtract = true;

    cpu.execute(0x3F);

    assert!(!cpu.flags.carry);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);

    cpu.execute(0x3F);

    assert!(cpu.flags.carry);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);
}

#[test]
fn test_ld_register_to_register() { //all `LD r, r` instructions
    let mut cpu = prepare_cpu();
    let (mut row, mut col, mut reg) = (0x40, 0x00, 0);

    loop {
        loop {
            let opcode = row + col;
            let target_reg = col % 0x08;
            let is_a = target_reg == 0x07;

            //open block here to borrow mut reference to work out expected value
            let expected = {
                let target = get_register(&mut cpu, target_reg);

                if let Option::Some(target_reg) = target {
                    *target_reg = rand::random::<u8>();

                    *target_reg
                } else {
                    0
                }
            };

            if expected != 0 {
                cpu.execute(opcode as u8);

                let actual = {
                    let src = get_register(
                        &mut cpu,
                        if is_a { 0x07 } else { reg }
                    );

                    if let Option::Some(src_reg) = src {
                        *src_reg
                    } else {
                        0
                    }
                };

                assert_eq!(
                    expected,
                    actual,
                    "testing ld r,r with registers {:#02x} and {:#02x}",
                    reg,
                    target_reg
                );
            }

            col += 0x01;
            if col == 0x07 {
                reg += 1;
            }

            if col > 0x0F {
                break;
            }
        }

        col = 0x00;
        row += 0x10;
        reg += 1;

        //only 0x76 ~ 0x7F are LD r,r
        if row == 0x70 {
            col = 0x08;
            reg += 1;
        }

        if row == 0x80 {
            break;
        }
    }
}

const LD_HL_R_START: u8 = 0x70;
#[test]
fn test_ld_register_to_hl_absolute() {
    let mut cpu = prepare_cpu();

    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;

    for opcode in LD_HL_R_START..0x78 {
        let expected = {
            let reg = get_register(&mut cpu, (opcode - LD_HL_R_START) as i32);

            if let Option::None = reg {
                continue;
            }

            let reg = reg.unwrap();
            *reg = rand::random::<u8>();
            *reg
        };

        let addr = cpu.registers.hl() as usize;

        cpu.memory[addr] = 0;
        cpu.execute(opcode);

        assert_eq!(
            cpu.memory[addr],
            expected,
            "executing {:#02x}",
            opcode
        );
    }
}

#[test]
fn test_0x46() { //LD B, [HL]
    let mut cpu = prepare_cpu();

    cpu.memory[0xC001] = 0x42;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;

    cpu.execute(0x46);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x42, cpu.registers.b);
}

#[test]
#[allow(non_snake_case)]
fn test_0x4E() { //LD C, [HL]
    let mut cpu = prepare_cpu();

    cpu.memory[0xC001] = 0x42;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;

    cpu.execute(0x4E);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x42, cpu.registers.c);
}

#[test]
#[allow(non_snake_case)]
fn test_0x56() { //LD D, [HL]
    let mut cpu = prepare_cpu();

    cpu.memory[0xC001] = 0x42;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;

    cpu.execute(0x56);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x42, cpu.registers.d);
}

#[test]
#[allow(non_snake_case)]
fn test_0x5E() { //LD E, [HL]
    let mut cpu = prepare_cpu();

    cpu.memory[0xC001] = 0x42;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;

    cpu.execute(0x5E);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x42, cpu.registers.e);
}

#[test]
#[allow(non_snake_case)]
fn test_0x66() { //LD H, [HL]
    let mut cpu = prepare_cpu();

    cpu.memory[0xC001] = 0x42;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;

    cpu.execute(0x66);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x42, cpu.registers.h);
}

#[test]
#[allow(non_snake_case)]
fn test_0x6E() { //LD L, [HL]
    let mut cpu = prepare_cpu();

    cpu.memory[0xC001] = 0x42;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;

    cpu.execute(0x6E);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x42, cpu.registers.l);
}

#[test]
#[allow(non_snake_case)]
fn test_0x7E() { //LD A, [HL]
    let mut cpu = prepare_cpu();

    cpu.memory[0xC001] = 0x42;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;

    cpu.execute(0x7E);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x42, cpu.registers.a);
}

const ADD_A_R_START: u8 = 0x80;
const TO_ADD: u8 = 0x42;
#[test]
fn test_register_add_to_a() { //ADD A, r
    let mut cpu = prepare_cpu();

    for opcode in ADD_A_R_START..0x88 {
        let expected = match opcode {
            0x87 => TO_ADD,
            _ => {
                let reg = get_register(&mut cpu, (opcode - ADD_A_R_START) as i32);

                if let Option::None = reg {
                    continue;
                }

                let reg = reg.unwrap();
                *reg = rand::random::<u8>();
                *reg
            }
        };

        cpu.registers.a = TO_ADD;
        cpu.execute(opcode);

        assert_eq!(
            cpu.registers.a,
            expected.wrapping_add(TO_ADD),
            "executing {:#02x}",
            opcode
        );
    }
}

#[test]
fn test_0x86() { //ADD A, [HL]
    let mut cpu = prepare_cpu();

    cpu.registers.a = 0x0A;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;
    cpu.memory[0xC001] = 0x14;

    cpu.execute(0x86);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x1E, cpu.registers.a);
}

const ADC_A_R_START: u8 = 0x88;
#[test]
fn test_register_adc_to_a() { //ADC A, r
    let mut cpu = prepare_cpu();

    for opcode in ADC_A_R_START..0x90 {
        let expected = match opcode {
            0x8F => TO_ADD,
            _ => {
                let reg = get_register(&mut cpu, (opcode - ADC_A_R_START) as i32);

                if let Option::None = reg {
                    continue;
                }

                let reg = reg.unwrap();
                *reg = rand::random::<u8>();
                *reg
            }
        };

        cpu.flags.carry = true;
        cpu.registers.a = TO_ADD;
        cpu.execute(opcode);

        assert_eq!(
            cpu.registers.a,
            expected.wrapping_add(TO_ADD.wrapping_add(1)),
            "executing {:#02x}",
            opcode
        );
    }
}

#[test]
#[allow(non_snake_case)]
fn test_0x8E() { //ADC A, [HL]
    let mut cpu = prepare_cpu();

    cpu.flags.carry = true;
    cpu.registers.a = 0x0A;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;
    cpu.memory[0xC001] = 0x14;

    cpu.execute(0x8E);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x1F, cpu.registers.a);
}

const SUB_A_R_START: u8 = 0x90;
const TO_SUB: u8 = 0x42;
#[test]
fn test_register_sub_from_a() { //SUB A, r
    let mut cpu = prepare_cpu();

    for opcode in SUB_A_R_START..0x98 {
        let expected = {
            let reg = get_register(&mut cpu, (opcode - SUB_A_R_START) as i32);

            if let Option::None = reg {
                continue;
            }

            let reg = reg.unwrap();
            *reg = rand::random::<u8>();
            *reg
        };

        cpu.registers.a = TO_SUB;
        cpu.execute(opcode);

        if opcode == 0x97 { //SUB A, A will always result in zero and the same flags
            assert_eq!(0, cpu.registers.a, "executing 0x97");
            assert!(cpu.flags.zero);
            assert!(cpu.flags.subtract);
            assert!(!cpu.flags.half_carry);
            assert!(!cpu.flags.carry);

            continue;
        }

        assert_eq!(
            cpu.registers.a,
            TO_SUB.wrapping_sub(expected),
            "executing {:#02x}",
            opcode
        );
    }
}

#[test]
fn test_0x96() { //SUB A, [HL]
    let mut cpu = prepare_cpu();

    cpu.registers.a = 0x0A;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;
    cpu.memory[0xC001] = 0x02;

    cpu.execute(0x96);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x08, cpu.registers.a);
}

const SBC_A_R_START: u8 = 0x98;
#[test]
fn test_register_sbc_from_a() { //SBC A, r
    let mut cpu = prepare_cpu();

    for opcode in SBC_A_R_START..0xA0 {
        let expected = match opcode {
            0x9F => TO_SUB,
            _ => {
                let reg = get_register(&mut cpu, (opcode - SBC_A_R_START) as i32);

                if let Option::None = reg {
                    continue;
                }

                let reg = reg.unwrap();
                *reg = rand::random::<u8>();
                *reg
            }
        };

        cpu.flags.carry = true;
        cpu.registers.a = TO_SUB;
        cpu.execute(opcode);

        assert_eq!(
            cpu.registers.a,
            TO_SUB.wrapping_sub(expected.wrapping_add(1)),
            "executing {:#02x}",
            opcode
        );
    }
}

#[test]
#[allow(non_snake_case)]
fn test_0x9E() { //SBC A, [HL]
    let mut cpu = prepare_cpu();

    cpu.flags.carry = true;
    cpu.registers.a = 0x0A;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;
    cpu.memory[0xC001] = 0x06;

    cpu.execute(0x9E);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x03, cpu.registers.a);
    assert!(cpu.flags.subtract);
}

const AND_A_R_START: u8 = 0xA0;
const TO_AND: u8 = 0x42;
#[test]
fn test_register_and_to_a() { //AND A, r
    let mut cpu = prepare_cpu();

    for opcode in AND_A_R_START..0xA8 {
        let expected = match opcode {
            0xA7 => TO_AND,
            _ => {
                let reg = get_register(&mut cpu, (opcode - AND_A_R_START) as i32);

                if let Option::None = reg {
                    continue;
                }

                let reg = reg.unwrap();
                *reg = rand::random::<u8>();
                *reg
            }
        };

        cpu.registers.a = TO_AND;
        cpu.execute(opcode);

        let expected = TO_AND & expected;

        assert_eq!(
            cpu.registers.a,
            expected,
            "executing {:#02x}",
            opcode
        );
        assert!(!cpu.flags.subtract);
        assert!(cpu.flags.half_carry);
        assert!(!cpu.flags.carry);
        assert_eq!(cpu.flags.zero, expected == 0);
    }
}

#[test]
#[allow(non_snake_case)]
fn test_0xA6() { //AND A, [HL]
    let mut cpu = prepare_cpu();

    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;
    cpu.registers.a = 0b01010010;
    cpu.memory[0xC001] = 0b00010011;

    cpu.execute(0xA6);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0b00010010, cpu.registers.a);
    assert!(!cpu.flags.subtract);
    assert!(cpu.flags.half_carry);
    assert!(!cpu.flags.carry);
    assert!(!cpu.flags.zero);
}

const XOR_A_R_START: u8 = 0xA8;
const TO_XOR: u8 = 0x42;
#[test]
fn test_register_xor_to_a() { //XOR A, r
    let mut cpu = prepare_cpu();

    for opcode in XOR_A_R_START..0xB0 {
        let expected = match opcode {
            0xAF => TO_XOR,
            _ => {
                let reg = get_register(&mut cpu, (opcode - XOR_A_R_START) as i32);

                if let Option::None = reg {
                    continue;
                }

                let reg = reg.unwrap();
                *reg = rand::random::<u8>();
                *reg
            }
        };

        cpu.registers.a = TO_XOR;
        cpu.execute(opcode);

        if opcode == 0xAF { //XOR A, A will always result in the same result
            assert_eq!(0, cpu.registers.a);
            assert!(!cpu.flags.subtract);
            assert!(!cpu.flags.half_carry);
            assert!(!cpu.flags.carry);
            assert!(cpu.flags.zero);

            continue;
        }

        let expected = TO_XOR ^ expected;

        assert_eq!(
            cpu.registers.a,
            expected,
            "executing {:#02x}",
            opcode
        );
        assert!(!cpu.flags.subtract);
        assert!(!cpu.flags.half_carry);
        assert!(!cpu.flags.carry);
        assert_eq!(cpu.flags.zero, expected == 0);
    }
}

#[test]
#[allow(non_snake_case)]
fn test_0xAE() { //XOR A, [HL]
    let mut cpu = prepare_cpu();

    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;
    cpu.registers.a = 0b01010010;
    cpu.memory[0xC001] = 0b00010011;

    cpu.execute(0xAE);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0b01000001, cpu.registers.a);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);
    assert!(!cpu.flags.carry);
    assert!(!cpu.flags.zero);
}

const OR_A_R_START: u8 = 0xB0;
const TO_OR: u8 = 0x42;
#[test]
fn test_register_or_to_a() { //OR A, r
    let mut cpu = prepare_cpu();

    for opcode in OR_A_R_START..0xB8 {
        let expected = match opcode {
            0xB7 => TO_OR,
            _ => {
                let reg = get_register(&mut cpu, (opcode - OR_A_R_START) as i32);

                if let Option::None = reg {
                    continue;
                }

                let reg = reg.unwrap();
                *reg = rand::random::<u8>();
                *reg
            }
        };

        cpu.registers.a = TO_OR;
        cpu.execute(opcode);

        let expected = TO_OR | expected;

        assert_eq!(
            cpu.registers.a,
            expected,
            "executing {:#02x}",
            opcode
        );
        assert!(!cpu.flags.subtract);
        assert!(!cpu.flags.half_carry);
        assert!(!cpu.flags.carry);
        assert_eq!(cpu.flags.zero, expected == 0);
    }
}

#[test]
#[allow(non_snake_case)]
fn test_0xB6() { //OR A, [HL]
    let mut cpu = prepare_cpu();

    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;
    cpu.registers.a = 0b01010010;
    cpu.memory[0xC001] = 0b00010011;

    cpu.execute(0xB6);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0b01010011, cpu.registers.a);
    assert!(!cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);
    assert!(!cpu.flags.carry);
    assert!(!cpu.flags.zero);
}

const CP_A_R_START: u8 = 0xB8;
#[test]
fn test_register_cp_to_a() { //CP A, r
    let mut cpu = prepare_cpu();

    for opcode in CP_A_R_START..0xC0 {
        let expected = {
            let reg = get_register(&mut cpu, (opcode - CP_A_R_START) as i32);

            if let Option::None = reg {
                continue;
            }

            let reg = reg.unwrap();
            *reg = rand::random::<u8>();
            *reg
        };

        cpu.registers.a = TO_SUB;
        cpu.execute(opcode);

        if opcode == 0xBF { //CP A, A will always result in zero and the same flags
            assert_eq!(TO_SUB, cpu.registers.a, "executing 0xBF"); //CP is like SUB but doesn't change the value in a
            assert!(cpu.flags.zero, "executing 0xBF");
            assert!(cpu.flags.subtract, "executing 0xBF");
            assert!(!cpu.flags.half_carry, "executing 0xBF");
            assert!(!cpu.flags.carry, "executing 0xBF");

            continue;
        }

        let result = TO_SUB.wrapping_sub(expected);
        assert_eq!(cpu.flags.zero, result == 0, "executing {:#02x}", opcode);
        assert_eq!(cpu.flags.subtract, true, "executing {:#02x}", opcode);
        assert_eq!(cpu.flags.half_carry, is_half_carry_subtract(cpu.registers.a, expected), "executing {:#02x}", opcode);
        assert_eq!(cpu.flags.carry, expected > TO_SUB, "executing {:#02x}", opcode);
    }
}

#[test]
#[allow(non_snake_case)]
fn test_0xBE() { //CP A, [HL]
    let mut cpu = prepare_cpu();

    cpu.registers.a = 0x0A;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;
    cpu.memory[0xC001] = 0x02;

    cpu.execute(0xBE);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x0A, cpu.registers.a);
    assert!(cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);
    assert!(!cpu.flags.carry);
    assert!(!cpu.flags.zero);

    cpu.registers.a = 0x0A;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;
    cpu.memory[0xC001] = 0x0A;

    cpu.execute(0xBE);

    assert_eq!(0x0A, cpu.registers.a);
    assert!(cpu.flags.subtract);
    assert!(!cpu.flags.half_carry);
    assert!(!cpu.flags.carry);
    assert!(cpu.flags.zero);
}

#[test]
#[allow(non_snake_case)]
fn test_0xC0() { //RET NZ
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0x03;
    cpu.memory[0x04] = 0xC0;
    cpu.memory[0x03] = 0x01;
    cpu.flags.zero = true;

    cpu.execute(0xC0);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x03, cpu.registers.stack_pointer);

    cpu.flags.zero = false;
    cpu.execute(0xC0);

    assert_eq!(0xC001, cpu.registers.program_counter);
    assert_eq!(0x05, cpu.registers.stack_pointer);
}

#[test]
#[allow(non_snake_case)]
fn test_0xC1() { //POP BC
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0x03;
    cpu.memory[0x04] = 0xC0;
    cpu.memory[0x03] = 0x01;

    cpu.execute(0xC1);

    assert_eq!(0x05, cpu.registers.stack_pointer);
    assert_eq!(0xC001, cpu.registers.bc());
}

#[test]
#[allow(non_snake_case)]
fn test_0xC2() { //JP NZ a16
    let mut cpu = prepare_cpu();

    cpu.flags.zero = true;
    cpu.execute_with_args(0xC2, Some(vec![0xC0, 0x01]));

    assert_eq!(3, cpu.registers.program_counter);

    cpu.flags.zero = false;
    cpu.execute_with_args(0xC2, Some(vec![0xC0, 0x01]));

    assert_eq!(0xC001, cpu.registers.program_counter);
}

#[test]
#[allow(non_snake_case)]
fn test_0xC3() { //JP a16
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0xC3, Some(vec![0xC0, 0x01]));

    assert_eq!(0xC001, cpu.registers.program_counter);
}

#[test]
#[allow(non_snake_case)]
fn test_0xC4() { //CALL NZ, a16
    let mut cpu = prepare_cpu();

    cpu.flags.zero = true;
    cpu.registers.program_counter = 0xA034;
    cpu.registers.stack_pointer = 0x05;
    cpu.execute_with_args(0xC4, Some(vec![0xC0, 0x01]));

    assert_eq!(0xA037, cpu.registers.program_counter);
    assert_eq!(0x05, cpu.registers.stack_pointer);
    assert_eq!(0x00, cpu.memory[0x04]);
    assert_eq!(0x00, cpu.memory[0x03]);

    cpu.flags.zero = false;
    cpu.registers.program_counter = 0xA034;
    cpu.registers.stack_pointer = 0x05;
    cpu.execute_with_args(0xC4, Some(vec![0xC0, 0x01]));

    assert_eq!(0xC001, cpu.registers.program_counter);
    assert_eq!(0x03, cpu.registers.stack_pointer);
    assert_eq!(0xA0, cpu.memory[0x04]);
    assert_eq!(0x34 + 3, cpu.memory[0x03]); //+3 to account for the instruction and two operands
}

#[test]
#[allow(non_snake_case)]
fn test_0xC5() { //PUSH BC
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0x05;
    cpu.registers.b = 0xC0;
    cpu.registers.c = 0x01;

    cpu.execute(0xC5);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x03, cpu.registers.stack_pointer);
    assert_eq!(0xC0, cpu.memory[0x04]);
    assert_eq!(0x01, cpu.memory[0x03]);
}

#[test]
#[allow(non_snake_case)]
fn test_0xC6() { //ADD A, n8
    let mut cpu = prepare_cpu();

    cpu.registers.a = 0x05;
    cpu.execute_with_args(0xC6, Some(vec![0x05]));

    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(0x0A, cpu.registers.a);
}

#[test]
#[allow(non_snake_case)]
fn test_0xC8() { //RET Z
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0x03;
    cpu.memory[0x04] = 0xC0;
    cpu.memory[0x03] = 0x01;
    cpu.flags.zero = false;

    cpu.execute(0xC8);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x03, cpu.registers.stack_pointer);

    cpu.flags.zero = true;
    cpu.execute(0xC8);

    assert_eq!(0xC001, cpu.registers.program_counter);
    assert_eq!(0x05, cpu.registers.stack_pointer);
}

#[test]
#[allow(non_snake_case)]
fn test_0xC9() { //RET
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0x03;
    cpu.memory[0x04] = 0xC0;
    cpu.memory[0x03] = 0x01;

    cpu.execute(0xC9);

    assert_eq!(0xC001, cpu.registers.program_counter);
    assert_eq!(0x05, cpu.registers.stack_pointer);
}

#[test]
#[allow(non_snake_case)]
fn test_0xCA() { //JP Z a16
    let mut cpu = prepare_cpu();

    cpu.flags.zero = false;
    cpu.execute_with_args(0xCA, Some(vec![0xC0, 0x01]));

    assert_eq!(3, cpu.registers.program_counter);

    cpu.flags.zero = true;
    cpu.execute_with_args(0xCA, Some(vec![0xC0, 0x01]));

    assert_eq!(0xC001, cpu.registers.program_counter);
}

#[test]
#[allow(non_snake_case)]
fn test_0xCC() { //CALL Z, a16
    let mut cpu = prepare_cpu();

    cpu.flags.zero = false;
    cpu.registers.program_counter = 0xA034;
    cpu.registers.stack_pointer = 0x05;
    cpu.execute_with_args(0xCC, Some(vec![0xC0, 0x01]));

    assert_eq!(0xA037, cpu.registers.program_counter);
    assert_eq!(0x05, cpu.registers.stack_pointer);
    assert_eq!(0x00, cpu.memory[0x04]);
    assert_eq!(0x00, cpu.memory[0x03]);

    cpu.flags.zero = true;
    cpu.registers.program_counter = 0xA034;
    cpu.registers.stack_pointer = 0x05;
    cpu.execute_with_args(0xCC, Some(vec![0xC0, 0x01]));

    assert_eq!(0xC001, cpu.registers.program_counter);
    assert_eq!(0x03, cpu.registers.stack_pointer);
    assert_eq!(0xA0, cpu.memory[0x04]);
    assert_eq!(0x34 + 3, cpu.memory[0x03]); //+3 to account for the instruction and two operands
}

#[test]
#[allow(non_snake_case)]
fn test_0xCD() { //CALL a16
    let mut cpu = prepare_cpu();

    cpu.registers.program_counter = 0xA034;
    cpu.registers.stack_pointer = 0x05;
    cpu.execute_with_args(0xCD, Some(vec![0xC0, 0x01]));

    assert_eq!(0xC001, cpu.registers.program_counter);
    assert_eq!(0x03, cpu.registers.stack_pointer);
    assert_eq!(0xA0, cpu.memory[0x04]);
    assert_eq!(0x34 + 3, cpu.memory[0x03]); //+3 to account for the instruction and two operands
}

#[test]
#[allow(non_snake_case)]
fn test_rst_instructions() { //RST vec
    let instructions: HashMap<u8, u8> = HashMap::from([
        (0xC7, 0x00),
        (0xD7, 0x10),
        (0xE7, 0x20),
        (0xF7, 0x30),
        (0xCF, 0x08),
        (0xDF, 0x18),
        (0xEF, 0x28),
        (0xFF, 0x38)
    ]);

    for (opcode, vector) in instructions.iter() {
        let mut cpu = prepare_cpu();

        cpu.registers.program_counter = 0xA034;
        cpu.registers.stack_pointer = 0x05;
        cpu.execute(*opcode);

        assert_eq!(to16_bit(*vector, 0x00), cpu.registers.program_counter, "executing {:#02x}", opcode);
        assert_eq!(0x03, cpu.registers.stack_pointer, "executing {:#02x}", opcode);
        assert_eq!(0xA0, cpu.memory[0x04], "executing {:#02x}", opcode);
        assert_eq!(0x34 + 3, cpu.memory[0x03], "executing {:#02x}", opcode); //+3 to account for the instruction and two operands
    }
}

#[test]
#[allow(non_snake_case)]
fn test_0xD0() { //RET NC
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0x03;
    cpu.memory[0x04] = 0xC0;
    cpu.memory[0x03] = 0x01;
    cpu.flags.carry = true;

    cpu.execute(0xD0);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x03, cpu.registers.stack_pointer);

    cpu.flags.carry = false;
    cpu.execute(0xD0);

    assert_eq!(0xC001, cpu.registers.program_counter);
    assert_eq!(0x05, cpu.registers.stack_pointer);
}

#[test]
#[allow(non_snake_case)]
fn test_0xD1() { //POP DE
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0x03;
    cpu.memory[0x04] = 0xC0;
    cpu.memory[0x03] = 0x01;

    cpu.execute(0xD1);

    assert_eq!(0x05, cpu.registers.stack_pointer);
    assert_eq!(0xC001, cpu.registers.de());
}

#[test]
#[allow(non_snake_case)]
fn test_0xD2() { //JP NC a16
    let mut cpu = prepare_cpu();

    cpu.flags.carry = true;
    cpu.execute_with_args(0xD2, Some(vec![0xC0, 0x01]));

    assert_eq!(3, cpu.registers.program_counter);

    cpu.flags.carry = false;
    cpu.execute_with_args(0xD2, Some(vec![0xC0, 0x01]));

    assert_eq!(0xC001, cpu.registers.program_counter);
}

#[test]
#[allow(non_snake_case)]
fn test_0xD4() { //CALL NC, a16
    let mut cpu = prepare_cpu();

    cpu.flags.carry = true;
    cpu.registers.program_counter = 0xA034;
    cpu.registers.stack_pointer = 0x05;
    cpu.execute_with_args(0xD4, Some(vec![0xC0, 0x01]));

    assert_eq!(0xA037, cpu.registers.program_counter);
    assert_eq!(0x05, cpu.registers.stack_pointer);
    assert_eq!(0x00, cpu.memory[0x04]);
    assert_eq!(0x00, cpu.memory[0x03]);

    cpu.flags.carry = false;
    cpu.registers.program_counter = 0xA034;
    cpu.registers.stack_pointer = 0x05;
    cpu.execute_with_args(0xD4, Some(vec![0xC0, 0x01]));

    assert_eq!(0xC001, cpu.registers.program_counter);
    assert_eq!(0x03, cpu.registers.stack_pointer);
    assert_eq!(0xA0, cpu.memory[0x04]);
    assert_eq!(0x34 + 3, cpu.memory[0x03]); //+3 to account for the instruction and two operands
}

#[test]
#[allow(non_snake_case)]
fn test_0xD5() { //PUSH DE
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0x05;
    cpu.registers.d = 0xC0;
    cpu.registers.e = 0x01;

    cpu.execute(0xD5);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x03, cpu.registers.stack_pointer);
    assert_eq!(0xC0, cpu.memory[0x04]);
    assert_eq!(0x01, cpu.memory[0x03]);
}

#[test]
#[allow(non_snake_case)]
fn test_0xD6() { //SUB A, n8
    let mut cpu = prepare_cpu();

    cpu.registers.a = 0x0A;
    cpu.execute_with_args(0xD6, Some(vec![0x05]));

    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(0x05, cpu.registers.a);
}

#[test]
#[allow(non_snake_case)]
fn test_0xD8() { //RET C
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0x03;
    cpu.memory[0x04] = 0xC0;
    cpu.memory[0x03] = 0x01;
    cpu.flags.carry = false;

    cpu.execute(0xD8);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x03, cpu.registers.stack_pointer);

    cpu.flags.carry = true;
    cpu.execute(0xD8);

    assert_eq!(0xC001, cpu.registers.program_counter);
    assert_eq!(0x05, cpu.registers.stack_pointer);
}

#[test]
#[allow(non_snake_case)]
fn test_0xDA() { //JP C a16
    let mut cpu = prepare_cpu();

    cpu.flags.carry = false;
    cpu.execute_with_args(0xDA, Some(vec![0xC0, 0x01]));

    assert_eq!(3, cpu.registers.program_counter);

    cpu.flags.carry = true;
    cpu.execute_with_args(0xDA, Some(vec![0xC0, 0x01]));

    assert_eq!(0xC001, cpu.registers.program_counter);
}

#[test]
#[allow(non_snake_case)]
fn test_0xDC() { //CALL C, a16
    let mut cpu = prepare_cpu();

    cpu.flags.carry = false;
    cpu.registers.program_counter = 0xA034;
    cpu.registers.stack_pointer = 0x05;
    cpu.execute_with_args(0xDC, Some(vec![0xC0, 0x01]));

    assert_eq!(0xA037, cpu.registers.program_counter);
    assert_eq!(0x05, cpu.registers.stack_pointer);
    assert_eq!(0x00, cpu.memory[0x04]);
    assert_eq!(0x00, cpu.memory[0x03]);

    cpu.flags.carry = true;
    cpu.registers.program_counter = 0xA034;
    cpu.registers.stack_pointer = 0x05;
    cpu.execute_with_args(0xDC, Some(vec![0xC0, 0x01]));

    assert_eq!(0xC001, cpu.registers.program_counter);
    assert_eq!(0x03, cpu.registers.stack_pointer);
    assert_eq!(0xA0, cpu.memory[0x04]);
    assert_eq!(0x34 + 3, cpu.memory[0x03]); //+3 to account for the instruction and two operands
}

#[test]
#[allow(non_snake_case)]
fn test_0xE1() { //POP HL
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0x03;
    cpu.memory[0x04] = 0xC0;
    cpu.memory[0x03] = 0x01;

    cpu.execute(0xE1);

    assert_eq!(0x05, cpu.registers.stack_pointer);
    assert_eq!(0xC001, cpu.registers.hl());
}

#[test]
#[allow(non_snake_case)]
fn test_0xE5() { //PUSH HL
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0x05;
    cpu.registers.h = 0xC0;
    cpu.registers.l = 0x01;

    cpu.execute(0xE5);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x03, cpu.registers.stack_pointer);
    assert_eq!(0xC0, cpu.memory[0x04]);
    assert_eq!(0x01, cpu.memory[0x03]);
}

#[test]
#[allow(non_snake_case)]
fn test_0xE6() { //AND A, n8
    let mut cpu = prepare_cpu();

    cpu.registers.a = 0b00001010;
    cpu.execute_with_args(0xE6, Some(vec![0b00000010]));

    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(0x02, cpu.registers.a);
}

#[test]
#[allow(non_snake_case)]
fn test_0xF1() { //POP AF
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0x03;
    cpu.memory[0x04] = 0xC0;
    cpu.memory[0x03] = 0xA0;

    cpu.execute(0xF1);

    assert_eq!(0x05, cpu.registers.stack_pointer);
    assert_eq!(0xC0A0, cpu.registers.af(&cpu.flags));
    assert!(cpu.flags.zero);
    assert!(!cpu.flags.subtract);
    assert!(cpu.flags.half_carry);
    assert!(!cpu.flags.carry);
}

#[test]
#[allow(non_snake_case)]
fn test_0xF5() { //PUSH AF
    let mut cpu = prepare_cpu();

    cpu.registers.stack_pointer = 0x05;
    cpu.flags.zero = true;
    cpu.flags.half_carry = true;
    cpu.registers.a = 0xC0;

    cpu.execute(0xF5);

    assert_eq!(1, cpu.registers.program_counter);
    assert_eq!(0x03, cpu.registers.stack_pointer);
    assert_eq!(0xC0, cpu.memory[0x04]);
    assert_eq!(0xA0, cpu.memory[0x03]);
}

#[test]
#[allow(non_snake_case)]
fn test_0xF6() { //OR A, n8
    let mut cpu = prepare_cpu();

    cpu.registers.a = 0b00001010;
    cpu.execute_with_args(0xF6, Some(vec![0b00000010]));

    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(0x0A, cpu.registers.a);
}