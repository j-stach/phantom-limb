# Phantom Limb
*"So we just skirt the hallway sides, a phantom and a fly; <br> Follow the lines and wonder why there's no connection."* <br>
&emsp;-- The Shins

This is a crate for simulating sensor inputs and reading behavioral outputs for bionic neural networks. <br>
Use the CLI tool as an example reference for building signal interfaces for Cajal. <br>

**NOTE: Crate is incomplete without Cajal and will not be useful until Cajal is also released.**
## phantom_limb
A library with types for creating Sensor and Motor interfaces for Cajal BNNs. <br>
```
cargo add phantom_limb
```
`Sensor` types provide an interface for environmental stimuli to be converted into neurotransmission signals.
These can be set up to accomodate a variety of data types, but rely on a frequency-of-occurence model to determine
the relevance of a data pattern. <br> See [TBD](https://cajal.io/theory/signals) for more info. <br>
`Motor` types receive behavioral (ouput) signal impulses in the form of `NeuronId`s, translating them into some practical action.
These serve the same role as nerve endings that drive gland and muscle activity in living animals. <br>

## phantom-limb
A CLI tool for mocking signals to and from Cajal BNNs.
### How to install:
Requires Rust and Cargo: [Get started](https://www.rust-lang.org/learn/get-started) <br>
Ensure that `.cargo/bin` is in the PATH in order to run executables installed with `cargo install`. <br>
```
$ cargo install phantom-limb
$ phantom-limb help
```
### How to use:
```
$ phantom-limb [dummy] [duration] [socket_address] [neuron_id]+
```
For `dummy` choose one of the sensors or motors below. <br>
In `duration` specify the number of seconds (in u64) for which to run the dummy. <br>
If the dummy is a sensor, `socket_address` is the IPv6 address of the remote target. <br>
For a motor, it is the address of the socket for receiving the neurotransmission signal. <br>
List one or more `neuron_id` for the dummy sensor/motor to transmit/handle, respectively. <br>
For reference, the expected pattern for a NeuronId is `(\w+)/(\d+):(\d+)/(\d+):(\d+)`. 

Needless to say, commands can become rather verbose; `phantom-limb` is intended to be called programmatically. <br>

#### Sensors
`white-noise`     | Tests up to 256 paths by sending NeuronIds in a randomly-generated sequence.
#### Motors
`read-and-weep`   | Behavioral sink that prints activation sequence to STDOUT.
### Example:
Test phantom-limb by connecting a Sensor directly to a Motor. Start by providing a single NeuronId for them to exchange:
1. Open two terminals side by side.
2. In the first, run:
```
$ phantom-limb read-and-weep 10 127.0.0.1:8080 Test/0.0/0.0
```
3. In the second, quickly run:
```
$ phantom-limb white-noise 5 127.0.0.1:8080 Test/0.0/0.0
```
4. You will see `Test/0.0/0.0` spammed to STDOUT for 5 seconds. Buckle up.





