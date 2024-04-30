
use serde::{ Serialize, Deserialize };


/// Unique identifier for neurons, containing information about its location.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct NeuronId {
    /// The name of the enclosing structure.
    pub structure: String,
    /// The row, index of cluster in structure.
    pub cluster: (usize, usize),
    /// The layer, index of neuron in cluster.
    pub neuron: (usize, usize)
}

impl NeuronId {
    /// Create an id without any information to serve as a placeholder.
    /// Should be set by calling Structure::label_neurons() after generation.
    pub fn null() -> Self {
        NeuronId {
            structure: String::new(),
            cluster: (0, 0),
            neuron: (0, 0)
        }
    }
    /// Creates a new id with indexing information.
    pub fn new(
        structure: &str,
        cluster: (usize, usize),
        neuron: (usize, usize)
    ) -> Self { NeuronId { structure: structure.to_owned(), cluster, neuron }}
}


impl std::fmt::Display for NeuronId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}:{}/{}:{}",
            self.structure,
            self.cluster.0,
            self.cluster.1,
            self.neuron.0,
            self.neuron.1,
        )
    }
}

impl std::str::FromStr for NeuronId {
    type Err = anyhow::Error;       // TODO: Needs dedicated error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = regex::Regex::new(r"(\w+)/(\d+):(\d+)/(\d+):(\d+)")
            .expect("Regular expression should be valid syntax");
        if let Some(values) = regex.captures(s) {
            let structure = values.get(1).unwrap().as_str();
            let cluster = {
                let row = values.get(2).unwrap().as_str().parse::<usize>()?;
                let index = values.get(3).unwrap().as_str().parse::<usize>()?;
                (row, index)
            };
            let neuron = {
                let layer = values.get(4).unwrap().as_str().parse::<usize>()?;
                let index = values.get(5).unwrap().as_str().parse::<usize>()?;
                (layer, index)
            };
            return Ok(NeuronId::new(structure, cluster, neuron))
        }

        Err(anyhow::anyhow!("NeuronId parse failed"))
    }
}
