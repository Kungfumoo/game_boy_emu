use cpu::CPU;

pub mod cpu;

fn main() {
    let mut cpu = CPU::new();

    cpu.status();
    cpu.execute(0x00);
    cpu.execute(0x10);
    cpu.execute(0x37);
    cpu.status();
}
