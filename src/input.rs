use crate::{Error, ParseError, Result};
use serde_json::Value;
use smol_str::SmolStr;

pub trait ZInput {
	fn to_json_value(&self) -> Result<Value>;
}

impl ZInput for Value {
	fn to_json_value(&self) -> Result<Value> {
		Ok(self.clone())
	}
}

impl ZInput for str {
	fn to_json_value(&self) -> Result<Value> {
		serde_json::from_str(self).map_err(ParseError::from).map_err(Error::from)
	}
}

impl ZInput for String {
	#[inline]
	fn to_json_value(&self) -> Result<Value> {
		self.as_str().to_json_value()
	}
}

impl ZInput for SmolStr {
	#[inline]
	fn to_json_value(&self) -> Result<Value> {
		self.as_str().to_json_value()
	}
}

impl ZInput for [u8] {
	fn to_json_value(&self) -> Result<Value> {
		serde_json::from_slice(self).map_err(ParseError::from).map_err(Error::from)
	}
}

impl ZInput for std::path::Path {
	fn to_json_value(&self) -> Result<Value> {
		let content = std::fs::read_to_string(self).map_err(Error::from)?;
		content.as_str().to_json_value()
	}
}

impl ZInput for std::path::PathBuf {
    #[inline]
    fn to_json_value(&self) -> Result<Value> {
        self.as_path().to_json_value()
    }
}