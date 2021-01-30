use num::traits::FromPrimitive;

// Bytecode ops.
// TODO: Assign predefined values and sort.
#[repr(u8)]
#[derive(Clone, Debug, PartialEq, FromPrimitive)]
pub(super) enum Op {
    Halt,     // Halt VM
    Return,   // Return from current function
    Constant, // Load constant (i24: constant address)
    Add,      // Add two values
}

pub(super) type ConstantId = usize;

// Ops require fixed number of parameters.
// When reading an op, we try to read all parameters into `OpCall`.
pub(super) enum OpCall {
    Halt,
    Return,
    Constant(ConstantId),
    Add,
}
