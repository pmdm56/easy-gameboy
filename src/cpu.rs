use crate::{instruction::{ArithmeticTarget, Instruction}, registers::Registers};

pub struct CPU {
    pub registers: Registers,
}

impl CPU {

    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(), // Chama o construtor de Registers
        }
    }

    fn execute(&mut self, instruction: Instruction){
        match instruction {
            Instruction::ADD(target) => {
                // Identifica o valor a ser somado com o acumulador
                let _value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                };
                // Chama a função no register que realiza a soma e atualiza os flags
            }
        }
    }
}