
use std::net::SocketAddr;
use rand::Rng;

use crate::types::{ Sensor, NeuronId };


/// Tests the connectivity of up to 256 input channels.
// TODO: Logging, errors
pub async fn white_noise(run: &bool, remote: SocketAddr, nids: Vec<NeuronId>)
-> Result<(), anyhow::Error> {

    let mut sensor = Sensor::<u8>::new(SocketAddr::new(super::IP, 0)).await?;
    let mut count = 0u8;
    for nid in nids {
        sensor.add_receptor(count, nid);
        count += 1
    }

    sensor.connect(&remote).await?;
    // TODO: Logging, errors
    println!("Sending white noise from {} to {} with {} signal paths",
             sensor.address, &remote, count - 1);
    let start = std::time::Instant::now();

    while *run {
        let mut sensory_environment = rand::thread_rng();
        let stimulus: u8 = sensory_environment.gen_range(0..count);
        if let Err(error) = sensor.send_impulse(&stimulus).await {
            // TODO: Logging, errors
            println!("Whoops! Stimulus '{}' didn't send!\n{:?}", stimulus, error)
        }
    }

    // TODO: Logging, errors
    println!("White noise from {} terminated after {} seconds",
             sensor.address, start.elapsed().as_secs());
    Ok(())
}

