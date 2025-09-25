use async_graphql::{Scalar, ScalarType, Value, InputValueResult, InputValueError};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug , Serialize, Deserialize)]
pub struct UUID(pub Uuid);

#[Scalar]
impl ScalarType for UUID {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(s) = &value {
            Uuid::parse_str(s)
                .map(UUID)
                .map_err(|_| InputValueError::custom("Invalid UUID"))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
