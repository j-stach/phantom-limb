
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use std::collections::HashMap;
use std::hash::Hash;
use serde::{ Serialize, Deserialize };


/// Corresponds to the NeuronId from cajal. In the future, just re-export cajal's.
#[derive(Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct NeuronId {
    structure: String,
    cluster: (usize, usize),
    neuron: (usize, usize)
}
impl std::fmt::Display for NeuronId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}:{}/{}:{}",
            self.structure,
            self.cluster.0,
            self.cluster.1,
            self.neuron.0,
            self.neuron.1,
        )
    }
}


/// Sends some data as a NeuronId to trigger a Complex's Inputs.
/// The frequency of that data's occurrence should form a meaningful signal.
pub struct Sensor<T: Hash + Eq> {
    pub address: SocketAddr,
    pub socket: UdpSocket,
    pub spectrum: HashMap<T, NeuronId>,
} impl<T: Hash + Eq> Sensor<T> {

    /// Create a sensor socket. Use port '0' to have the system assign a port.
    /// The socket address will be recorded in the address field.
    pub async fn new(address: SocketAddr) -> Result<Self, anyhow::Error> {
        let mut sensor = Sensor {
            address,
            socket: UdpSocket::bind(address).await?,
            spectrum: HashMap::new()
        };
        sensor.address = sensor.socket.local_addr()?;
        Ok(sensor)
    }

    /// Maps a sensory bit to a new NeuronId.
    pub fn add_receptor(&mut self, quantum: T, nid: NeuronId) {
        self.spectrum.insert(quantum, nid);
    }

    /// Connect to a remote socket. Remember to ensure that the corresponding Input
    /// can handle all NeuronIds that will be sent by this sensor.
    pub async fn connect(&self, remote: &SocketAddr) -> Result<(), anyhow::Error> {
        self.socket.connect(remote).await?;
        Ok(())
    }

    /// Attempts to send a sensory datum as a neurotransmission impulse.
    pub async fn send_impulse(&self, quantum: &T) -> Result<(), anyhow::Error> {
        if let Some(nid) = self.spectrum.get(quantum) {
            let nid = bincode::serialize(nid)?;
            self.socket.send(&nid).await?;
            Ok(())
        } else { Err(anyhow::anyhow!("Information not recognized by sensor")) }
    }
}

/// Handles the behavioral output of a bionic neural network made with cajal.
/// When it receives a NeuronId, it executes the corresponding function.
pub struct Motor {
    pub address: SocketAddr,
    pub socket: UdpSocket,
    pub nerves: HashMap<NeuronId, ()>,
} impl Motor {

    /// Create a motor socket. Use port '0' to have the system assign a port.
    /// The socket address will be recorded in the address field.
    pub async fn new(address: SocketAddr) -> Result<Self, anyhow::Error> {
        let mut motor = Motor {
            address,
            socket: UdpSocket::bind(address).await?,
            nerves: HashMap::new()
        };
        motor.address = motor.socket.local_addr()?;
        Ok(motor)
    }

    /// Maps a neurotransmission signal to a void function or closure to be executed.
    pub fn add_nerve(&mut self, impulse: &NeuronId, behavior: ()) {
        self.nerves.insert(impulse.clone(), behavior);
    }

    /// Receives NeuronId messages and executes the corresponding function or closure.
    pub async fn recv_impulse(&self, buffer: &mut [u8]) -> Result<(), anyhow::Error> {
        if let Ok(n_bytes) = self.socket.recv(buffer).await {
            let buff = &buffer[..n_bytes];
            let impulse: NeuronId = bincode::deserialize_from(buff)?;
            if let Some(behavior) = self.nerves.get(&impulse) { *behavior }
        }
        Ok(())
    }
}
