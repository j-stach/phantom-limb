# Phantom Limb
> *"So we just skirt the hallway sides, a phantom and a fly; <br> Follow the lines and wonder why there's no connection."* <br>
> -- The Shins

A library with generic types for `Sensor` and `Motor` interfaces 
    to and from spiking networks 
    built with the [Cajal framework](https://github.com/j-stach/cajal). <br>

## Use
```
cargo add phantom_limb
```
- `Sensor` types provide an interface for environmental stimuli to be converted into neurotransmission signals.
These can be used to convert a variety of data types into the frequency-of-occurence model that Cajal uses. <br> 
- `Motor` types receive behavioral (ouput) signal impulses in the form of `NeuronId`, translating them into some practical action.
These serve the same role as nerve endings that drive gland and muscle activity in living creatures. <br>

