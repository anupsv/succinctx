use std::fmt::Debug;

use ethers::types::H256;
use plonky2::field::extension::Extendable;
use plonky2::hash::hash_types::RichField;
use plonky2::iop::witness::{Witness, WitnessWrite};

use super::{ByteVariable, BytesVariable, BoolVariable, CircuitVariable, EvmVariable, Variable};
use crate::builder::CircuitBuilder;
use crate::ops::{PartialEq};

/// A variable in the circuit representing a byte32 value.
#[derive(Debug, Clone, Copy)]
pub struct Bytes32Variable(pub BytesVariable<32>);

impl Bytes32Variable {
    pub fn as_slice(&self) -> [ByteVariable; 32] {
        self.0.0
    }
}

impl From<[ByteVariable; 32]> for Bytes32Variable {
    fn from(bytes: [ByteVariable; 32]) -> Self {
        Self(BytesVariable(bytes))
    }
}

impl From<&[ByteVariable]> for Bytes32Variable {
    fn from(bytes: &[ByteVariable]) -> Self {
        let bytes_fixed: [ByteVariable; 32] = bytes.try_into().unwrap();
        Self(BytesVariable(bytes_fixed))
    }
}

impl CircuitVariable for Bytes32Variable {
    type ValueType<F: RichField> = H256;

    fn init<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
    ) -> Self {
        Self(BytesVariable::init(builder))
    }

    fn constant<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        value: Self::ValueType<F>,
    ) -> Self {
        Self(BytesVariable::constant(
            builder,
            value.as_bytes().try_into().unwrap(),
        ))
    }

    fn variables(&self) -> Vec<super::Variable> {
        self.0.variables()
    }

    fn from_variables(variables: &[Variable]) -> Self {
        Self(BytesVariable::from_variables(variables))
    }

    fn get<F: RichField, W: Witness<F>>(&self, witness: &W) -> Self::ValueType<F> {
        let bytes = self.0.get(witness);
        H256::from_slice(&bytes)
    }

    fn set<F: RichField, W: WitnessWrite<F>>(&self, witness: &mut W, value: Self::ValueType<F>) {
        self.0.set(witness, value.0);
    }
}

impl<F: RichField + Extendable<D>, const D: usize> PartialEq<F, D>
    for Bytes32Variable
{
    fn eq(self, rhs: Bytes32Variable, builder: &mut CircuitBuilder<F, D>) -> BoolVariable {
        let mut result = builder.init::<BoolVariable>();
        for i in 0..32 {
            let lhs_byte = self.0.0[i];
            let rhs_byte = rhs.0.0[i];
            let byte_eq = builder.eq(lhs_byte, rhs_byte);
            result = builder.and(result, byte_eq);
        }
        result
    }

}

impl EvmVariable for Bytes32Variable {
    fn encode<F: RichField + Extendable<D>, const D: usize>(
        &self,
        builder: &mut CircuitBuilder<F, D>,
    ) -> Vec<ByteVariable> {
        self.0.encode(builder)
    }

    fn decode<F: RichField + Extendable<D>, const D: usize>(
        builder: &mut CircuitBuilder<F, D>,
        bytes: &[super::ByteVariable],
    ) -> Self {
        Self(BytesVariable::decode(builder, bytes))
    }

    fn encode_value<F: RichField>(value: Self::ValueType<F>) -> Vec<u8> {
        value.as_bytes().to_vec()
    }

    fn decode_value<F: RichField>(bytes: &[u8]) -> Self::ValueType<F> {
        H256::from_slice(bytes)
    }
}
