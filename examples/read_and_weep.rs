
/*
use crate::types::{ Motor, NeuronId };


fn main() {
    // TODO: Read and weep here
}

/// Sink for behavioral output that prints activation sequence to STDOUT.
async fn read_and_weep(
    duration: std::time::Duration, 
    address: std::net::SocketAddr, 
    nids: Vec<NeuronId>
) -> Result<(), anyhow::Error> {

    let mut motor = Motor::new(address).await?;

    for nid in nids {
        let id = nid.clone();
        motor.add_nerve(&nid, move |_| {
            let now = chrono::Utc::now();
            let timestamp: chrono::DateTime<chrono::Utc> = now.into();
            println!("{} @ {}, good", id, timestamp.format("%M:%S.%9f"))
        })
    }

    println!(
        "Reading nerve activity at {} with {} signal paths",
        motor.address, 
        motor.nerves.len()
    );

    let mut buffer = vec![0; 100];
    let start = std::time::Instant::now();

    while start.elapsed() < duration {

        let impulse = tokio::time::timeout(
            std::time::Duration::from_secs(4),
            motor.recv_impulse(&mut buffer, ())
        );

        if let Err(e) = impulse.await {
            println!("\nWhoops! Impulse not detected: {}", e);
        }
    }

    println!(
        "Signal stream at {} read for {} seconds",
        motor.address, 
        start.elapsed().as_secs()
    );

    Ok(())
}

*/
