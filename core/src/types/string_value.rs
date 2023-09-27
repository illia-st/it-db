use super::ValueBuilder;

pub struct StringValue {
    value: String,
}
#[derive(Default)]
pub struct StringValueBuilder {
    row_value: Option<String>,
}

impl ValueBuilder for StringValueBuilder {
    type Value = StringValue;
    type RowValueType = String;
    fn validate(&self) -> Result<Self::RowValueType, String> {
        // TODO: think about removing duplication here
        match &self.row_value {
            Some(value) => Ok(value.trim().to_owned()),
            None => Err("the value is expected to be set up".to_string())
        }
    }

    fn build(self) -> Result<Self::Value, String> {
        match self.validate() {
            Ok(value) => Ok(StringValue::new(value)),
            Err(err) => Err(err)
        }
    }

    fn with_raw_value(mut self, raw_value: String) -> Self {
        self.row_value = Some(raw_value);
        self
    }
}

impl StringValue {
    fn new(value: String) -> Self {
        Self { value }
    }
    pub fn builder() -> StringValueBuilder {
        StringValueBuilder::default()
    }
    pub fn get_value(&self) -> &str { self.value.as_str() }
}

#[cfg(test)]
mod tests {
    use crate::types::string_value::StringValue;
    use super::ValueBuilder;
    #[test]
    fn test_string_creation_success_1() {
        // https://www.youtube.com/watch?v=hB-WHw6uMWg
        const RAW_VALUE: &str = "throw away your television";
        const EXPECTED_RESULT: &str = "throw away your television";
        let builder = StringValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_ok());
        let value = builder.build();
        assert!(value.is_ok());
        assert_eq!(value.unwrap().get_value(), EXPECTED_RESULT);
    }
    #[test]
    fn test_string_creation_success_2() {
        // https://www.youtube.com/watch?v=Q9OZpSgiLGQ
        const RAW_VALUE: &str = "21st Century";
        const EXPECTED_RESULT: &str = "21st Century";
        let builder = StringValue::builder()
            .with_raw_value(RAW_VALUE.to_string());
        assert!(builder.validate().is_ok());
        let value = builder.build();
        assert!(value.is_ok());
        assert_eq!(value.unwrap().get_value(), EXPECTED_RESULT);
    }
    #[test]
    fn test_string_creation_failure() {
        let builder = StringValue::builder();
        assert!(builder.validate().is_err());
        let value = builder.build();
        assert!(value.is_err());
    }
}