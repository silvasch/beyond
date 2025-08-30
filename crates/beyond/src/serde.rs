use base64::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Error;

pub fn encode_request<R: Serialize>(request: R) -> Result<String, Error> {
    let json_request = serde_json::to_string(&request).map_err(Error::SerializeRequest)?;
    let base64_request = BASE64_STANDARD.encode(json_request);
    Ok(base64_request)
}

pub fn encode_response<R: Serialize>(response: R) -> Result<String, Error> {
    let json_response = serde_json::to_string(&response).map_err(Error::SerializeResponse)?;
    let base64_response = BASE64_STANDARD.encode(json_response);
    Ok(base64_response)
}

pub fn decode_request<R: for<'a> Deserialize<'a>>(base64_request: &str) -> Result<R, Error> {
    let json_request = String::from_utf8_lossy(&BASE64_STANDARD.decode(base64_request).map_err(Error::Base64DecodeRequest)?).to_string();
    let request = serde_json::from_str(&json_request).map_err(Error::DeserializeRequest)?;
    Ok(request)
}

pub fn decode_response<R: for<'a> Deserialize<'a>>(base64_response: &str) -> Result<R, Error> {
    let json_response = String::from_utf8_lossy(&BASE64_STANDARD.decode(base64_response).map_err(Error::Base64DecodeResponse)?).to_string();
    let response = serde_json::from_str(&json_response).map_err(Error::DeserializeResponse)?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Request {
        name: String,
    }

    #[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Response {
        message: String,
    }

    #[test]
    fn serde_request_test() {
        let request = Request {
            name: "Bob".to_string(),
        };

        let encoded_request = encode_request(request.clone()).unwrap();
        let decoded_request: Request = decode_request(&encoded_request).unwrap();

        assert_eq!(request, decoded_request);
    }

    #[test]
    fn serde_response_test() {
        let response = Response {
            message: "Hello, Bob!".to_string(),
        };

        let encoded_response = encode_response(response.clone()).unwrap();
        let decoded_response: Response = decode_response(&encoded_response).unwrap();

        assert_eq!(response, decoded_response);
    }
}
