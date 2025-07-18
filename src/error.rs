
#[derive(Debug, thiserror::Error)]
pub enum BuildError {

    #[error("Failed to create socket: {0}")]
    SocketFailed(#[from] std::io::Error)
}


#[derive(Debug, thiserror::Error)]
pub enum CommunicationError {

    #[error("Failed to communicate with socket: {0}")]
    SocketFailed(#[from] std::io::Error),

    #[error("Couldn't serialize/deserialize message: {0}")]
    SerdeFailed(#[from] Box<bincode::ErrorKind>),

    #[error("Unrecognized NeuronId: {0}")]
    UnrecognizedImpulse(cajal::neuron::NeuronId),

    #[error("Unrecognized trigger from Sensor '{0}'")]
    UnrecognizedTrigger(String)
}

