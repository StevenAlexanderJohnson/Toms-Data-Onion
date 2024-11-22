mod tomtel;

pub fn execute(input: &[u8]) -> Vec<u8> {
    let mut vm = tomtel::Tomtel::new(input.len());
    let mut i = 0;
    loop {
        let instruction_pointer = vm.pc() as usize;
        let opcode = input[instruction_pointer];
        let result = match opcode {
            0xC2 => vm.add(),
            0xE1 => vm.advance_pointer(input[instruction_pointer + 1]),
            0xC1 => vm.compare(),
            0x01 => {
                println!("Halting: {}", i);
                vm.halt()
            },
            0x21 => vm.jump_equals_zero(u32::from_le_bytes([
                input[instruction_pointer + 1],
                input[instruction_pointer + 2],
                input[instruction_pointer + 3],
                input[instruction_pointer + 4],
            ])),
            0x22 => vm.jump_not_equals_zero(u32::from_le_bytes([
                input[instruction_pointer + 1],
                input[instruction_pointer + 2],
                input[instruction_pointer + 3],
                input[instruction_pointer + 4],
            ])),
            0x02 => vm.output(),
            0xC3 => vm.subtract(),
            0xC4 => vm.xor(),
            // These masks are put at the end of the switch statement because some of the previous opcodes overlap with the ranges.
            // If it's the 8 bit move instruction
            code if (code & 0b01000000 == 0b01000000) => {
                let source = opcode & 0b00000111;
                let destination = (opcode & 0b00111000) >> 3;

                if source == 0 {
                    vm.mvi(input[instruction_pointer + 1], destination)
                } else {
                    vm.mv(source, destination)
                }
            }
            // If it's the 32 bit move instruction
            code if (code & 0b10000000 == 0b10000000) => {
                let source = opcode & 0b00000111;
                let destination = (opcode & 0b00111000) >> 3;

                if source == 0 {
                    let value = u32::from_le_bytes([
                        input[instruction_pointer + 1],
                        input[instruction_pointer + 2],
                        input[instruction_pointer + 3],
                        input[instruction_pointer + 4],
                    ]);
                    vm.mvi32(value, destination)
                } else {
                    vm.mv32(source, destination)
                }
            }
            _ => panic!("Unknown opcode: 0x{:02X} -> {}", opcode, vm.pc()),
        };

        i += 1;

        // The only time an instruction would error is if it executes a HALT instruction.
        match result {
            None => continue,
            Some(_) => break,
        }
    }

    println!("{:?}", vm);

    vm.output_stream()
}
