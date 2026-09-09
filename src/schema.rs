mod string;
#[doc(inline)]
pub use string::StringSchema;

mod number;
#[doc(inline)]
pub use number::NumberSchema;

mod boolean;
#[doc(inline)]
pub use boolean::BooleanSchema;

mod array;
#[doc(inline)]
pub use array::ArraySchema;

mod object;
#[doc(inline)]
pub use object::ObjectSchema;

mod tuple;
#[doc(inline)]
pub use tuple::TupleSchema;

mod union;
#[doc(inline)]
pub use union::UnionSchema;

mod literal;
#[doc(inline)]
pub use literal::LiteralSchema;

mod null;
#[doc(inline)]
pub use null::NullSchema;

mod optional;
#[doc(inline)]
pub use optional::Optional;

use crate::Error;
use crate::Validate;
use crate::ZInput;

pub trait Schema<T = serde_json::Value>: Validate<T> {
	fn parse<I: ZInput>(&self, input: I) -> Result<T, Error> {
		let value = input.to_json_value()?;
		self.validate(&value)
	}
}

impl<S, T> Schema<T> for S where S: Validate<T> {}

pub fn string() -> StringSchema {
	StringSchema::new()
}

pub fn number() -> NumberSchema {
	NumberSchema::new()
}

pub fn boolean() -> BooleanSchema {
	BooleanSchema::new()
}

pub fn array<S>(element: S) -> ArraySchema<S> {
	ArraySchema::new(element)
}

pub fn object() -> ObjectSchema {
	ObjectSchema::new()
}

pub fn tuple<T>(items: T) -> TupleSchema<T> {
	TupleSchema::new(items)
}

pub fn union() -> UnionSchema {
	UnionSchema::new()
}


pub fn literal(value: impl Into<smol_str::SmolStr>) -> LiteralSchema {
	LiteralSchema::string(value)
}

pub fn null() -> NullSchema {
	NullSchema::new()
}

pub fn optional<S, T>(inner: S) -> Optional<S, T>
where
	S: Validate<T>,
{
	Optional::new(inner)
}
