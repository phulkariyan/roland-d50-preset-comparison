mod d50_preset_comparison {
use std::collections::HashMap;

    pub struct D50Preset {
        name: String,
        parameters: HashMap<String, f32>,
    }

    

    
    impl D50Preset {
        pub fn new(name: &str) -> Self {
            D50Preset {
                name: name.to_string(),
                parameters: HashMap::new(),
            }
        }
        
        pub fn add_parameter(&mut self, key: &str, value: f32) {
            self.parameters.insert(key.to_string(), value);
        }
    }

    pub fn compare_presets(preset1: &D50Preset, preset2: &D50Preset) -> f32 {
        let mut similarity_score = 0.0;
        let total_parameters = preset1.parameters.len().max(preset2.parameters.len());

        for (key, value1) in &preset1.parameters {
            if let Some(value2) = preset2.parameters.get(key) {
                let difference = (value1 - value2).abs();
                similarity_score += 1.0 - (difference / 127.0); // Assuming parameter range 0-127
            }
        }

        similarity_score / total_parameters as f32
    }
}

fn main() {
    use d50_preset_comparison::{D50Preset, compare_presets};

    let mut preset1 = D50Preset::new("Digital Native Dance");
    preset1.add_parameter("cutoff", 64.0);
    preset1.add_parameter("resonance", 32.0);
    preset1.add_parameter("attack", 20.0);

    let mut preset2 = D50Preset::new("Fantasia");
    preset2.add_parameter("cutoff", 96.0);
    preset2.add_parameter("resonance", 48.0);
    preset2.add_parameter("attack", 40.0);

    let similarity = compare_presets(&preset1, &preset2);
    println!("Similarity between presets: {:.2}", similarity);
}
