use std::collections::VecDeque;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Status {
    Ready,
    WaitingForInput,
    Finished,
}

#[derive(Clone, Debug)]
pub struct Icvm {
    program: Vec<isize>,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
    pc: usize,
    status: Status,
}

impl Icvm {

    /// Create a new Intcode VM.
    pub fn new(program: Vec<isize>) -> Self {
        Self {
            program,
            input: VecDeque::new(),
            output: VecDeque::new(),
            pc: 0,
            status: Status::Ready,
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
        OPS[self.program[self.pc] as usize]((
            &mut self.program,
            &mut self.pc,
            &mut self.input,
            &mut self.output,
            &mut self.status,
        ));
    }

    /// Same as `run`, but without bounds checking.
    pub unsafe fn run_unchecked(&mut self) {
        while self.status == Status::Ready {
            self.step_unchecked();
        }
    }

    /// Same as `step`, but without bounds checking.
    pub unsafe fn step_unchecked(&mut self) {
        OPS.get_unchecked(*self.program.get_unchecked(self.pc) as usize)((
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
    pub fn program(&self) -> &[isize] {
        &self.program
    }

    /// Add an input to the VM.
    pub fn push_input(&mut self, i: isize) {
        self.input.push_back(i);
        if self.status == Status::WaitingForInput {
            self.status = Status::Ready;
        }
    }

    /// Add multiple inputs to the VM.
    pub fn push_inputs<I>(&mut self, i: I)
    where
        I: IntoIterator<Item = isize>
    {
        self.input.extend(i);
        if self.status == Status::WaitingForInput && !self.input.is_empty() {
            self.status = Status::Ready;
        }
    }

    /// Read the current input queue.
    pub fn inputs(&self) -> impl Iterator<Item = isize> + '_ {
        self.input.iter().copied()
    }

    /// Pop an output from the output queue.
    pub fn pop_output(&mut self) -> Option<isize> {
        self.output.pop_front()
    }

    /// Read the outputs without popping them off the output queue.
    pub fn outputs(&self) -> impl Iterator<Item = isize> + '_ {
        self.output.iter().copied()
    }

    /// Read the outputs while popping them off the output queue. See [`Vec::drain`].
    ///
    /// [`Vec::drain`]: https://doc.rust-lang.org/std/vec/struct.Vec.html#method.drain
    pub fn drain_outputs(&mut self) -> impl Iterator<Item = isize> + '_ {
        self.output.drain(..)
    }
}

type Op = fn((&mut [isize], &mut usize, &mut VecDeque<isize>, &mut VecDeque<isize>, &mut Status));

// divisions are evil
static OPS: [Op; 1109] = {
    let mut ops: [Op; 1109] = [|(..)| unreachable!(); 1109];

    ops[   1] = (|(p, c, ..)| { p[p[*c+3] as usize] = p[p[*c+1] as usize] + p[p[*c+2] as usize]; *c += 4; }) as Op;
    ops[ 101] = (|(p, c, ..)| { p[p[*c+3] as usize] =   p[*c+1]           + p[p[*c+2] as usize]; *c += 4; }) as Op;
    ops[1001] = (|(p, c, ..)| { p[p[*c+3] as usize] = p[p[*c+1] as usize] +   p[*c+2];           *c += 4; }) as Op;
    ops[1101] = (|(p, c, ..)| { p[p[*c+3] as usize] =   p[*c+1]           +   p[*c+2];           *c += 4; }) as Op;

    ops[   2] = (|(p, c, ..)| { p[p[*c+3] as usize] = p[p[*c+1] as usize] * p[p[*c+2] as usize]; *c += 4; }) as Op;
    ops[ 102] = (|(p, c, ..)| { p[p[*c+3] as usize] =   p[*c+1]           * p[p[*c+2] as usize]; *c += 4; }) as Op;
    ops[1002] = (|(p, c, ..)| { p[p[*c+3] as usize] = p[p[*c+1] as usize] *   p[*c+2];           *c += 4; }) as Op;
    ops[1102] = (|(p, c, ..)| { p[p[*c+3] as usize] =   p[*c+1]           *   p[*c+2];           *c += 4; }) as Op;

    ops[   3] = (|(p, c, i, _, s)| if let Some(x) = i.pop_front() { p[p[*c+1] as usize] = x; *c += 2; } else { *s = Status::WaitingForInput; }) as Op;

    ops[   4] = (|(p, c, _, o, _)| { o.push_back(p[p[*c+1] as usize]); *c += 2; }) as Op;
    ops[ 104] = (|(p, c, _, o, _)| { o.push_back(  p[*c+1]          ); *c += 2; }) as Op;

    ops[   5] = (|(p, c, ..)| if p[p[*c+1] as usize] != 0 { *c = p[p[*c+2] as usize] as usize; } else { *c += 3; }) as Op;
    ops[ 105] = (|(p, c, ..)| if   p[*c+1]           != 0 { *c = p[p[*c+2] as usize] as usize; } else { *c += 3; }) as Op;
    ops[1005] = (|(p, c, ..)| if p[p[*c+1] as usize] != 0 { *c =   p[*c+2]           as usize; } else { *c += 3; }) as Op;
    ops[1105] = (|(p, c, ..)| if   p[*c+1]           != 0 { *c =   p[*c+2]           as usize; } else { *c += 3; }) as Op;

    ops[   6] = (|(p, c, ..)| if p[p[*c+1] as usize] == 0 { *c = p[p[*c+2] as usize] as usize; } else { *c += 3; }) as Op;
    ops[ 106] = (|(p, c, ..)| if   p[*c+1]           == 0 { *c = p[p[*c+2] as usize] as usize; } else { *c += 3; }) as Op;
    ops[1006] = (|(p, c, ..)| if p[p[*c+1] as usize] == 0 { *c =   p[*c+2]           as usize; } else { *c += 3; }) as Op;
    ops[1106] = (|(p, c, ..)| if   p[*c+1]           == 0 { *c =   p[*c+2]           as usize; } else { *c += 3; }) as Op;

    ops[   7] = (|(p, c, ..)| { p[p[*c+3] as usize] = (p[p[*c+1] as usize] < p[p[*c+2] as usize]) as isize; *c += 4; }) as Op;
    ops[ 107] = (|(p, c, ..)| { p[p[*c+3] as usize] = (  p[*c+1]           < p[p[*c+2] as usize]) as isize; *c += 4; }) as Op;
    ops[1007] = (|(p, c, ..)| { p[p[*c+3] as usize] = (p[p[*c+1] as usize] <   p[*c+2]          ) as isize; *c += 4; }) as Op;
    ops[1107] = (|(p, c, ..)| { p[p[*c+3] as usize] = (  p[*c+1]           <   p[*c+2]          ) as isize; *c += 4; }) as Op;

    ops[   8] = (|(p, c, ..)| { p[p[*c+3] as usize] = (p[p[*c+1] as usize] == p[p[*c+2] as usize]) as isize; *c += 4; }) as Op;
    ops[ 108] = (|(p, c, ..)| { p[p[*c+3] as usize] = (  p[*c+1]           == p[p[*c+2] as usize]) as isize; *c += 4; }) as Op;
    ops[1008] = (|(p, c, ..)| { p[p[*c+3] as usize] = (p[p[*c+1] as usize] ==   p[*c+2]          ) as isize; *c += 4; }) as Op;
    ops[1108] = (|(p, c, ..)| { p[p[*c+3] as usize] = (  p[*c+1]           ==   p[*c+2]          ) as isize; *c += 4; }) as Op;

    ops[  99] = (|(.., s)| { *s = Status::Finished; }) as Op;

    ops
};

static UNSAFE_OPS: [Op; 1109] = {
    let mut ops: [Op; 1109] = [|(..)| unreachable!(); 1109];

    ops[   1] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = *p.get_unchecked(*p.get_unchecked(*c+1) as usize) + *p.get_unchecked(*p.get_unchecked(*c+2) as usize); *c += 4; }) as Op;
    ops[ 101] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) =                  *p.get_unchecked(*c+1)           + *p.get_unchecked(*p.get_unchecked(*c+2) as usize); *c += 4; }) as Op;
    ops[1001] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = *p.get_unchecked(*p.get_unchecked(*c+1) as usize) +                  *p.get_unchecked(*c+2);           *c += 4; }) as Op;
    ops[1101] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) =                  *p.get_unchecked(*c+1)           +                  *p.get_unchecked(*c+2);           *c += 4; }) as Op;

    ops[   2] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = *p.get_unchecked(*p.get_unchecked(*c+1) as usize) * *p.get_unchecked(*p.get_unchecked(*c+2) as usize); *c += 4; }) as Op;
    ops[ 102] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) =                  *p.get_unchecked(*c+1)           * *p.get_unchecked(*p.get_unchecked(*c+2) as usize); *c += 4; }) as Op;
    ops[1002] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = *p.get_unchecked(*p.get_unchecked(*c+1) as usize) *                  *p.get_unchecked(*c+2);           *c += 4; }) as Op;
    ops[1102] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) =                  *p.get_unchecked(*c+1)           *                  *p.get_unchecked(*c+2);           *c += 4; }) as Op;

    ops[   3] = (|(p, c, i, _, s)| unsafe { if let Some(x) = i.pop_front() { *p.get_unchecked_mut(*p.get_unchecked(*c+1) as usize) = x; *c += 2; } else { *s = Status::WaitingForInput; }}) as Op;

    ops[   4] = (|(p, c, _, o, _)| unsafe { o.push_back(*p.get_unchecked(*p.get_unchecked(*c+1) as usize)); *c += 2; }) as Op;
    ops[ 104] = (|(p, c, _, o, _)| unsafe { o.push_back(                 *p.get_unchecked(*c+1)          ); *c += 2; }) as Op;

    ops[   5] = (|(p, c, ..)| unsafe { if *p.get_unchecked(*p.get_unchecked(*c+1) as usize) != 0 { *c = *p.get_unchecked(*p.get_unchecked(*c+2) as usize) as usize; } else { *c += 3; }}) as Op;
    ops[ 105] = (|(p, c, ..)| unsafe { if                  *p.get_unchecked(*c+1)           != 0 { *c = *p.get_unchecked(*p.get_unchecked(*c+2) as usize) as usize; } else { *c += 3; }}) as Op;
    ops[1005] = (|(p, c, ..)| unsafe { if *p.get_unchecked(*p.get_unchecked(*c+1) as usize) != 0 { *c =                  *p.get_unchecked(*c+2)           as usize; } else { *c += 3; }}) as Op;
    ops[1105] = (|(p, c, ..)| unsafe { if                  *p.get_unchecked(*c+1)           != 0 { *c =                  *p.get_unchecked(*c+2)           as usize; } else { *c += 3; }}) as Op;

    ops[   6] = (|(p, c, ..)| unsafe { if *p.get_unchecked(*p.get_unchecked(*c+1) as usize) == 0 { *c = *p.get_unchecked(*p.get_unchecked(*c+2) as usize) as usize; } else { *c += 3; }}) as Op;
    ops[ 106] = (|(p, c, ..)| unsafe { if                  *p.get_unchecked(*c+1)           == 0 { *c = *p.get_unchecked(*p.get_unchecked(*c+2) as usize) as usize; } else { *c += 3; }}) as Op;
    ops[1006] = (|(p, c, ..)| unsafe { if *p.get_unchecked(*p.get_unchecked(*c+1) as usize) == 0 { *c =                  *p.get_unchecked(*c+2)           as usize; } else { *c += 3; }}) as Op;
    ops[1106] = (|(p, c, ..)| unsafe { if                  *p.get_unchecked(*c+1)           == 0 { *c =                  *p.get_unchecked(*c+2)           as usize; } else { *c += 3; }}) as Op;

    ops[   7] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (*p.get_unchecked(*p.get_unchecked(*c+1) as usize) < *p.get_unchecked(*p.get_unchecked(*c+2) as usize)) as isize; *c += 4; }) as Op;
    ops[ 107] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (                 *p.get_unchecked(*c+1)           < *p.get_unchecked(*p.get_unchecked(*c+2) as usize)) as isize; *c += 4; }) as Op;
    ops[1007] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (*p.get_unchecked(*p.get_unchecked(*c+1) as usize) <                  *p.get_unchecked(*c+2)          ) as isize; *c += 4; }) as Op;
    ops[1107] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (                 *p.get_unchecked(*c+1)           <                  *p.get_unchecked(*c+2)          ) as isize; *c += 4; }) as Op;

    ops[   8] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (*p.get_unchecked(*p.get_unchecked(*c+1) as usize) == *p.get_unchecked(*p.get_unchecked(*c+2) as usize)) as isize; *c += 4; }) as Op;
    ops[ 108] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (                 *p.get_unchecked(*c+1)           == *p.get_unchecked(*p.get_unchecked(*c+2) as usize)) as isize; *c += 4; }) as Op;
    ops[1008] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (*p.get_unchecked(*p.get_unchecked(*c+1) as usize) ==                  *p.get_unchecked(*c+2)          ) as isize; *c += 4; }) as Op;
    ops[1108] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (                 *p.get_unchecked(*c+1)           ==                  *p.get_unchecked(*c+2)          ) as isize; *c += 4; }) as Op;

    ops[  99] = (|(.., s)| { *s = Status::Finished; }) as Op;

    ops
};
