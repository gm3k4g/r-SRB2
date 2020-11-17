# Sonic Robo Blast 2 - made with Rust

## What is this exactly?

- This is an attempt to translate the entire source code of Sonic Robo Blast 2 from C/C++ to Rust. The entire process is basically just eyeballing the source code and attempting to translate it into Rust code by hand.

- So far I've barely gotten anything up and running, I've mostly only gotten bits of the 'skeleton' of the code written down, what with TODOs here and there. I will probably need much time to finish this off.

- For those who are curious about the structure of this source code, it's something like this:
1. Make a struct for each .c/.h file. If there are other individual enums/structs within the same files, those will be declared in the same file.
2. Pass everything around

- I'm open to critiques about the code and if I'm doing anything wrong; at this point I'm pretty sure I'm not even correctly utilizing Rust correctly but I'm mostly concentrating on getting everything to *work* like SRB2's source code. So mostly focused on *translating* over to Rust.

## What is this being tested on?

- (Bedrock linux) x86_64 Arch linux Desktop.

## Why do all this? What's the point?

- Honestly there's really no point in doing this, SRB2 is already going pretty well. But I believe this is a good opportunity for me to really try and open my eyes, at least with Rust; To get more experienced? *And maybe because of a few benefits that Rust offers, though it's not really anything special.*

- (this README is part of a work in progress and will change over time.)
