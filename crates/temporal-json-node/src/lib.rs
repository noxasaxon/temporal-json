#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod encoder {
  use napi::Status;
  use temporal_json::Encoder;

  #[napi]
  fn encode_default_from_json_string(json_string: String) -> napi::Result<String> {
    Encoder::encode_default_from_json_string(&json_string).map_err(|err| {
      napi::Error::new(
        Status::GenericFailure,
        format!("failed to encode from json string, {}", err),
      )
    })
  }

  #[napi]
  fn decode_to_json_string(encoded_string: String) -> napi::Result<String> {
    Encoder::decode_to_json_string(&encoded_string).map_err(|err| {
      napi::Error::new(
        Status::GenericFailure,
        format!("failed to decode from encoded string, {}", err),
      )
    })
  }
}
