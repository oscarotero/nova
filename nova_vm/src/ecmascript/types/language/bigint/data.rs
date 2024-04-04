use crate::ecmascript::{execution::agent::JsError, types::Value};
use num_bigint::BigInt;
// use num_traits::cast::ToPrimitive;

#[derive(Debug, Clone)]
pub struct BigIntHeapData {
    pub(crate) data: BigInt,
}

impl TryInto<f64> for BigIntHeapData {
    type Error = JsError;

    fn try_into(self) -> Result<f64, Self::Error> {
        // self.data.to_f64()
        Err(JsError::new(Value::Undefined))
    }
}
