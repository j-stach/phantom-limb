
use crate::types::{ Motor, NeuronId };


/// Sink for behavioral output that prints nervous activation sequence to STDOUT.
pub async fn read_and_weep(duration: std::time::Duration, address: std::net::SocketAddr, nids: Vec<NeuronId>)
-> Result<(), anyhow::Error> {
    let mut motor = Motor::new(address).await?;
    for nid in nids {
        motor.add_nerve(&nid, {
            let now = chrono::Utc::now();
            let timestamp: chrono::DateTime<chrono::Utc> = now.into();
            print!("{} @ {}, ", &nid, timestamp.format("%M:%S.%9f"))
        })
    }

    log::info!("Reading nerve activity at {} with {} signal paths",
             motor.address, motor.nerves.len());

    let mut buffer = vec![0; 100];
    let start = std::time::Instant::now();
    while start.elapsed() < duration {
        if let Err(error) = motor.recv_impulse(&mut buffer).await {
            // TODO Error type
            log::warn!("Whoops! Impulse not recognized\n{:?}", error)
        }
    }

    log::info!("Signal stream from {} read for {} seconds",
             motor.address, start.elapsed().as_secs());

    Ok(())
}

