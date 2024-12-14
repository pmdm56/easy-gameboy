pub enum Instruction {
    ADD(ArithmeticTarget)
}

/// Enum que representa os alvos de operações aritméticas
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}