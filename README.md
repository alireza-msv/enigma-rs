# Enigma Encryption Simulator

[Enigma](https://en.wikipedia.org/wiki/Enigma_machine) is a encrpytion device used by Nazi Germany during World War II.

This project is a simple simalation of enigma machine

### How to use
First you need to create random enigma rotors.

#### Creating Enigma rotors
Following command will create enigma rotors.

```
cargo run generate
```

rotors state is saved in `rotors.enigma`.

#### Encoding message

```
cargo run encode <message>
```

This command will encode given message and prints the result.

#### Decoding cipher

```
cargo run decode <cipher>
```

This command will decode given cipher into plain text.