#![feature(nll)]

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate nalgebra;

pub mod packet;
pub mod client;
pub mod server;

use std::io;

#[derive(Debug)]
pub enum Error {
    NetworkErr(io::Error),
    CannotSerialize,
    CannotDeserialize,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::NetworkErr(e)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ClientMode {
    Headless,
    Character,
}
