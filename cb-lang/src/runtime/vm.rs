use super::chunk::{Chunk, ConstantId};
use super::op::{Op, OpCall};
use crate::value::{RcValue, Value};

pub(super) type VmPointer = usize;

#[derive(Debug, Clone)]
pub(super) enum VmError {
    PointerOutOfBounds { ip: VmPointer },
    InvalidOp { ip: VmPointer },
    ConstantNotFound { ip: VmPointer },
    StackEmpty { ip: VmPointer },
    TypeMismatch { ip: VmPointer },
}

pub(super) struct Vm {
    chunk: Chunk,
    ip: VmPointer,
    stack: Vec<RcValue>,
}

impl Vm {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<RcValue, VmError> {
        loop {
            let op_call = self.read_op()?;

            match op_call {
                OpCall::Halt => return Ok(self.pop()?),
                OpCall::Constant(address) => {
                    if address as usize >= self.chunk.constants.len() {
                        return Err(VmError::ConstantNotFound { ip: self.ip });
                    }
                    self.stack
                        .push(self.chunk.constants[address as usize].clone());
                }
                OpCall::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;

                    let result = match (&*a, &*b) {
                        (Value::Int(a), Value::Int(b)) => RcValue::new(Value::Int(a + b)),
                        _ => return Err(VmError::TypeMismatch { ip: self.ip }),
                    };

                    self.stack.push(result);
                }
                OpCall::Return => {}
            }
        }
    }

    fn pop(&mut self) -> Result<RcValue, VmError> {
        match self.stack.pop() {
            Some(value) => return Ok(value.clone()),
            None => {
                return Err(VmError::StackEmpty { ip: self.ip });
            }
        }
    }

    fn read_op(&mut self) -> Result<OpCall, VmError> {
        let op = self.chunk.read_op(self.ip)?;

        self.ip += 1;

        match op {
            Op::Halt => Ok(OpCall::Halt),
            Op::Constant => {
                let arg1 = self.chunk.read_u32(self.ip)?;
                self.ip += 4;

                Ok(OpCall::Constant(arg1 as ConstantId))
            }
            Op::Add => Ok(OpCall::Add),
            Op::Return => Ok(OpCall::Return),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

        let mut vm = Vm::new(chunk);

        let value_result = Value::Int(3);

        assert_eq!(*vm.run().unwrap(), value_result);
    }
}
