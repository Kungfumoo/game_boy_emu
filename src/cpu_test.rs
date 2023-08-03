use super::CPU;

const PROGRAM_COUNTER: u16 = 0;

fn prepare_cpu() -> CPU {
    let mut cpu = CPU::new();
    cpu.registers.program_counter = PROGRAM_COUNTER;

    cpu
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
fn text_0x18() { //JR e8
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
fn text_0x20() { //JR NZ, e8
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
#[allow(non_snake_case)]
fn test_0x3E() { //LD A, u8
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x3E, Option::Some(vec![100])); //load A with 100
    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(cpu.registers.a, 100);
}