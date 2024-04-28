
# Phantom Limb

*"So we just skirt the hallway sides, a phantom and a fly; Follow the lines and wonder why there's no connection."* <br>
-- The Shins

Crate for simulating sensor inputs and reading behavioral outputs for bionic neural networks (BNN). <br>
Use the CLI tool as an example reference for building signal interfaces for [Cajal][https://cajal.io].

## phantom_limb
A library with types for creating Sensor and Motor interfaces for Cajal BNNs. <br>
```
cargo add phantom_limb
```
**Sensor:**<br>
Sensor types provide an interface for environmental stimuli to be converted into neurotransmission signals.
These can be set up to accomodate a variety of data types, but rely on a frequency-of-occurence model to determine
the relevance of a data pattern. See [TBD][https://cajal.io/theory/signals] for more info.
**Motor:**<br>
Motor types receive behavioral (ouput) signal impulses in the form of NeuronIds, translating them into some practical action.
These serve the same role as nerve endings that drive gland and muscle activity in living animals.

## phantom-limb
A CLI tool for mocking signals to and from Cajal BNNs.
#### How to install:
Requires Rust and Cargo: [Get started][https://www.rust-lang.org/learn/get-started] <br>
Ensure that `.cargo/bin` is in the PATH in order to run executables installed with `cargo install`. <br>
```
$ cargo install phantom-limb
$ phantom-limb help
```
#### How to use:
```
$ phantom-limb [dummy] [duration] [socket_address] [neuron_id]+
```
For `dummy` choose one of the sensors or motors below. <br>
In `duration` specify the number of seconds (in u64) for which to run the dummy. <br>
If the dummy is a sensor, `socket_address` is the IPv6 address of the remote target. <br>
For a motor, it is the address of the socket for receiving the neurotransmission signal. <br>
List one or more `neuron_id` for the dummy sensor/motor to transmit/handle, respectively.
For reference, the expected pattern for a NeuronId is `(\w+)/(\d+):(\d+)/(\d+):(\d+)`. <br>
(See [NeuronId][https://docs.rs/cajal/latest/cajal/neuron/id/NeuronId]) <br>

Needless to say, commands can become rather verbose; `phantom-limb` is intended to be called programmatically. <br>

#### Sensors
`white-noise`     | Tests up to 256 paths by sending NeuronIds in a randomly-generated sequence.
#### Motors
`read-and-weep`   | Behavioral sink that prints activation sequence to STDOUT.


