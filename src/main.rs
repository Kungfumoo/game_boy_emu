use cpu::CPU;

pub mod cpu;

fn main() {
    let mut cpu = CPU::new();

    cpu.status();
    cpu.execute_with_args(0x06, Option::Some(vec![0xC0]));
    cpu.execute_with_args(0x0E, Option::Some(vec![0x01]));
    cpu.execute_with_args(0x3E, Option::Some(vec![42]));
    cpu.execute(0x02);
    cpu.status();
}
