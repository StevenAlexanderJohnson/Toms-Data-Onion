mod tomtel;

pub fn execute(input: &[u8]) -> Vec<u8> {
    let mut input = input.to_vec();
    let mut vm = tomtel::Tomtel::new();

    loop {
        let instruction_pointer = vm.pc() as usize;
        let opcode = input[instruction_pointer];
        let result = match opcode {
            0xC2 => vm.add(),
            0xE1 => vm.advance_pointer(input[instruction_pointer + 1]),
            0xC1 => vm.compare(),
            0x01 => vm.halt(),
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
            0b01001000..=0b01111111 => {
                let source = opcode & 0b00000111;
                let destination = (opcode & 0b00111000) >> 3;

                if source == 0 {
                    let value = input[instruction_pointer + 1];
                    match vm.mvi(destination, value) {
                        Some(tomtel::MemoryPointerResult::Read(address)) => vm.mvi(destination, input[address as usize]),
                        Some(tomtel::MemoryPointerResult::Write((address, value))) => {
                            input[address as usize] = value;
                            None
                        },
                        None => None,
                    };
                } else {
                    match vm.mv(source, destination) {
                        Some(tomtel::MemoryPointerResult::Read(address)) => vm.mv(destination, input[address as usize]),
                        Some(tomtel::MemoryPointerResult::Write((address, value))) => {
                            input[address as usize] = value;
                            None
                        },
                        None => None,
                    };
                }

                Ok(())
            }
            0b10001000..=0b10111111 => {
                let source = opcode & 0b00000111;
                let destination = (opcode & 0b00111000) >> 3;

                if source == 0 {
                    let value = u32::from_le_bytes([
                        input[instruction_pointer + 1],
                        input[instruction_pointer + 2],
                        input[instruction_pointer + 3],
                        input[instruction_pointer + 4],
                    ]);
                    vm.mvi32(destination, value)
                } else {
                    vm.mv32(source, destination)
                }
            }
            0x02 => vm.output(),
            0xC3 => vm.subtract(),
            0xC4 => vm.xor(),
            _ => panic!("Unknown opcode: 0x{:02X}", opcode),
        };

        // The only time an instruction would error is if it executes a HALT instruction.
        match result {
            Ok(_) => continue,
            Err(_) => break,
        }
    }

    vm.output_stream()
}
