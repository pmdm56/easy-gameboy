//Implementar af, bc, de, hl
//af é acumulador e flags (nas flags apenas os bits 4 5 6 7)

/* ┌-> Carry
 ┌-+> Subtraction
 | |
1111 0000
| |
└-+> Zero
  └-> Half Carry */

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
}

pub struct FlagsRegister {
    zero: bool,
    subtract: bool,
    half_carry: bool,
    carry: bool,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagsRegister {
                zero: false,
                subtract: false,
                half_carry: false,
                carry: false,
            },
            h: 0,
            l: 0,
        }
    }

    //af
    fn get_af(&self) -> u16 {
        ((self.a as u16) << 8) | (self.f.to_u8() as u16)
    }

    fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from_u8((value & 0x00FF) as u8)
    }

    //bc
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    //de
    fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    //hl
    fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }
}

impl FlagsRegister {
    /// Converte os flags em um valor de 8 bits
    fn to_u8(&self) -> u8 {
        (self.zero as u8) << 7
            | (self.subtract as u8) << 6
            | (self.half_carry as u8) << 5
            | (self.carry as u8) << 4
    }

    /// Atualiza os flags a partir de um valor de 8 bits
    fn from_u8(value: u8) -> Self {
        FlagsRegister {
            zero: (value & 0b1000_0000) != 0,
            subtract: (value & 0b0100_0000) != 0,
            half_carry: (value & 0b0010_0000) != 0,
            carry: (value & 0b0001_0000) != 0,
        }
    }
}
