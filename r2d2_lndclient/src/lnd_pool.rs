pub extern crate r2d2;

use anyhow;
use rust_lndclient::client::LndClient;
use std::error;
use std::error::Error as _StdError;
use std::fmt;

//TODO rewrite error part
#[derive(Debug)]
pub enum Error {
    Other(anyhow::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        #[allow(deprecated)] // `cause` is replaced by `Error:source` in 1.33
        match self.cause() {
            Some(cause) => write!(fmt, "{}: {}", self.description(), cause),
            None => write!(fmt, "{}", self.description()),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Other(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::Other(ref err) => {
                #[allow(deprecated)] // `cause` is replaced by `Error:source` in 1.33
                err.cause()
            }
        }
    }
}
#[derive(Debug)]
pub struct LightningConnectionInfo {
    pub host: String,
    pub cert: String,
    pub macaroon: String,
}

#[derive(Debug)]
pub struct LightningConnectionManager {
    connection_info: LightningConnectionInfo,
}

impl LightningConnectionManager {
    pub fn new(params: LightningConnectionInfo) -> Result<LightningConnectionManager, Error> {
        Ok(LightningConnectionManager {
            connection_info: params,
        })
    }
}

impl r2d2::ManageConnection for LightningConnectionManager {
    type Connection = LndClient;
    type Error = Error;

    fn connect(&self) -> Result<LndClient, Error> {
        let client = LndClient::builder(
            self.connection_info.host.clone(),
            self.connection_info.cert.clone(),
            self.connection_info.macaroon.clone(),
        )
        .map_err(Error::Other)?;
        let res = client.build().map_err(Error::Other);
        res
    }

    fn is_valid(&self, client: &mut LndClient) -> Result<(), Error> {
        // TODO add call to server get_info
        Ok(())
    }

    fn has_broken(&self, conn: &mut LndClient) -> bool {
        false
    }
}
