
use std::collections::VecDeque;
use std::net::SocketAddr;
use std::time::Duration;

// TODO: use cajal::neuron::NeuronId;
use phantom_limb::id::NeuronId;
use phantom_limb::dummy::{ sensor::white_noise, motor::read_and_weep };
use phantom_limb::error;

mod help;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

println!("1");
    let mut args: VecDeque<String> = std::env::args().collect();
    args.pop_front(); // Discard program name
println!("2");
    let dummy = args.pop_front().ok_or(error::HelpNeeded::please(help::help_please()))?;

println!("3");
    match dummy.as_str() {
        "help"|"--help"|"-h" => { println!("{}", help::help_please()) },
        "version" => { println!("{}", help::version()) },

        "white-noise" => {
println!("whin");
            let (duration, address, nids) = extract_args(args)?;
            white_noise(duration, address, nids).await?;
        },

        "read-and-weep" => {
println!("raw");
            let (duration, address, nids) = extract_args(args)?;
            read_and_weep(duration, address, nids).await?;
        },

        _ => { return Err(error::ArgError::Bad(error::BadArg::dummy(dummy)).into()) }
    }

    Ok(())
}

fn extract_args(mut args: VecDeque<String>)
-> Result<(Duration, SocketAddr, Vec<NeuronId>), error::ArgError> {
println!("a");
    let arg = args.pop_front().ok_or(error::MissingArg::Duration)?;
println!("b");
    let secs = if let Ok(secs) = arg.parse::<u64>() { secs }
        else { return Err(error::BadArg::duration(arg).into()) };
println!("c");

    let duration = std::time::Duration::from_secs(secs);
println!("d");

    let arg = args.pop_front().ok_or(error::MissingArg::Address)?;
println!("e");
    let address = if let Ok(addr) = arg.parse::<std::net::SocketAddr>() { addr }
        else { return Err(error::BadArg::address(arg).into()) };
println!("f");

    let mut nids = Vec::new();
    for arg in args.into_iter() {
        if let Ok(nid) = arg.parse::<NeuronId>() { nids.push(nid) }
        else { return Err(error::BadArg::neuron_id(arg).into()) }
println!("g");
    }

println!("h");
    Ok((duration, address, nids))
}






