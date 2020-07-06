pub mod encoder {
    const ALPHABETS: &'static str = "abcdefghijklmnopqrstuvwxyz";

    pub fn encode(rotors: &mut [String; 3], input: &String) -> String {
        let mut encoded: String = String::new();

        for c in input.chars() {
            let mut ch: char = c;

            for r in rotors.into_iter() {
                ch = encode_char(&r, ch);
            }
            
            ch = reflect(ch);
            
            for r in rotors.into_iter().rev() {
                ch = encode_char_rev(&r, ch);
            }

            encoded.push(ch);
        }

        encoded
    }

    fn encode_char(rotor: &String, c: char) -> char {
        let index = ALPHABETS.find(c).unwrap();
        let cipher = rotor.chars().nth(index).unwrap();

        cipher
    }

    fn encode_char_rev(rotor: &String, c: char) -> char {
        let index = rotor.find(c).unwrap();
        let cipher = ALPHABETS.chars().nth(index).unwrap();

        cipher
    }

    fn reflect(c: char) -> char {
        let index = ALPHABETS.len() - ALPHABETS.find(c).unwrap() - 1;
        let reflected = ALPHABETS.chars().nth(index).unwrap();
    
        reflected
    }
}
