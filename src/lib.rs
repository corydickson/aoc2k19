pub fn calculate_fuel_req(mass: u32) -> i32 {
    let div = (mass / 3) as f32;
    return (div.floor() - 2.0) as i32;
}

pub fn read_program(instructions: &mut Vec<u128>) -> () {
    let opcode_length: usize = 4;

    let mut idx: usize = 0;

    while idx < instructions.len() {

        let opcode = instructions[idx];

        match opcode {
            1 => {
                let rhs = instructions[idx+1] as usize;
                let lhs = instructions[idx+2] as usize;
                let location = instructions[idx+3] as usize;

                let answer = instructions[rhs] + instructions[lhs];

                instructions[location] = answer;
            },
            2 => {
                let rhs = instructions[idx+1] as usize;
                let lhs = instructions[idx+2] as usize;
                let location = instructions[idx+3] as usize;

                let answer = instructions[rhs] * instructions[lhs];

                instructions[location] = answer;
            },
            _ => ()
        }

        idx += opcode_length;
    }
}


/*
pub struct Program {
    instructions: Vec<u32>
}

pub enum Opcodes {
    OPCODE1,
    OPCODE2
}

*/

