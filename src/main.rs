fn main() {
    println!("Hello, world!");
}

// TODO:
// Read ports and print to stdout
//
// pub type MotorAction = fn() -> bool;
// 
// pub struct Motor {
//     address: SocketAddr,
//     /// UDP Socket for tract communication.
//     socket: UdpSocket,
//     /// Key is neuron id as a vec of bytes, and value is the operation to be executed upon firing.
//     actions: HashMap<Vec<u8>, Vec<MotorAction>>,
// } impl Motor {
// 
//     /// Set up the incoming socket for a tract.
//     pub async fn new(addr: SocketAddr) -> Result<Self, anyhow::Error> {
//         let mut tract = Motor {
//             address: addr,
//             socket: UdpSocket::bind(addr).await?,
//             actions: HashMap::new(),
//         };
//         tract.address = tract.socket.local_addr().expect("Socket was assigned to port");
//         Ok(tract)
//     }
// 
//     /// Listen for incoming neurotransmission signals from corresponding TractOut, then process them.
//     /// Run this function in a dedicated thread to ensure it has priority and to prevent bottlenecks.
//     pub async fn send(&self) { // TEST
//         let mut buffer = vec![0; 100]; // TBD Avoid creating this, move to enclosing function? Or keep for consistency?
//         if let Ok(n_bytes) = self.socket.recv(&mut buffer).await {
//             let nid = &buffer[..n_bytes]; // TBD need conversion from neuron id type to vec u8
//             if let Some(a) = self.actions.get(nid) { a; } // TBD
//         }
//     }
// 
// }
