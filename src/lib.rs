type Address = u16;

#[derive(Default)]
struct Motherboard {
    cpu: Core::SM83,
    memory_map: MemoryMap,
}

struct MemoryMap {
    work_ram: [u8; 0x2000],
}

impl Default for MemoryMap {
    fn default() -> Self {
        Self {
            work_ram: [0; 0x2000],
        }
    }
}

impl MemoryMap {
    fn read_byte_at(&self, address: Address) -> u8 {
        self.work_ram[address as usize]
    }

    fn write_byte_at(&mut self, address: Address, data: u8) {
        self.work_ram[address as usize] = data;
    }
}

mod Core {
    use crate::{Address, Motherboard};

    use bitflags::bitflags;
    bitflags! {
        #[derive(Default)]
        struct Flags: u8 {
            const ZERO = 0b1000_0000;
            const SUBTRACT = 0b0100_0000;
            const HALF_CARRY = 0b0010_0000;
            const CARRY = 0b0001_0000;
        }
    }

    enum Reg8 {
        A,
        F,
        B,
        C,
        D,
        E,
        H,
        L,
    }

    enum Reg16 {
        AF,
        BC,
        DE,
        HL,
    }

    #[derive(Default)]
    struct RegisterFile {
        a: u8,
        f: Flags,
        b: u8,
        c: u8,
        d: u8,
        e: u8,
        h: u8,
        l: u8,
        sp: Address,
        pc: Address,
    }

    impl RegisterFile {
        fn read16(&self, reg: Reg16) -> u16 {
            match reg {
                Reg16::AF => {
                    let a: u16 = self.a.into();
                    let f: u16 = self.f.bits.into();
                    a << 8 | f
                }
                Reg16::BC => {
                    let b: u16 = self.b.into();
                    let c: u16 = self.c.into();
                    b << 8 | c
                }
                Reg16::DE => {
                    let d: u16 = self.d.into();
                    let e: u16 = self.e.into();
                    d << 8 | e
                }
                Reg16::HL => {
                    let h: u16 = self.h.into();
                    let l: u16 = self.l.into();
                    h << 8 | l
                }
            }
        }

        fn write16(&mut self, reg: Reg16, word: u16) {
            match reg {
                Reg16::AF => {
                    self.a = (word >> 8) as u8;
                    self.f = Flags::from_bits_truncate(word as u8);
                }
                Reg16::BC => {
                    self.b = (word >> 8) as u8;
                    self.c = word as u8;
                }
                Reg16::DE => {
                    self.d = (word >> 8) as u8;
                    self.e = word as u8;
                }
                Reg16::HL => {
                    self.h = (word >> 8) as u8;
                    self.l = word as u8;
                }
            }
        }
    }

    #[derive(Default)]
    pub struct SM83 {
        registers: RegisterFile,
    }

    impl SM83 {
        fn fetch_byte(&mut self, mb: Motherboard) -> u8 {
            let address = self.registers.pc;
            let byte = mb.memory_map.read_byte_at(address);
            self.registers.pc = address.wrapping_add(1);
            byte
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn we_can_instantiate_cpu() {
            let cpu = SM83::default();
        }
    }
}
