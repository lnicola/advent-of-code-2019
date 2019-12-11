use std::collections::HashMap;
use std::iter::FromIterator;

#[derive(Clone)]
pub struct IntCode {
    mem: HashMap<usize, i128>,
    ip: usize,
    rb: i128,
}

pub struct InputCookie<'a>(usize, &'a mut IntCode);

impl InputCookie<'_> {
    pub fn set(self, val: i128) {
        *self.1.mem.entry(self.0).or_default() = val;
    }
}

pub enum State<'a> {
    Input(InputCookie<'a>),
    Output(i128),
    Halted,
}

impl IntCode {
    pub fn new() -> Self {
        Self {
            mem: HashMap::new(),
            ip: 0,
            rb: 0,
        }
    }

    pub fn from(mem: Vec<i128>) -> Self {
        Self {
            mem: HashMap::from_iter(mem.into_iter().enumerate()),
            ip: 0,
            rb: 0,
        }
    }

    pub fn run(&mut self) -> State {
        loop {
            let instr = self.mem[&self.ip];
            match instr % 100 {
                1 => {
                    let p1 = self.get_param(0);
                    let p2 = self.get_param(1);
                    let p3 = self.get_param_target(2);
                    *self.mem.entry(p3).or_default() = p1 + p2;
                    self.ip += 4;
                }
                2 => {
                    let p1 = self.get_param(0);
                    let p2 = self.get_param(1);
                    let p3 = self.get_param_target(2);
                    *self.mem.entry(p3).or_default() = p1 * p2;
                    self.ip += 4;
                }
                3 => {
                    let p1 = self.get_param_target(0);
                    self.ip += 2;
                    break State::Input(InputCookie(p1, self));
                }
                4 => {
                    let p1 = self.get_param(0);
                    self.ip += 2;
                    break State::Output(p1);
                }
                5 => {
                    let p1 = self.get_param(0);
                    let p2 = self.get_param(1);
                    if p1 != 0 {
                        self.ip = p2 as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                6 => {
                    let p1 = self.get_param(0);
                    let p2 = self.get_param(1);
                    if p1 == 0 {
                        self.ip = p2 as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                7 => {
                    let p1 = self.get_param(0);
                    let p2 = self.get_param(1);
                    let p3 = self.get_param_target(2);
                    if p1 < p2 {
                        *self.mem.entry(p3).or_default() = 1;
                    } else {
                        *self.mem.entry(p3).or_default() = 0;
                    }
                    self.ip += 4
                }
                8 => {
                    let p1 = self.get_param(0);
                    let p2 = self.get_param(1);
                    let p3 = self.get_param_target(2);
                    if p1 == p2 {
                        *self.mem.entry(p3).or_default() = 1;
                    } else {
                        *self.mem.entry(p3).or_default() = 0;
                    }
                    self.ip += 4
                }
                9 => {
                    let p1 = self.get_param(0);
                    self.rb += p1;
                    self.ip += 2;
                }
                _ => break State::Halted,
            }
        }
    }

    pub fn read_mem(&self, addr: usize) -> i128 {
        self.mem[&addr]
    }

    fn get_param(&self, pos: u8) -> i128 {
        let (addr, mode) = match pos {
            0 => (self.ip + 1, self.mem[&self.ip] / 100 % 10),
            1 => (self.ip + 2, self.mem[&self.ip] / 1_000 % 10),
            2 => (self.ip + 3, self.mem[&self.ip] / 10_000 % 10),
            _ => unreachable!(),
        };
        let val = self.mem[&addr];
        match mode {
            0 => self.mem[&(val as usize)],
            1 => val as i128,
            2 => self.mem[&((self.rb + val) as usize)],
            _ => unreachable!(),
        }
    }

    fn get_param_target(&self, pos: u8) -> usize {
        let (addr, mode) = match pos {
            0 => (self.ip + 1, self.mem[&self.ip] / 100 % 10),
            1 => (self.ip + 2, self.mem[&self.ip] / 1_000 % 10),
            2 => (self.ip + 3, self.mem[&self.ip] / 10_000 % 10),
            _ => unreachable!(),
        };
        let val = self.mem[&addr];
        match mode {
            0 => val as usize,
            2 => (self.rb + val) as usize,
            _ => unreachable!(),
        }
    }
}
