
use std::net::SocketAddr;
use rand::Rng;

use crate::types::{ Sensor, NeuronId };


/// Tests the connectivity of up to 256 input channels.
// TODO: Logging, errors
pub async fn white_noise(duration: std::time::Duration, remote: SocketAddr, nids: Vec<NeuronId>)
-> Result<(), anyhow::Error> {
    let ip = "127.0.0.1".parse::<std::net::IpAddr>().expect("Parse localhost address");

    let mut sensor = Sensor::<u8>::new(SocketAddr::new(ip, 0)).await?;
    let mut count = 0u8;
    for nid in nids {
        sensor.add_receptor(count, nid);
        count += 1
    }

    sensor.connect(&remote).await?;
    // TODO: Logging, errors
    println!("Sending white noise from {} to {} with {} signal paths",
             sensor.address, &remote, count);

    let mut sensory_environment = rand::thread_rng();
    let start = std::time::Instant::now();

    while start.elapsed() < duration {
        let stimulus: u8 = sensory_environment.gen_range(0..count);
        if let Err(error) = sensor.send_impulse(&stimulus).await {
            // TODO: Logging, errors
            println!("Whoops! Stimulus '{}' didn't send! {:?}", stimulus, error)
        }
    }

    // TODO: Logging, errors
    println!("White noise from {} terminated after {} seconds",
             sensor.address, start.elapsed().as_secs());
    Ok(())
}

