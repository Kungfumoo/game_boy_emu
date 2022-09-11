use cpu::CPU;

pub mod cpu;

fn main() {
    let mut cpu = CPU::new();

    cpu.status();
    cpu.execute_with_args(0x3E, Option::Some(vec![100])); //load A with 100
    cpu.execute_with_args(0x21, Option::Some(vec![0xC0, 0x01])); //load HL with c001
    cpu.execute(0x22); //load A into (HL) and increment HL
    cpu.execute(0x32); //load A into (HL) and decrement HL
    cpu.execute(0x34); //increment (HL)
    cpu.status();
}
