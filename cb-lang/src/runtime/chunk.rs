use super::op::Op;
use super::vm::{VmError, VmPointer};
use crate::value::{RcValue, Value};
use num::traits::FromPrimitive;

pub(super) type ConstantId = usize;

pub(super) struct Chunk {
    pub(super) code: Vec<u8>,
    pub(super) constants: Vec<RcValue>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn write_op(&mut self, op: Op) {
        self.code.push(op as u8);
    }

    pub fn write_u32(&mut self, value: u32) {
        value
            .to_be_bytes()
            .iter()
            .for_each(|byte| self.code.push(*byte));
    }

    pub fn write_constant(&mut self, value: Value) -> ConstantId {
        self.constants.push(RcValue::new(value));
        self.constants.len() - 1
    }

    pub fn read_op(&self, ip: VmPointer) -> Result<Op, VmError> {
        if ip >= self.code.len() {
            return Err(VmError::PointerOutOfBounds { ip });
        }

        let op_code = self.code[ip];

        let op = FromPrimitive::from_u8(op_code);

        match op {
            Some(op) => Ok(op),
            None => Err(VmError::InvalidOp { ip }),
        }
    }

    pub fn read_u32(&self, ip: VmPointer) -> Result<u32, VmError> {
        use std::convert::TryInto;

        if ip >= self.code.len() {
            return Err(VmError::PointerOutOfBounds { ip });
        }

        match self.code[ip..ip + 4].try_into() {
            Ok(bytes) => Ok(u32::from_be_bytes(bytes)),
            Err(_) => Err(VmError::PointerOutOfBounds { ip: ip + 3 }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_and_read_chunk_values() {
        let mut chunk = Chunk::new();

        let op = Op::Constant;

        chunk.write_op(op.clone());

        let value = 1234567890;

        chunk.write_u32(value);

        assert_eq!(chunk.read_op(0).unwrap(), op);
        assert_eq!(chunk.read_u32(1).unwrap(), value);
    }

    #[test]
    fn read_constant() {
        let mut chunk = Chunk::new();

        let value = Value::Int(16);

        let cp = chunk.write_constant(value.clone());

        chunk.write_op(Op::Constant);
        chunk.write_u32(cp as u32);
        chunk.write_op(Op::Halt);

        let mut vm = crate::runtime::vm::Vm::new(chunk);

        assert_eq!(*vm.run().unwrap(), value);
    }

    #[test]
    fn add() {
        let mut chunk = Chunk::new();

        let value_a = Value::Int(1);
        let value_b = Value::Int(2);

        let cp_a = chunk.write_constant(value_a.clone());
        let cp_b = chunk.write_constant(value_b.clone());

        chunk.write_op(Op::Constant);
        chunk.write_u32(cp_a as u32);
        chunk.write_op(Op::Constant);
        chunk.write_u32(cp_b as u32);
        chunk.write_op(Op::Add);
        chunk.write_op(Op::Halt);

        let mut vm = crate::runtime::vm::Vm::new(chunk);

        let value_result = Value::Int(3);

        assert_eq!(*vm.run().unwrap(), value_result);
    }
}
