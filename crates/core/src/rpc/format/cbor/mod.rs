mod convert;

pub use convert::Cbor;

use crate::rpc::request::Request;
use crate::rpc::RpcError;
use crate::sql::Value;
use ciborium::Value as Data;

use super::ResTrait;

pub fn parse_value(val: Vec<u8>) -> Result<Value, RpcError> {
	println!("val {:?}", val);
	let cbor = ciborium::from_reader::<Data, _>(&mut val.as_slice())
		.map_err(|_| RpcError::ParseError)
		.map(Cbor)?;

	println!("cbor {:?}", cbor);
	Value::try_from(cbor).map_err(|v: &str| RpcError::Thrown(v.into()))
}

pub fn req(val: Vec<u8>) -> Result<Request, RpcError> {
	println!("REQ {:?}", val);
	parse_value(val)?.try_into()
}

pub fn res(res: impl ResTrait) -> Result<Vec<u8>, RpcError> {
	// Convert the response into a value
	let val: Value = res.into();
	println!("value 1 {:?}", val);
	let val: Cbor = val.try_into()?;
	println!("value 2 {:?}", val);
	// Create a new vector for encoding output
	let mut res = Vec::new();
	// Serialize the value into CBOR binary data
	ciborium::into_writer(&val.0, &mut res).unwrap();
	// Return the message length, and message as binary
	Ok(res)
}
