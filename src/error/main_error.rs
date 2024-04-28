
use thiserror::Error;
use std::fmt::{ Display, Error, Formatter };

// TODO: Clarify expected format for misparsed strings


#[derive(Error, Debug)]
pub enum ArgError {
    Bad(#[from] BadArg),
    Missing(#[from] MissingArg)
}
impl Display for ArgError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Failed to parse arguments:")
    }
}

#[derive(Error, Debug)]
pub enum BadArg {
    #[error("Unrecognized dummy routine: {0}")]
    Dummy(String),
    #[error("Invalid duration: Unable to parse {0} as u64")]
    Duration(String),
    #[error("Invalid socket: Unable to parse {0} as IPv6 address")]
    Address(String),
    #[error("Invalid id: Unable to parse {0} as NeuronId")]
    NeuronId(String),
} impl BadArg {
    pub fn dummy(bad_arg: String) -> Self { Self::Dummy(bad_arg) }
    pub fn duration(bad_arg: String) -> Self { Self::Duration(bad_arg) }
    pub fn address(bad_arg: String) -> Self { Self::Address(bad_arg) }
    pub fn neuron_id(bad_arg: String) -> Self { Self::NeuronId(bad_arg) }
}

#[derive(Error, Debug)]
pub enum MissingArg {
    Duration,
    Address,
    NeuronId
}
impl Display for MissingArg {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let expected_arg = match self {
            Self::Duration => "duration",
            Self::Address => "socket address",
            Self::NeuronId => "NeuronId"
        };
        write!(f, "Expected {} argument, none found", expected_arg)
    }
}
impl MissingArg {
    pub fn duration() -> Self { Self::Duration }
    pub fn address() -> Self { Self::Address }
    pub fn neuron_id() -> Self { Self::NeuronId }
}

#[derive(Error, Debug)]
#[error("Help documentation\n{help:}")]
pub struct HelpNeeded { help: String }
impl HelpNeeded {
    pub fn please(help: String) -> Self { HelpNeeded { help }}
}
