use cpu::CPU;

pub mod cpu;

fn main() {
    let mut cpu = CPU::new();

    cpu.status();
    cpu.execute_with_args(0x26, Option::Some(vec![0xC0]));
    cpu.execute_with_args(0x2E, Option::Some(vec![0x01]));
    cpu.execute_with_args(0x36, Option::Some(vec![100]));
    cpu.execute(0x34);
    cpu.status();
}
