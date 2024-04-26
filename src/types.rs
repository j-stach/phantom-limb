
use std::net::{ IpAddr, SocketAddr };
use tokio::net::UdpSocket;
use std::collections::HashMap;
use std::hash::Hash;
use serde::{ Serialize, Deserialize };


/// Corresponds to the NeuronId from cajal. In the future, just re-export cajal's.
#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct NeuronId {
    structure: String,
    cluster: (usize, usize),
    neuron: (usize, usize)
}

/// Sends some data as a NeuronId to trigger a Complex's Inputs.
/// The frequency of that data's occurrence should form a meaningful signal.
pub struct Sensor<T: Hash + Eq> {
    pub(crate) address: SocketAddr,
    pub(crate) socket: UdpSocket,
    pub(crate) spectrum: HashMap<T, NeuronId>,
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
    pub async fn connect(&self, remote: IpAddr) -> Result<(), anyhow::Error> {
        let remote_socket = SocketAddr::new(remote, self.address.port());
        self.socket.connect(remote_socket).await?;
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
/// When it receives a NeuronId, it executes the corresponding void function.
pub struct Motor<B: Fn()> {
    pub(crate) address: SocketAddr,
    pub(crate) socket: UdpSocket,
    pub(crate) nerves: HashMap<NeuronId, B>,
} impl<B: Fn()> Motor<B> {

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

    /// Maps a neurotransmission signal to a function or closure to be executed.
    pub fn add_nerve(&mut self, impulse: NeuronId, behavior: B) {
        self.nerves.insert(impulse, behavior);
    }

    /// Receives NeuronId messages and executes the corresponding function or closure.
    pub async fn recv_impulse(&self) -> Result<(), anyhow::Error> {
        let mut buffer = vec![0; 100]; // TBD Avoid creating this, move to enclosing function? Or keep for consistency?
        if let Ok(n_bytes) = self.socket.recv(&mut buffer).await {
            // TODO: task spawn?
            let buff = &buffer[..n_bytes];
            let behavior: NeuronId = bincode::deserialize_from(buff)?;
            if let Some(behavior) = self.nerves.get(&behavior) { behavior() }
        }
        Ok(())
    }
}
