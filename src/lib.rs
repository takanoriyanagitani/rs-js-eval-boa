use serde_json::Value;

use boa_engine::Context;

use boa_engine::Source;
use boa_engine::{JsError, JsResult, JsValue};

pub struct JsContext(pub Context);

impl JsContext {
    pub fn new() -> Result<Self, JsError> {
        let context = Context::builder().build()?;
        Ok(JsContext(context))
    }

    pub fn evaluate(&mut self, source: &str) -> JsResult<JsValue> {
        let src = Source::from_bytes(source);
        self.0.eval(src)
    }

    pub fn val2json(&mut self, val: &JsValue) -> JsResult<Option<Value>> {
        val.to_json(&mut self.0)
    }
}
