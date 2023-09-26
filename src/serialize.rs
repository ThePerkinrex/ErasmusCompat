use serde::Serializer;

pub fn serialize_to_string<T, S>(data: &T, ser: S) -> Result<S::Ok, S::Error> where S: Serializer, T: ToString {
	ser.serialize_str(&data.to_string())
}