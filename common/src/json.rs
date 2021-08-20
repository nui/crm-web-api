use std::borrow::Cow;
use std::marker::PhantomData;

use serde::Serialize;

type StrCow = Cow<'static, str>;

#[derive(Serialize)]
pub enum ApiJson<T> {
    #[serde(rename = "data")]
    Data(Option<T>),
    #[serde(rename = "error")]
    Error(ErrorCodeMessage),
}

#[derive(Serialize)]
pub struct ErrorCodeMessage {
    code: StrCow,
    message: Option<StrCow>,
}

#[derive(Default)]
pub struct ApiJsonErrorBuilder<T> {
    code: Option<StrCow>,
    message: Option<StrCow>,
    _phantom: PhantomData<T>,
}

const DEFAULT_ERROR_CODE: &str = "500";
const DEFAULT_ERROR_MESSAGE: &str = "Internal server error";

impl<T> ApiJson<T> {
    pub fn error_builder() -> ApiJsonErrorBuilder<T> {
        ApiJsonErrorBuilder::new()
    }
}

impl<T> ApiJsonErrorBuilder<T> {
    pub fn new() -> Self {
        Self {
            code: None,
            message: None,
            _phantom: PhantomData,
        }
    }

    pub fn code(mut self, code: StrCow) -> Self {
        self.code = Some(code);
        self
    }

    pub fn message(mut self, message: StrCow) -> Self {
        self.message = Some(message);
        self
    }

    pub fn build(self) -> ApiJson<T> {
        ApiJson::Error(ErrorCodeMessage {
            code: self.code.unwrap_or_else(|| DEFAULT_ERROR_CODE.into()),
            message: self.message.or_else(|| Some(DEFAULT_ERROR_MESSAGE.into())),
        })
    }
}

impl<T: Serialize> ApiJson<T> {
    pub const NO_CONTENT: Self = ApiJson::Data(None);

    pub fn ok(data: T) -> Self {
        Self::Data(Some(data))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    type UnitApiJson = ApiJson<()>;

    #[derive(Serialize, Clone)]
    struct TestData {
        foo: String,
    }

    #[test]
    fn test_build_ok_data() {
        let data = TestData {
            foo: "bar".to_owned(),
        };

        let json = ApiJson::ok(data);
        let actual = serde_json::to_value(json).unwrap();

        let expect = json!({
            "data": {
                "foo": "bar"
            },
        });

        assert_eq!(actual, expect);
    }

    #[test]
    fn test_build_default_error_message() {
        let json = UnitApiJson::error_builder().build();
        let actual = serde_json::to_value(json).unwrap();
        let expect = json!({
            "error": {
                "code": DEFAULT_ERROR_CODE,
                "message": DEFAULT_ERROR_MESSAGE,
            },
        });
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_build_error_with_message() {
        let json = UnitApiJson::error_builder().message("foo".into()).build();
        let actual = serde_json::to_value(json).unwrap();
        let expect = json!({
            "error": {
                "code": DEFAULT_ERROR_CODE,
                "message": "foo",
            },
        });
        assert_eq!(actual, expect);
    }

    #[test]
    fn test_build_error_with_code_and_message() {
        let json = UnitApiJson::error_builder()
            .code("-200".into())
            .message("bar".into())
            .build();
        let actual = serde_json::to_value(json).unwrap();
        let expect = json!({
            "error": {
                "code": "-200",
                "message": "bar",
            },
        });
        assert_eq!(actual, expect);
    }
}
