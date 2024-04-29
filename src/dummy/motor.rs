
use crate::types::{ Motor, NeuronId };


/// Sink for behavioral output that prints nervous activation sequence to STDOUT.
pub async fn read_and_weep(duration: std::time::Duration, address: std::net::SocketAddr, nids: Vec<NeuronId>)
-> Result<(), anyhow::Error> {
    env_logger::init();
    let mut motor = Motor::new(address).await?;
println!("A");
    for nid in nids {
        motor.add_nerve(&nid, { // BUG
            let now = chrono::Utc::now();
            let timestamp: chrono::DateTime<chrono::Utc> = now.into();
            print!("{} @ {}, ", &nid, timestamp.format("%M:%S.%9f"))
        })
    }
println!("B");

    log::info!("Reading nerve activity at {} with {} signal paths",
             motor.address, motor.nerves.len());

    let mut buffer = vec![0; 100];
    let start = std::time::Instant::now();
println!("C");
    while start.elapsed() < duration {
println!("D");
        //let nid = tokio::time::timeout(std::time::Duration::from_secs(1), motor.recv_impulse(&mut buffer)).await?;
        if let Err(error) = motor.recv_impulse(&mut buffer).await { // TODO: FIX: Hangs here awaiting message, won't time out
            // TODO Error type
            log::warn!("Whoops! Impulse not recognized\n{:?}", error);
println!("Error");
        } else {

println!("Good");
        }
    }

    log::info!("Signal stream from {} read for {} seconds",
             motor.address, start.elapsed().as_secs());

println!("G");
    Ok(())
}

