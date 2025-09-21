use async_graphql::{Scalar, Value, ScalarType};
use chrono::{DateTime, Utc};

#[derive(Clone, Copy, Debug)]
pub struct DateTimeUtc(pub DateTime<Utc>);

#[Scalar]
impl ScalarType for DateTimeUtc {
    fn parse(value: Value) -> async_graphql::InputValueResult<Self> {
        if let Value::String(s) = &value {
            let dt = s.parse::<DateTime<Utc>>()?;
            Ok(DateTimeUtc(dt))
        } else {
            Err("Invalid DateTime".into())
        }
    }
    fn to_value(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }
}
