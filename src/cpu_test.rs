use super::CPU;

#[test]
fn test_0x22() { //LD (HL+), A
    let mut cpu = CPU::new();

    cpu.execute_with_args(0x3E, Option::Some(vec![100])); //load A with 100
    cpu.execute_with_args(0x21, Option::Some(vec![0xC0, 0x01])); //load HL with c001
    cpu.execute(0x22); //load A into (HL) and increment HL

    assert_eq!(cpu.registers.a, 100);
    assert_eq!(cpu.registers.hl(), 0xC002);
}

#[test]
fn test_0x32() { //LD (HL-), A
    let mut cpu = CPU::new();

    cpu.execute_with_args(0x3E, Option::Some(vec![100])); //load A with 100
    cpu.execute_with_args(0x21, Option::Some(vec![0xC0, 0x01])); //load HL with c001
    cpu.execute(0x32); //load A into (HL) and increment HL

    assert_eq!(cpu.registers.a, 100);
    assert_eq!(cpu.registers.hl(), 0xC000);
}

#[test]
fn test_0x34() { //INC (HL)
    let mut cpu = CPU::new();

    cpu.execute_with_args(0x3E, Option::Some(vec![100])); //load A with 100
    cpu.execute_with_args(0x21, Option::Some(vec![0xC0, 0x01])); //load HL with c001
    cpu.execute(0x32); //load A into (HL) and increment HL

    assert_eq!(cpu.registers.a, 100);
    assert_eq!(cpu.registers.hl(), 0xC000);
}