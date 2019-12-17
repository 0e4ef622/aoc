use std::collections::VecDeque;
use super::Status;

type Op = fn((&mut [i128], &mut usize, &mut VecDeque<i128>, &mut VecDeque<i128>, &mut Status));

// divisions are evil
pub static OPS: [Op; 1109] = {
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

    ops[   7] = (|(p, c, ..)| { p[p[*c+3] as usize] = (p[p[*c+1] as usize] < p[p[*c+2] as usize]) as i128; *c += 4; }) as Op;
    ops[ 107] = (|(p, c, ..)| { p[p[*c+3] as usize] = (  p[*c+1]           < p[p[*c+2] as usize]) as i128; *c += 4; }) as Op;
    ops[1007] = (|(p, c, ..)| { p[p[*c+3] as usize] = (p[p[*c+1] as usize] <   p[*c+2]          ) as i128; *c += 4; }) as Op;
    ops[1107] = (|(p, c, ..)| { p[p[*c+3] as usize] = (  p[*c+1]           <   p[*c+2]          ) as i128; *c += 4; }) as Op;

    ops[   8] = (|(p, c, ..)| { p[p[*c+3] as usize] = (p[p[*c+1] as usize] == p[p[*c+2] as usize]) as i128; *c += 4; }) as Op;
    ops[ 108] = (|(p, c, ..)| { p[p[*c+3] as usize] = (  p[*c+1]           == p[p[*c+2] as usize]) as i128; *c += 4; }) as Op;
    ops[1008] = (|(p, c, ..)| { p[p[*c+3] as usize] = (p[p[*c+1] as usize] ==   p[*c+2]          ) as i128; *c += 4; }) as Op;
    ops[1108] = (|(p, c, ..)| { p[p[*c+3] as usize] = (  p[*c+1]           ==   p[*c+2]          ) as i128; *c += 4; }) as Op;

    ops[  99] = (|(.., s)| { *s = Status::Finished; }) as Op;

    ops
};

pub static UNSAFE_OPS: [Op; 1109] = {
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

    ops[   7] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (*p.get_unchecked(*p.get_unchecked(*c+1) as usize) < *p.get_unchecked(*p.get_unchecked(*c+2) as usize)) as i128; *c += 4; }) as Op;
    ops[ 107] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (                 *p.get_unchecked(*c+1)           < *p.get_unchecked(*p.get_unchecked(*c+2) as usize)) as i128; *c += 4; }) as Op;
    ops[1007] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (*p.get_unchecked(*p.get_unchecked(*c+1) as usize) <                  *p.get_unchecked(*c+2)          ) as i128; *c += 4; }) as Op;
    ops[1107] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (                 *p.get_unchecked(*c+1)           <                  *p.get_unchecked(*c+2)          ) as i128; *c += 4; }) as Op;

    ops[   8] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (*p.get_unchecked(*p.get_unchecked(*c+1) as usize) == *p.get_unchecked(*p.get_unchecked(*c+2) as usize)) as i128; *c += 4; }) as Op;
    ops[ 108] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (                 *p.get_unchecked(*c+1)           == *p.get_unchecked(*p.get_unchecked(*c+2) as usize)) as i128; *c += 4; }) as Op;
    ops[1008] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (*p.get_unchecked(*p.get_unchecked(*c+1) as usize) ==                  *p.get_unchecked(*c+2)          ) as i128; *c += 4; }) as Op;
    ops[1108] = (|(p, c, ..)| unsafe { *p.get_unchecked_mut(*p.get_unchecked(*c+3) as usize) = (                 *p.get_unchecked(*c+1)           ==                  *p.get_unchecked(*c+2)          ) as i128; *c += 4; }) as Op;

    ops[  99] = (|(.., s)| { *s = Status::Finished; }) as Op;

    ops
};
