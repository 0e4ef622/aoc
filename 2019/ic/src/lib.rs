use std::collections::VecDeque;

mod op_lookup;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Status {
    Ready,
    WaitingForInput,
    Finished,
}

#[derive(Clone, Debug)]
pub struct Icvm {
    program: Vec<i128>,
    input: VecDeque<i128>,
    output: VecDeque<i128>,
    pc: usize,
    status: Status,
    base: i128,
}

impl Icvm {

    /// Create a new Intcode VM.
    pub fn new(mut program: Vec<i128>) -> Self {
        program.resize(program.len()*10, 0);
        Self {
            program,
            input: VecDeque::new(),
            output: VecDeque::new(),
            pc: 0,
            status: Status::Ready,
            base: 0,
        }
    }

    /// Run until completion, or until the program needs more input.
    pub fn run(&mut self) {
        while self.status == Status::Ready {
            self.step();
        }
    }

    /// Step one instruction forward.
    pub fn step(&mut self) {

        // starting from 1

        let op = self.program[self.pc] % 100;

        let j = match op {

            1 => { *self.get_param(3) = *self.get_param(1) + *self.get_param(2); 4 }
            2 => { *self.get_param(3) = *self.get_param(1) * *self.get_param(2); 4 }

            3 => if let Some(i) = self.input.pop_front() { *self.get_param(1) = i; 2 }
                 else { self.status = Status::WaitingForInput; 0 }

            4 => { let o = *self.get_param(1); self.output.push_back(o); 2 },

            5 => if *self.get_param(1) != 0 { self.pc = *self.get_param(2) as usize; 0 } else { 3 }
            6 => if *self.get_param(1) == 0 { self.pc = *self.get_param(2) as usize; 0 } else { 3 }

            7 => { *self.get_param(3) = (*self.get_param(1) < *self.get_param(2)) as i128; 4 }
            8 => { *self.get_param(3) = (*self.get_param(1) == *self.get_param(2)) as i128; 4 }

            9 => { self.base += *self.get_param(1); 2 }

            99 => { self.status = Status::Finished; 0 }

            _ => panic!("bad opcode"),
        };

        self.pc += j;
    }

    fn get_param(&mut self, n: u32) -> &mut i128 {

        let mode = self.program[self.pc] / 10i128.pow(n+1) % 10;
        let pc = self.pc;
        let pg = &mut self.program[..]; // magically avoids lifetime issues

        match mode {
            0 => &mut pg[pg[pc + n as usize] as usize],
            1 => &mut pg[pc + n as usize],
            2 => &mut pg[(pg[self.pc + n as usize] + self.base) as usize],
            _ => panic!("bad mode"),
        }
    }

    /// Same as `run`, but uses a lookup table.
    pub fn run_fast(&mut self) {
        while self.status == Status::Ready {
            self.step_fast();
        }
    }

    /// Same as `step`, but uses a lookup table.
    pub fn step_fast(&mut self) {
        op_lookup::OPS[self.program[self.pc] as usize]((
            &mut self.program,
            &mut self.pc,
            &mut self.input,
            &mut self.output,
            &mut self.status,
        ));
    }

    /// Same as `run_fast`, but without bounds checking.
    pub unsafe fn run_fast_unchecked(&mut self) {
        while self.status == Status::Ready {
            self.step_fast_unchecked();
        }
    }

    /// Same as `step_fast`, but without bounds checking.
    pub unsafe fn step_fast_unchecked(&mut self) {
        op_lookup::UNSAFE_OPS.get_unchecked(*self.program.get_unchecked(self.pc) as usize)((
            &mut self.program,
            &mut self.pc,
            &mut self.input,
            &mut self.output,
            &mut self.status,
        ));
    }

    /// The current program counter, or instruction pointer.
    pub fn pc(&self) -> usize {
        self.pc
    }

    /// The current status.
    pub fn status(&self) -> Status {
        self.status
    }

    /// The program this VM contains.
    pub fn program(&self) -> &[i128] {
        &self.program
    }

    /// Add an input to the VM.
    pub fn push_input(&mut self, i: impl Into<i128>) {
        self.input.push_back(i.into());
        if self.status == Status::WaitingForInput {
            self.status = Status::Ready;
        }
    }

    /// Add multiple inputs to the VM.
    pub fn push_inputs<I, T>(&mut self, i: I)
    where
        T: Into<i128>,
        I: IntoIterator<Item = T>
    {
        self.input.extend(i.into_iter().map(Into::into));
        if self.status == Status::WaitingForInput && !self.input.is_empty() {
            self.status = Status::Ready;
        }
    }

    /// Read the current input queue.
    pub fn inputs(&self) -> impl Iterator<Item = i128> + '_ {
        self.input.iter().copied()
    }

    /// Pop an output from the output queue.
    pub fn pop_output(&mut self) -> Option<i128> {
        self.output.pop_front()
    }

    /// Read the outputs without popping them off the output queue.
    pub fn outputs(&self) -> impl Iterator<Item = i128> + '_ {
        self.output.iter().copied()
    }

    /// Read the outputs while popping them off the output queue. See [`Vec::drain`].
    ///
    /// [`Vec::drain`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain
    pub fn drain_outputs(&mut self) -> impl Iterator<Item = i128> + '_ {
        self.output.drain(..)
    }
}
