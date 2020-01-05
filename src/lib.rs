pub fn calculate_fuel_req(mass: u32) -> i32 {
    let div = (mass / 3) as f32;
    return (div.floor() - 2.0) as i32;
}

pub struct Wire {
    pub input: String,
    pub points: Vec<(i64,i64)>
}

impl Wire {
    pub fn thread(mut self) -> Vec<(i64,i64)> {
        let split = self.input.split(",");

        for s in split {
            let direction = s.chars().next().unwrap();
            let intensity = &s[1..s.len()].parse::<i64>().unwrap();

            let mut updated: i64;
            let init_x = self.points[self.points.len()-1].0;
            let init_y = self.points[self.points.len()-1].1;

            // TODO: how would we generalize this so we don't have so much boilerplate code
            match direction {
                'L' =>  {
                    for x in 1..*intensity + 1 {
                        updated = init_x - x;
                        self.points.push((updated, init_y));
                    }
                },
                'R' =>  {
                    for x in 1..*intensity + 1 {
                        updated = init_x + x;
                        self.points.push((updated, init_y));
                    }
                },
                'U' =>  {
                    for y in 1..*intensity + 1 {
                        updated = init_y + y;
                        self.points.push((init_x, updated));
                    }
                },
                'D' =>  {
                    for y in 1..*intensity + 1 {
                        updated = init_y - y;
                        self.points.push((init_x, updated));
                    }
                },
                _ => ()
            }
        }

        return self.points;
    }
}

enum Opcodes {
    Op1,
    Op2,
    Op99
}

pub struct IntProgram {
    pub mem: Vec<u128>,
    pub pointer: usize
}

impl IntProgram {
    fn handle_op(&mut self, op: Opcodes) {
        let rhs = self.mem[self.pointer+1] as usize;
        let lhs = self.mem[self.pointer+2] as usize;
        let loc = self.mem[self.pointer+3] as usize;

        let answer = match op {
            Opcodes::Op1 => Some(self.mem[rhs] + self.mem[lhs]),
            Opcodes::Op2 => Some(self.mem[rhs] * self.mem[lhs]),
            Opcodes::Op99 => None
        };

        match answer {
            Some(val) => { self.mem[loc] = val },
            None => ()
        }
    }

    pub fn run(&mut self, init1: u128, init2: u128) {
        let opcode_length: usize = 4;

        while self.pointer < self.mem.len() {
            let opcode = self.mem[self.pointer];
            self.mem[1] = init1;
            self.mem[2] = init2;

            match opcode {
                1 => {
                    self.handle_op(Opcodes::Op1);
                },
                2 => {
                    self.handle_op(Opcodes::Op2);
                },
                99 => {
                    self.handle_op(Opcodes::Op99);
                },
                _ => ()
            }

            self.pointer += opcode_length;
        }
    }
}
