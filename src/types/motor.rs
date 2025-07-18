
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use std::collections::HashMap;

pub use crate::NeuronId;


/// Handles the behavioral output of a bionic neural network made with cajal.
/// When it receives a NeuronId, it executes the corresponding function.
pub struct Motor<B: Fn(A) -> R, A, R> {
    pub address: SocketAddr,
    pub socket: UdpSocket,
    pub nerves: HashMap<NeuronId, B>,
    phantom_data: std::marker::PhantomData<(A, R)>
} impl<B: Fn(A) -> R, A, R> Motor<B, A, R> {

    /// Create a motor socket. Use port '0' to have the system assign a port.
    /// The socket address will be recorded in the address field.
    pub async fn new(address: SocketAddr) -> Result<Self, anyhow::Error> {
        let mut motor = Motor {
            address,
            socket: UdpSocket::bind(address).await?,
            nerves: HashMap::new(),
            phantom_data: std::marker::PhantomData
        };
        motor.address = motor.socket.local_addr()?;
        Ok(motor)
    }

    /// Maps a neurotransmission signal to a process to be executed.
    pub fn add_nerve(&mut self, impulse: &NeuronId, behavior: B) {
        self.nerves.insert(impulse.clone(), behavior);
    }

    /// Receives NeuronId messages and executes the corresponding function or closure.
    pub async fn recv_impulse(&self, buffer: &mut [u8], args: A) -> Result<R, anyhow::Error> {
        if let Ok(n_bytes) = self.socket.recv(buffer).await {
            let buff = &buffer[..n_bytes];
            let impulse: NeuronId = bincode::deserialize_from(buff)?;
            if let Some(behavior) = self.nerves.get(&impulse) { return Ok(behavior(args)) }
            else { println!("Impulse '{}' not recognized", impulse) }
        }
        Err(anyhow::anyhow!("Impulse not received, behavior not executed"))
    }
}

