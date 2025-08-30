#[derive(Debug)]
pub enum Error {
    SerializeRequest(serde_json::Error),
    SerializeResponse(serde_json::Error),
    DeserializeRequest(serde_json::Error),
    DeserializeResponse(serde_json::Error),

    Base64DecodeRequest(base64::DecodeError),
    Base64DecodeResponse(base64::DecodeError),

    SSHProcessLaunch(std::io::Error),
    SSHProcessExecute { stderr: String, },

    InvalidRoute { route_name: String },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SerializeRequest(e) => write!(f, "failed to serialize the request: {}", e),
            Error::SerializeResponse(e) => write!(f, "failed to serialize the response: {}", e),
            Error::DeserializeRequest(e) => write!(f, "failed to deserialize the request: {}", e),
            Error::DeserializeResponse(e) => write!(f, "failed to deserialize the response: {}", e),

            Error::Base64DecodeRequest(e) => write!(f, "failed to decode the request from base 64: {}", e),
            Error::Base64DecodeResponse(e) => write!(f, "failed to decode the response from base 64: {}", e),

            Error::SSHProcessLaunch(e) => write!(f, "failed to launch the ssh process: {}", e),
            Error::SSHProcessExecute { stderr } => write!(f, "failure while executing the ssh call: {}", stderr),

            Error::InvalidRoute { route_name } => write!(f, "'{}' is not a valid route", route_name),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::SerializeRequest(e) => Some(e),
            Error::SerializeResponse(e) => Some(e),
            Error::DeserializeRequest(e) => Some(e),
            Error::DeserializeResponse(e) => Some(e),

            Error::Base64DecodeRequest(e) => Some(e),
            Error::Base64DecodeResponse(e) => Some(e),

            Error::SSHProcessLaunch(e) => Some(e),
            Error::SSHProcessExecute { stderr: _ } => None,

            Error::InvalidRoute { route_name: _ } => None,
        }
    }
}
