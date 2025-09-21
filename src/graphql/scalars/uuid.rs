use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UuidScalar(pub Uuid);

#[Scalar]
impl ScalarType for UuidScalar {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(s) = &value {
            Ok(UuidScalar(Uuid::parse_str(s).map_err(|_| InputValueError::custom("Invalid UUID"))?))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
