use super::CPU;

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
            expected.wrapping_add(TO_ADD + 1),
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
            TO_SUB.wrapping_sub(expected + 1),
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