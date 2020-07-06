pub mod encoder {
    const ALPHABETS: &'static str = "abcdefghijklmnopqrstuvwxyz";

    pub fn encode(rotors: &mut [String; 3], input: &String) -> String {
        let mut encoded: String = String::new();
        let mut encoded_chars: usize = 0;

        for c in input.chars() {
            let mut ch: char = c;
            encoded_chars += 1;

            for r in rotors.into_iter() {
                ch = encode_char(&r, ch);
            }

            ch = reflect(ch);
            
            for r in rotors.into_iter().rev() {
                ch = encode_char_rev(&r, ch);
            }

            rotors[0] = rotate(&rotors[0]);
            
            if encoded_chars % ALPHABETS.len() == 0 {
                rotors[1] = rotate(&rotors[1]);
            }

            if encoded_chars % (ALPHABETS.len() * ALPHABETS.len()) == 0 {
                rotors[2] = rotate(&rotors[2]);
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

    fn rotate(rotor: &String) -> String {
        let mut rotated = String::new();
        rotated.push_str(&rotor[1..]);
        rotated.push(rotor.chars().nth(0).unwrap());

        rotated
    }
}
