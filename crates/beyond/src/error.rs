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

    /// Failed to create an SSH session.
    SSHSessionCreate,
    /// Failed to the the host for the SSH session.
    SSHSetHost(ssh::Error),
    /// Failed to parse the local SSH config.
    SSHConfigParse(ssh::Error),
    /// Failed to connect to the server over SSH.
    SSHConnect(ssh::Error),
    /// Failed to authenticate on the server.
    SSHAuth(ssh::Error),
    /// Failed to create the SSH channel to execute the command.
    SSHChannelCreate(ssh::Error),
    /// Failed to open the SSH channel to execute the command.
    SSHChannelOpen(ssh::Error),
    /// Failed to execute the command over SSH.
    SSHExecute(ssh::Error),
    /// The command was stopped by a signal on the server.
    SSHCommandStoppedBySignal,
    /// Failed to read stdout from SSH.
    SSHReadStdout(std::io::Error),
    /// Failed to read stderr from SSH.
    SSHReadStderr(std::io::Error),

    /// The requested route does not exist.
    InvalidRoute { route_name: String },
    /// The server component is not installed on the server.
    ServerComponentNotInstalled,
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

            Error::SSHSessionCreate => write!(f, "failed to create the ssh session"),
            Error::SSHSetHost(e) => write!(f, "failed to set the host for ssh: {}", e),
            Error::SSHConfigParse(e) => write!(f, "failed to parse the ssh config: {}", e),
            Error::SSHConnect(e) => write!(f, "ssh failed to connect: {}", e),
            Error::SSHAuth(e) => write!(f, "ssh authentication failed: {}", e),
            Error::SSHChannelCreate(e) => write!(f, "failed to create an ssh channel: {}", e),
            Error::SSHChannelOpen(e) => write!(f, "failed to open the session on the ssh channel: {}", e),
            Error::SSHExecute(e) => write!(f, "failed to execute the command over ssh: {}", e),
            Error::SSHCommandStoppedBySignal => write!(f, "the command executed over ssh was stopped by a signal"),
            Error::SSHReadStdout(e) => write!(f, "failed to read stdout over ssh: {}", e),
            Error::SSHReadStderr(e) => write!(f, "failed to read stderr over ssh: {}", e),

            Error::InvalidRoute { route_name } => write!(f, "'{}' is not a valid route", route_name),
            Error::ServerComponentNotInstalled => write!(f, "the server component is not installed on the server"),
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

            Error::SSHSessionCreate => None,
            Error::SSHSetHost(e) => Some(e),
            Error::SSHConfigParse(e) => Some(e),
            Error::SSHConnect(e) => Some(e),
            Error::SSHAuth(e) => Some(e),
            Error::SSHChannelCreate(e) => Some(e),
            Error::SSHChannelOpen(e) => Some(e),
            Error::SSHExecute(e) => Some(e),
            Error::SSHCommandStoppedBySignal => None,
            Error::SSHReadStdout(e) => Some(e),
            Error::SSHReadStderr(e) => Some(e),

            Error::InvalidRoute { route_name: _ } => None,
            Error::ServerComponentNotInstalled => None,
        }
    }
}
