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
#[allow(non_snake_case)]
fn test_0x3E() { //LD A, u8
    let mut cpu = prepare_cpu();

    cpu.execute_with_args(0x3E, Option::Some(vec![100])); //load A with 100
    assert_eq!(2, cpu.registers.program_counter);
    assert_eq!(cpu.registers.a, 100);
}