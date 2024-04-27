
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::time::Duration;

use cajal::neuron::NeuronId;
use phantom_limb::dummy::{ sensor::white_noise, motor::read_and_weep };


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let mut args: VecDeque<String> = std::env::args().collect();
    let dummy = args.pop_front().ok_or(anyhow::anyhow!("Help error"))?;
    match dummy.as_str() {
        "help"|"--help"|"-h" => { println!("{}", help_please()) },
        "white-noise" => {
            let (duration, address, nids) = extract_args(args)?;
            white_noise(duration, address, nids).await?;
        },
        "read-and-weep" => {
            let (duration, address, nids) = extract_args(args)?;
            read_and_weep(duration, address, nids).await?;
        },
        _ => { return Err(anyhow::anyhow!("Command not recognized"))}
    }

    Ok(())
}

fn extract_args(mut args: VecDeque<String>) -> Result<(Duration, SocketAddr, Vec<NeuronId>), anyhow::Error> {
    let secs = args.pop_front()
        .ok_or(anyhow::anyhow!("Must provide a duration"))?
        .parse::<u64>()?;
    let duration = std::time::Duration::from_secs(secs);

    let address = args.pop_front()
        .ok_or(anyhow::anyhow!("Must provide a socket address"))?
        .parse::<std::net::SocketAddr>()?;

    let mut nids = Vec::new();
    for arg in args.into_iter() {
        if let Ok(nid) = arg.parse::<NeuronId>() { nids.push(nid) }
    }

    Ok((duration, address, nids))
}

fn help_please() -> String {
r#" phantom-limb v0.0.1 (c) J. Stach

$ phantom-limb [dummy] [duration] [socket_address] [neuron_id]+

For `dummy` choose one of the sensors or motors below.
In `duration` specify the number of seconds (u64) for which to run the dummy.
If the dummy is a sensor, `socket_address` is the IPv6 address of the remote target.
For a motor, it is the address of the socket for receiving the neurotransmission signal.
List the `neuron_id`s for the dummy sensor/motor to transmit/handle, respectively.
(See cajal::neuron::soma::NeuronId)

SENSORS:
white-noise     Tests up to 256 paths by sending randomly-generated signal data.

MOTORS:
read-and-weep   Behavioral sink that prints activation sequence to STDOUT.

"#.to_string()
}






