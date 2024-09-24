# Phantom Limb
> *"So we just skirt the hallway sides, a phantom and a fly; <br> Follow the lines and wonder why there's no connection."* <br>
> -- The Shins

This is a crate for simulating sensor inputs and reading behavioral outputs for simulated spiking neural networks. <br>

- Use the [CLI tool](https://github.com/j-stach/phantom-limb/blob/main/src/dummy/) as a reference for building signal interfaces for Cajal. <br>
- See [PongLand](https://github.com/j-stach/pongland/) for a more complete example of Phantom Limb in action.

**NOTE: Crate is incomplete without Cajal and will not be useful until Cajal is also released.**
## phantom_limb
A library with generic types for creating Sensor and Motor interfaces for spiking networks. <br>
```
cargo add phantom_limb
```
`Sensor` types provide an interface for environmental stimuli to be converted into neurotransmission signals.
These can be used to convert a variety of data types into the frequency-of-occurence model that Cajal uses.<br> 
`Motor` types receive behavioral (ouput) signal impulses in the form of `NeuronId`, translating them into some practical action.
These serve the same role as nerve endings that drive gland and muscle activity in living animals. <br>

## phantom-limb
A CLI tool for mocking signals to and from Cajal SNs.
### How to install:
**NOTE: Cajal is currently supported for Linux only.** <br> MacOS and WSL may have luck but I make no guarantees.
1. Requires Rust and Cargo: [Get started](https://www.rust-lang.org/learn/get-started) <br>
2. Add `~/.cargo/bin` to your `PATH` in order to run executables installed with `cargo install`. <br>
3. Build from source:
```
cargo install phantom-limb
phantom-limb help
```
### How to use:
```
phantom-limb [dummy] [duration] [socket_address] [neuron_id]+
```
1. For `dummy` choose one of the sensors or motors below. <br>
2. In `duration` specify the number of seconds (in u64) for which to run the dummy. <br>
3. If the dummy is a sensor, `socket_address` is the IPv6 address to send the neurotransmission signal to. <br>
For a motor, it is the address of the socket that will receive the neurotransmission signal. <br>
4. List one or more `neuron_id` for the dummy sensor/motor to transmit/handle, respectively. <br>
For reference, the expected pattern for a NeuronId is `(\w+)/(\d+):(\d+)/(\d+):(\d+)`. 

Needless to say, commands can become rather verbose; `phantom-limb` is designed to be called programmatically. <br>

#### Sensors
`white-noise`     | Tests up to 256 paths by sending NeuronIds in a randomly-generated sequence.
#### Motors
`read-and-weep`   | Behavioral sink that prints activation sequence to STDOUT.
### Example:
Test phantom-limb by connecting a Sensor directly to a Motor. Provide a single NeuronId for them to exchange:
1. Open two terminals side by side.
2. In the first, run:
```
phantom-limb read-and-weep 10 127.0.0.1:8080 Test/0.0/0.0
```
3. In the second, quickly run:
```
phantom-limb white-noise 5 127.0.0.1:8080 Test/0.0/0.0
```
4. This will print a stream of `Test/0.0/0.0` impulses lasting 5 seconds. Buckle up.





