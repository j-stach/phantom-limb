
// clockwise/counterclockwise impulses



use crate::types::{ Motor, NeuronId };


/// Sink for behavioral output that prints nervous activation sequence to STDOUT.
pub async fn read_and_weep(duration: std::time::Duration, address: std::net::SocketAddr, nids: Vec<NeuronId>)
-> Result<(), anyhow::Error> {
    let mut motor = Motor::new(address).await?;
    for nid in nids {
        let id = nid.clone();
        motor.add_nerve(&nid, move |_| {
            let now = chrono::Utc::now();
            let timestamp: chrono::DateTime<chrono::Utc> = now.into();
            println!("{} @ {}, good", id, timestamp.format("%M:%S.%9f"))
        })
    }

    println!("Reading nerve activity at {} with {} signal paths",
             motor.address, motor.nerves.len());

    let mut buffer = vec![0; 100];
    let start = std::time::Instant::now();
    while start.elapsed() < duration {
        let impulse = tokio::time::timeout(
            std::time::Duration::from_secs(4),
            motor.recv_impulse(&mut buffer, ())
        );
        if let Err(_) = impulse.await {
            // TODO Error type
            println!("\nWhoops! Impulse not detected");
        }
    }

    println!("Signal stream at {} read for {} seconds",
             motor.address, start.elapsed().as_secs());

    Ok(())
}


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

