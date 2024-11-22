use tomtel::MemoryPointerResult;

mod tomtel;

pub fn execute(input: &[u8]) -> Vec<u8> {
    let mut memory = input.to_vec();
    let mut vm = tomtel::Tomtel::new(input.len());
    loop {
        let instruction_pointer = vm.pc() as usize;
        let opcode = input[instruction_pointer];
        let result = match opcode {
            0xC2 => {
                vm.increment_pc(1);
                vm.add()
            }
            0xE1 => {
                vm.increment_pc(2);
                vm.advance_pointer(input[instruction_pointer + 1])
            }
            0xC1 => {
                vm.increment_pc(1);
                vm.compare()
            }
            0x01 => {
                vm.increment_pc(1);
                vm.halt()
            }
            0x21 => {
                vm.increment_pc(5);
                vm.jump_equals_zero(u32::from_le_bytes([
                    input[instruction_pointer + 1],
                    input[instruction_pointer + 2],
                    input[instruction_pointer + 3],
                    input[instruction_pointer + 4],
                ]))
            }
            0x22 => {
                vm.increment_pc(5);
                vm.jump_not_equals_zero(u32::from_le_bytes([
                    input[instruction_pointer + 1],
                    input[instruction_pointer + 2],
                    input[instruction_pointer + 3],
                    input[instruction_pointer + 4],
                ]))
            }
            0x02 => {
                vm.increment_pc(1);
                vm.output()
            }
            0xC3 => {
                vm.increment_pc(1);
                vm.subtract()
            }
            0xC4 => {
                vm.increment_pc(1);
                vm.xor()
            },
            // These masks are put at the end of the switch statement because some of the previous opcodes overlap with the ranges.
            // If it's the 8 bit move instruction
            code if (code & 0b01000000 == 0b01000000) => {
                let source = opcode & 0b00000111;
                let destination = (opcode & 0b00111000) >> 3;

                if source == 0 {
                    vm.increment_pc(2);
                    match vm.mvi(input[instruction_pointer + 1], destination) {
                        Some(MemoryPointerResult::Write((address, value))) => {
                            memory[address as usize] = value;
                            None
                        }
                        _ => None,
                    }
                } else {
                    vm.increment_pc(1);
                    match vm.mv(source, destination) {
                        Some(MemoryPointerResult::Read(address)) => {
                            vm.mvi(memory[address as usize], destination);
                            None
                        }
                        Some(MemoryPointerResult::Write((address, value))) => {
                            memory[address as usize] = value;
                            None
                        }
                        _ => None,
                    }
                }
            }
            // If it's the 32 bit move instruction
            code if (code & 0b10000000 == 0b10000000) => {
                let source = opcode & 0b00000111;
                let destination = (opcode & 0b00111000) >> 3;

                if source == 0 {
                    vm.increment_pc(5);
                    let value = u32::from_le_bytes([
                        input[instruction_pointer + 1],
                        input[instruction_pointer + 2],
                        input[instruction_pointer + 3],
                        input[instruction_pointer + 4],
                    ]);
                    vm.mvi32(value, destination)
                } else {
                    vm.increment_pc(1);
                    vm.mv32(source, destination)
                }
            }
            _ => panic!("Unknown opcode: 0x{:02X} -> {}", opcode, vm.pc()),
        };

        // The only time an instruction would error is if it executes a HALT instruction.
        match result {
            None => continue,
            Some(_) => break,
        }
    }

    vm.output_stream()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        let input = vec![
            0x50, 0x48, //  # MVI b <- 72
            0xC2, //     # ADD a <- b
            0x02, //    # OUT a
            0xA8, 0x4D, 0x00, 0x00, 0x00, //  # MVI32 ptr <- 0x0000004d
            0x4F, //     # MV a <- (ptr+c)
            0x02, //     # OUT a
            0x50, 0x09, //  # MVI b <- 9
            0xC4, //     # XOR a <- b
            0x02, //     # OUT a
            0x02, //     # OUT a
            0xE1, 0x01, //  # APTR 0x00000001
            0x4F, //     # MV a <- (ptr+c)
            0x02, //     # OUT a
            0xC1, //     # CMP
            0x22, 0x1D, 0x00, 0x00, 0x00, //  # JNZ 0x0000001d
            0x48, 0x30, //  # MVI a <- 48
            0x02, //     # OUT a
            0x58, 0x03, //  # MVI c <- 3
            0x4F, //     # MV a <- (ptr+c)
            0x02, //     # OUT a
            0xB0, 0x29, 0x00, 0x00, 0x00, //  # MVI32 pc <- 0x00000029
            0x48, 0x31, //  # MVI a <- 49
            0x02, //     # OUT a
            0x50, 0x0C, //  # MVI b <- 12
            0xC3, //     # SUB a <- b
            0x02, //     # OUT a
            0xAA, //     # MV32 ptr <- lb
            0x57, //     # MV b <- (ptr+c)
            0x48, 0x02, //  # MVI a <- 2
            0xC1, //     # CMP
            0x21, 0x3A, 0x00, 0x00, 0x00, //  # JEZ 0x0000003a
            0x48, 0x32, //  # MVI a <- 50
            0x02, //     # OUT a
            0x48, 0x77, //  # MVI a <- 119
            0x02, //     # OUT a
            0x48, 0x6F, //  # MVI a <- 111
            0x02, //     # OUT a
            0x48, 0x72, //  # MVI a <- 114
            0x02, //     # OUT a
            0x48, 0x6C, //  # MVI a <- 108
            0x02, //     # OUT a
            0x48, 0x64, //  # MVI a <- 100
            0x02, //     # OUT a
            0x48, 0x21, //  # MVI a <- 33
            0x02, //     # OUT a
            0x01, //     # HALT
            0x65, 0x6F, 0x33, 0x34, 0x2C, //  # non-instruction data
        ];

        let output = super::execute(&input);
        let output = output.iter().map(|&x| x as char).collect::<String>();
        assert_eq!(output, "Hello, world!");
    }
}
