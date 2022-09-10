use cpu::CPU;

pub mod cpu;

fn main() {
    let mut cpu = CPU::new();

    cpu.status();
    cpu.execute_with_args(0x06, Option::Some(vec![15]));
    cpu.execute(0x04);
    cpu.status();
}
