
use std::net::SocketAddr;

use crate::types::{ Motor, NeuronId };

/// Sink for behavioral output that prints nerve activation sequence to STDOUT.
pub async fn read_and_weep(run: &bool, address: SocketAddr, nids: Vec<NeuronId>)
-> Result<(), anyhow::Error> {
    let mut motor = Motor::new(address).await?;
    for nid in nids {
        motor.add_nerve(&nid, {
            let now = chrono::Utc::now();
            let timestamp: chrono::DateTime<chrono::Utc> = now.into();
            // TODO: Logging, errors
            print!("{} @ {}, ", &nid, timestamp.format("%M:%S.%9f"))
        })
    }

    // TODO: Logging, errors
    println!("Reading nerve activity at {} with {} signal paths",
             motor.address, motor.nerves.len());

    let mut buffer = vec![0; 100];
    let start = std::time::Instant::now();
    while *run {
        if let Err(error) = motor.recv_impulse(&mut buffer).await {
            println!("Whoops! Impulse not recognized\n{:?}", error)
        }
    }

    // TODO: Logging, errors
    println!("Signal stream from {} read for {} seconds",
             motor.address, start.elapsed().as_secs());

    Ok(())
}

