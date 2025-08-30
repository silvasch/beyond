/// `beyond`'s error type.
#[derive(Debug)]
pub enum Error {
    /// The client failed to serialize a request.
    SerializeRequest(serde_json::Error),
    /// The server failed to serialize a response.
    SerializeResponse(serde_json::Error),
    /// The server failed to deserialize a request.
    DeserializeRequest(serde_json::Error),
    /// The client failed to deserialize a response.
    DeserializeResponse(serde_json::Error),

    /// The server failed to decode the base64-form of the request.
    Base64DecodeRequest(base64::DecodeError),
    /// The client failed to decode the base64-form of the response.
    Base64DecodeResponse(base64::DecodeError),

    /// The client failed to launch the SSH process.
    SSHProcessLaunch(std::io::Error),
    /// Something went wrong on the server or the client while
    /// while executing the SSH process.
    /// This could happen if the server refused the connection,
    /// if the server does not have the corresponding binary installed
    /// or the server binary panicked.
    SSHProcessExecute { stderr: String, },

    /// The requested route does not exist.
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
