use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::str::FromStr;

use anyhow::{anyhow, Context};

use crate::input::day_18::INPUT;

pub fn part_1() -> i64 {
    let program: Program = INPUT.parse().unwrap();
    let mut interpreter = DuetInterpreter::for_part_1(program);

    loop {
        if let Some(n) = interpreter.execute_next().unwrap().received() {
            return n;
        }
    }
}

#[cfg(not(feature = "day_18_with_channels"))]
pub fn part_2() -> usize {
    part_2_sync()
}

#[cfg(feature = "day_18_with_channels")]
pub use with_channels::part_2;

#[allow(unused)]
fn part_2_sync() -> usize {
    let program: Program = INPUT.parse().unwrap();

    let queue_from_0_to_1 = SyncQueue::new();
    let queue_from_1_to_0 = SyncQueue::new();

    let mut interpreters = [
        DuetInterpreter::for_part_2(
            program.clone(),
            0,
            queue_from_1_to_0.clone(),
            queue_from_0_to_1.clone(),
        ),
        DuetInterpreter::for_part_2(
            program.clone(),
            1,
            queue_from_0_to_1.clone(),
            queue_from_1_to_0.clone(),
        ),
    ];

    let mut interpreter = 0usize;
    let mut wait_count = 0;
    loop {
        let next_interpreter = &mut interpreters[interpreter];
        loop {
            match next_interpreter.execute_next().unwrap() {
                InstructionResult::Waiting => break,
                _ => wait_count = 0,
            }
        }

        wait_count += 1;
        if wait_count == 2 {
            return interpreters[1].send_count();
        }
        interpreter = (interpreter + 1) % interpreters.len();
    }
}

trait SendQueue<T> {
    fn push(&mut self, value: T);
}

trait ReceiveQueue<T> {
    fn pop(&mut self, interpreter_id: i64) -> Option<T>;
    fn pop_last(&mut self, interpreter_id: i64) -> Option<T>;
}

#[derive(Debug, Clone)]
struct SyncQueue<T>(Rc<RefCell<VecDeque<T>>>);

impl<T> SyncQueue<T> {
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(VecDeque::new())))
    }
}

impl<T> SendQueue<T> for SyncQueue<T> {
    fn push(&mut self, value: T) {
        self.0.borrow_mut().push_back(value);
    }
}

impl<T> ReceiveQueue<T> for SyncQueue<T> {
    fn pop(&mut self, _interpreter_id: i64) -> Option<T> {
        self.0.borrow_mut().pop_front()
    }

    fn pop_last(&mut self, _interpreter_id: i64) -> Option<T> {
        self.0.borrow_mut().pop_back()
    }
}

#[derive(Debug, Default)]
struct Registers(HashMap<char, i64>);

impl Registers {
    pub fn get(&self, register: char) -> i64 {
        self.0.get(&register).copied().unwrap_or_default()
    }

    pub fn set(&mut self, register: char, value: i64) {
        self.0.insert(register, value);
    }
}

#[derive(Debug, Copy, Clone)]
enum Value {
    Number(i64),
    Register(char),
}

impl Value {
    pub fn get(&self, registers: &Registers) -> i64 {
        match self {
            Self::Number(n) => *n,
            Self::Register(register) => registers.get(*register),
        }
    }
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i64>() {
            Ok(n) => Self::Number(n),
            Err(_) => Self::Register(s.chars().next().with_context(|| "empty value")?),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum InstructionResult {
    Unit,
    JmpOffset(i64),
    Received(i64),
    Waiting,
}

impl InstructionResult {
    pub fn jmp_offset(&self) -> i64 {
        match self {
            Self::JmpOffset(offset) => *offset,
            Self::Waiting => 0,
            _ => 1,
        }
    }

    pub fn received(&self) -> Option<i64> {
        match self {
            Self::Received(n) => Some(*n),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Snd(Value),
    Set(char, Value),
    Add(char, Value),
    Mul(char, Value),
    Mod(char, Value),
    Rcv(char),
    Jgz(Value, Value),
}

impl Instruction {
    pub fn execute(
        &self,
        id: i64,
        registers: &mut Registers,
        send_count: &mut usize,
        send_queue: &mut impl SendQueue<i64>,
        rcv_queue: &mut impl ReceiveQueue<i64>,
        part_1: bool,
    ) -> Result<InstructionResult, anyhow::Error> {
        match self {
            Self::Snd(value) => {
                let value = value.get(registers);
                *send_count += 1;
                send_queue.push(value);
                println!("interpreter {id} sent {value} (send_count: {})", *send_count);
            },
            Self::Set(register, value) => registers.set(*register, value.get(registers)),
            Self::Add(register, value) => {
                registers.set(*register, registers.get(*register) + value.get(registers))
            },
            Self::Mul(register, value) => {
                registers.set(*register, registers.get(*register) * value.get(registers))
            },
            Self::Mod(register, value) => {
                registers.set(*register, registers.get(*register).rem_euclid(value.get(registers)))
            },
            Self::Rcv(register) => {
                if part_1 {
                    if registers.get(*register) != 0 {
                        return Ok(InstructionResult::Received(
                            rcv_queue.pop_last(id).with_context(|| "no sound played")?,
                        ));
                    }
                } else {
                    return Ok(match rcv_queue.pop(id) {
                        Some(n) => {
                            registers.set(*register, n);
                            println!("interpreter {id} received {n}");
                            InstructionResult::Received(n)
                        },
                        None => InstructionResult::Waiting,
                    });
                }
            },
            Self::Jgz(value, jmp_offset) => {
                if value.get(registers) > 0 {
                    return Ok(InstructionResult::JmpOffset(jmp_offset.get(registers)));
                }
            },
        }

        Ok(InstructionResult::Unit)
    }

    fn read_register<'a, I>(parts: &mut I) -> Result<char, anyhow::Error>
    where
        I: Iterator<Item = &'a str>,
    {
        parts
            .next()
            .with_context(|| "missing register name")?
            .chars()
            .next()
            .with_context(|| "empty register name")
    }

    fn read_value<'a, I>(parts: &mut I) -> Result<Value, anyhow::Error>
    where
        I: Iterator<Item = &'a str>,
    {
        parts.next().with_context(|| "missing value")?.parse()
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let opcode = parts.next().with_context(|| "empty instruction")?;

        match opcode {
            "snd" => Ok(Self::Snd(Self::read_value(&mut parts)?)),
            "set" => Ok(Self::Set(Self::read_register(&mut parts)?, Self::read_value(&mut parts)?)),
            "add" => Ok(Self::Add(Self::read_register(&mut parts)?, Self::read_value(&mut parts)?)),
            "mul" => Ok(Self::Mul(Self::read_register(&mut parts)?, Self::read_value(&mut parts)?)),
            "mod" => Ok(Self::Mod(Self::read_register(&mut parts)?, Self::read_value(&mut parts)?)),
            "rcv" => Ok(Self::Rcv(Self::read_register(&mut parts)?)),
            "jgz" => Ok(Self::Jgz(Self::read_value(&mut parts)?, Self::read_value(&mut parts)?)),
            opcode => Err(anyhow!("invalid opcode: {opcode}")),
        }
    }
}

#[derive(Debug, Clone)]
struct Program(Vec<Instruction>);

impl Program {
    #[allow(clippy::too_many_arguments)]
    pub fn execute(
        &self,
        id: i64,
        ip: i64,
        registers: &mut Registers,
        send_count: &mut usize,
        send_queue: &mut impl SendQueue<i64>,
        rcv_queue: &mut impl ReceiveQueue<i64>,
        part_1: bool,
    ) -> Result<InstructionResult, anyhow::Error> {
        self.0
            .get(ip as usize)
            .with_context(|| format!("invalid instruction pointer: {ip}"))?
            .execute(id, registers, send_count, send_queue, rcv_queue, part_1)
    }
}

impl FromStr for Program {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?))
    }
}

#[derive(Debug)]
struct DuetInterpreter<SQ, RQ> {
    id: i64,
    program: Program,
    registers: Registers,
    ip: i64,
    send_count: usize,
    send_queue: SQ,
    rcv_queue: RQ,
    part_1: bool,
}

impl DuetInterpreter<SyncQueue<i64>, SyncQueue<i64>> {
    pub fn for_part_1(program: Program) -> Self {
        let send_queue = SyncQueue::new();
        let rcv_queue = send_queue.clone();

        Self {
            id: 0,
            program,
            registers: Registers::default(),
            ip: 0,
            send_count: 0,
            send_queue,
            rcv_queue,
            part_1: true,
        }
    }
}

impl<SQ, RQ> DuetInterpreter<SQ, RQ> {
    pub fn for_part_2(program: Program, id: i64, send_queue: SQ, rcv_queue: RQ) -> Self {
        let mut registers = Registers::default();
        registers.set('p', id);

        Self { id, program, registers, ip: 0, send_count: 0, send_queue, rcv_queue, part_1: false }
    }

    pub fn send_count(&self) -> usize {
        self.send_count
    }
}

impl<SQ, RQ> DuetInterpreter<SQ, RQ>
where
    SQ: SendQueue<i64>,
    RQ: ReceiveQueue<i64>,
{
    pub fn execute_next(&mut self) -> Result<InstructionResult, anyhow::Error> {
        let result = self.program.execute(
            self.id,
            self.ip,
            &mut self.registers,
            &mut self.send_count,
            &mut self.send_queue,
            &mut self.rcv_queue,
            self.part_1,
        )?;
        self.ip += result.jmp_offset();
        Ok(result)
    }
}

mod with_channels {
    use std::fmt::Display;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::mpsc::{Receiver, Sender};
    use std::sync::{mpsc, Arc};
    use std::thread;

    use super::*;

    #[derive(Debug, Default)]
    struct DeadlockDetector(AtomicBool);

    impl DeadlockDetector {
        fn request_read(&self) -> bool {
            self.0
                .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
        }

        fn release_read(&self) {
            self.0
                .compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst)
                .expect("release_read should succeed if the read request succeeded");
        }
    }

    #[derive(Debug)]
    struct SendChannel<T>(Sender<T>);

    impl<T> SendQueue<T> for SendChannel<T> {
        fn push(&mut self, value: T) {
            self.0.send(value).unwrap()
        }
    }

    #[derive(Debug)]
    struct ReceiveChannel<T> {
        deadlock_detector: Arc<DeadlockDetector>,
        receiver: Receiver<T>,
    }

    impl<T> ReceiveQueue<T> for ReceiveChannel<T>
    where
        T: Display,
    {
        fn pop(&mut self, interpreter_id: i64) -> Option<T> {
            if self.deadlock_detector.request_read() {
                println!("interpreter {interpreter_id}: waiting to receive value");
                // We're the only ones currently attempting to read, so go ahead.
                if let Ok(value) = self.receiver.recv() {
                    self.deadlock_detector.release_read();
                    println!("interpreter {interpreter_id}: received value {value}");
                    Some(value)
                } else {
                    // This means the other interpreter detected a deadlock; stop.
                    println!("interpreter {interpreter_id}: recv() failed, other interpreter detected a deadlock");
                    None
                }
            } else {
                // This means the other interpreter is currently waiting for a read as well.
                // This means we've reached a deadlock; stop.
                println!("interpreter {interpreter_id}: deadlock detected, stopping");
                None
            }
        }

        fn pop_last(&mut self, _interpreter_id: i64) -> Option<T> {
            unreachable!("pop_last is not supported for part 2");
        }
    }

    pub fn part_2() -> usize {
        let program: Program = INPUT.parse().unwrap();

        let (sender_to_0, receiver_for_0) = mpsc::channel();
        let (sender_to_1, receiver_for_1) = mpsc::channel();

        let deadlock_detector = Arc::new(DeadlockDetector::default());

        let sender_to_1 = SendChannel(sender_to_1);
        let receiver_for_0 = ReceiveChannel {
            deadlock_detector: Arc::clone(&deadlock_detector),
            receiver: receiver_for_0,
        };

        let sender_to_0 = SendChannel(sender_to_0);
        let receiver_for_1 = ReceiveChannel {
            deadlock_detector: Arc::clone(&deadlock_detector),
            receiver: receiver_for_1,
        };

        let make_interpreter = |id, sender, receiver| {
            let program = program.clone();
            thread::spawn(move || {
                let mut interpreter = DuetInterpreter::for_part_2(program, id, sender, receiver);

                loop {
                    if interpreter.execute_next().unwrap() == InstructionResult::Waiting {
                        // This indicates a deadlock; stop execution and return send count.
                        return interpreter.send_count();
                    }
                }
            })
        };

        let interpreter_0 = make_interpreter(0, sender_to_1, receiver_for_0);
        let interpreter_1 = make_interpreter(1, sender_to_0, receiver_for_1);

        interpreter_0.join().unwrap();
        interpreter_1.join().unwrap()
    }
}
