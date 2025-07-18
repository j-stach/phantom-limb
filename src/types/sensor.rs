
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use std::collections::HashMap;
use std::hash::Hash;

pub use super::NeuronId;


/// Sends some data as a NeuronId to trigger a Complex's Inputs.
/// The frequency of that data's occurrence should form a meaningful signal.
pub struct Sensor<Q: Hash + Eq> {
    pub address: SocketAddr,
    pub socket: UdpSocket,
    pub spectrum: HashMap<Q, NeuronId>,
} impl<Q: Hash + Eq> Sensor<Q> {

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
    pub fn add_receptor(&mut self, quantum: Q, nid: NeuronId) {
        self.spectrum.insert(quantum, nid);
    }

    /// Connect to a remote socket. Remember to ensure that the corresponding Input
    /// can handle all NeuronIds that will be sent by this sensor.
    pub async fn connect(&self, remote: &SocketAddr) -> Result<(), anyhow::Error> {
        self.socket.connect(remote).await?;
        Ok(())
    }

    /// Attempts to send a sensory datum as a neurotransmission impulse.
    pub async fn send_impulse(&self, quantum: &Q) -> Result<(), anyhow::Error> {
        if let Some(nid) = self.spectrum.get(quantum) {
            let nid = bincode::serialize(nid)?;
            self.socket.send(&nid).await?;
            Ok(())
        } else { Err(anyhow::anyhow!("Information not recognized by sensor")) }
    }
}

