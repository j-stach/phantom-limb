
pub fn version() -> String { "phantom-limb v0.0.1 (c) J. Stach\n".to_string() }

pub fn help_please() -> String {
    format!("{}{}", version(),
r#"
$ phantom-limb [dummy] [duration] [socket_address] [neuron_id]+

For `dummy` choose one of the sensors or motors below.
In `duration` specify the number of seconds (u64) for which to run the dummy.
If the dummy is a sensor, `socket_address` is the IPv6 address of the remote target.
For a motor, it is the address of the socket for receiving the neurotransmission signal.
List one or more `neuron_id` for the dummy sensor/motor to transmit/handle, respectively.
(See cajal::neuron::soma::NeuronId)

SENSORS:
white-noise     Tests up to 256 paths by sending randomly-generated signal data.

MOTORS:
read-and-weep   Behavioral sink that prints activation sequence to STDOUT.

"#)
}
