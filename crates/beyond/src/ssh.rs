use std::{io::Read, process::ExitStatus};

use ssh::Session;

use crate::Error;

pub struct SSH {
    session: Session,
}

impl SSH {
    pub fn new(destination: &str) -> Result<Self, Error> {
        let mut session = Session::new().map_err(|_| Error::SSHSessionCreate)?;
        session.set_host(destination).map_err(Error::SSHSetHost)?;
        session.parse_config(None).map_err(Error::SSHConfigParse)?;
        session.connect().map_err(Error::SSHConnect)?;
        session
            .userauth_publickey_auto(None)
            .map_err(Error::SSHAuth)?;

        Ok(Self { session })
    }

    pub fn execute(
        &mut self,
        command: &str,
    ) -> Result<std::process::Output, Error> {
        let mut channel = self
            .session
            .channel_new()
            .map_err(Error::SSHChannelCreate)?;
        channel.open_session().map_err(Error::SSHChannelOpen)?;
        channel.request_exec(command.as_bytes()).map_err(Error::SSHExecute)?;
        channel.send_eof().map_err(Error::SSHExecute)?;

        let raw_exit_status = channel.get_exit_status().ok_or(Error::SSHCommandStoppedBySignal)?;
        let exit_status;
        if cfg!(unix) {
            use std::os::unix::process::ExitStatusExt;
            exit_status  = ExitStatus::from_raw(raw_exit_status);
        } else {
            unimplemented!()
        }

        let mut stdout = vec![];
        channel.stdout().read_to_end(&mut stdout).map_err(Error::SSHReadStdout)?;

        let mut stderr = vec![];
        channel.stderr().read_to_end(&mut stderr).map_err(Error::SSHReadStdout)?;

        Ok(std::process::Output {
            status: exit_status,
            stdout,
            stderr,
        })
    }
}
