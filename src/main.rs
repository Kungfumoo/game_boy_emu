use cpu::CPU;

pub mod cpu;

fn main() {
    let mut cpu = CPU::new();

    cpu.status();
    cpu.execute_with_args(0x0E, Option::Some(vec![32]));
    cpu.status();
    cpu.execute(0x00);
    cpu.execute(0x10);
    cpu.execute(0x37);
    cpu.execute(0x41);
    cpu.status();
}
