pub mod rotors {
    use std::fs::File;
    use std::io::prelude::*;
    use rand::seq::SliceRandom;

    const ALPHABETS: &'static str = "abcdefghijklmnopqrstuvwxyz";

    pub fn generate() -> std::io::Result<()> {
        let mut file = File::create("rotors.enigma")?;

        for _ in 0..3 {
            let rotor: Vec<u8> = generate_rotor();
            file.write_all(&rotor)?;
        }

        file.sync_all()?;
        Ok(())
    }

    pub fn read() -> std::io::Result<[String; 3]> {
        let mut content = String::new();
        let mut file = File::open("rotors.enigma")?;
        file.read_to_string(&mut content)?;

        let mut rotors: [String; 3] = [String::new(), String::new(), String::new()];
        rotors[0] = content[..26].to_string();
        rotors[1] = content[26..52].to_string();
        rotors[2] = content[52..].to_string();
        
        Ok(rotors)
    }

    fn generate_rotor() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut bytes = ALPHABETS.to_string().into_bytes();
        bytes.shuffle(&mut rng);
    
        bytes
    }
}
