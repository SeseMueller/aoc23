// Switchboard madness!
// There are three modules: broadcaster, flip-flop and conjunction.

// The broadcaster, when receiving a pulse, sends it to all its outputs.
// The flip-flop, when receiving a low pulse, it flips from on to off and vice versa. It then sends a pulse to its output which is high, if it's now on.
// The conjunction remembers the most recent pulse it received fpr every connected module.
// When it receives a pulse, it first updates its own memory, then it sends a low pulse to its output if all its inputs are high, otherwise it sends a high pulse.

use std::{collections::HashMap, fmt::Debug};

use itertools::Itertools;

#[derive(Debug)]
struct State {
    modules: Vec<Box<dyn Module>>,
}

impl State {
    fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    fn add_module(&mut self, module: impl Module + 'static) {
        self.modules.push(Box::new(module));
    }

    // Ticks the State and returns the number of high and low pulses as well for part 2, if there was a low pulse sent to rx.
    fn tick(&mut self) -> (i32, i32, bool) {
        // First get the initial messages from the broadcaster.
        let mess = Message {
            sender: "button".to_string(),
            receiver: "broadcaster".to_string(),
            is_high: false,
        };
        let mut messages = Vec::new();
        for module in &mut self.modules {
            messages.append(&mut module.tick(mess.clone()));
        }

        let mut num_messages = (0, 1); // (high, low); The first message is low and not counted explicitly.
        messages.iter().for_each(|x| {
            if x.is_high {
                num_messages.0 += 1;
            } else {
                num_messages.1 += 1;
            }
        });

        let mut part2_rx = false;

        // From here on, first send all messages, recording the new ones, until there are no new messages.
        loop {
            let mut new_messages = Vec::new();
            for message in &messages {
                // dbg!(message.clone());
                if message.receiver == "rx" && !message.is_high {
                    part2_rx = true;
                }
                for module in &mut self.modules {
                    new_messages.append(&mut module.tick(message.clone()));
                }
            }
            if new_messages.is_empty() {
                break;
            }

            new_messages.iter().for_each(|x| {
                if x.is_high {
                    num_messages.0 += 1;
                } else {
                    num_messages.1 += 1;
                }
            });

            messages = new_messages;
        }
        (num_messages.0, num_messages.1, part2_rx)
    }

    fn tick_many(&mut self, num: usize) -> (i32, i32, bool) {
        let mut num_messages = (0, 0, false);
        for _ in 0..num {
            let result = self.tick();
            num_messages.0 += result.0;
            num_messages.1 += result.1;
            if result.2 {
                num_messages.2 = true;
            }
        }
        num_messages
    }
}

// impl Hash for State{
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.modules.hash(state);
//     }
// }

#[derive(Debug, Clone)]
struct Message {
    sender: String,
    receiver: String,
    is_high: bool,
}

trait Module: Debug {
    /// The module receives a message, may updates it's internal state and returns a list of messages to send.
    fn tick(&mut self, input: Message) -> Vec<Message>;

    /// If the module is an inverter, get its state.
    fn get_flipflop_state(&self) -> Option<bool> {
        None
    }

    /// If the module is a conjunction, get its state.
    fn get_conjunction_state(&self) -> Option<bool> {
        None
    }

    /// Gets the name of the module.
    fn get_name(&self) -> String;
}

#[derive(Debug, Clone)]
struct Broadcaster {
    name: String,
    outputs: Vec<String>,
}

impl Broadcaster {
    fn new(name: impl Into<String>, outputs: Vec<String>) -> Self {
        Self {
            name: name.into(),
            outputs,
        }
    }
}

impl Module for Broadcaster {
    // Simply sends the message to all outputs.
    fn tick(&mut self, input: Message) -> Vec<Message> {
        if input.sender == self.name {
            return Vec::new();
        }
        if input.receiver != self.name {
            return Vec::new();
        }
        let mut messages = Vec::new();
        for output in &self.outputs {
            messages.push(Message {
                sender: self.name.clone(),
                receiver: output.clone(),
                is_high: input.is_high,
            });
        }
        messages
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
struct FlipFlop {
    name: String,
    outputs: Vec<String>,
    is_on: bool,
}

impl FlipFlop {
    fn new(name: impl Into<String>, outputs: Vec<String>) -> Self {
        Self {
            name: name.into(),
            outputs,
            is_on: false,
        }
    }
}

impl Module for FlipFlop {
    fn tick(&mut self, input: Message) -> Vec<Message> {
        if input.sender == self.name {
            return Vec::new();
        }
        if input.receiver != self.name {
            return Vec::new();
        }
        if input.is_high {
            return Vec::new();
        }
        self.is_on = !self.is_on;
        let mut messages = Vec::new();
        for output in &self.outputs {
            messages.push(Message {
                sender: self.name.clone(),
                receiver: output.clone(),
                is_high: self.is_on,
            });
        }
        messages
    }

    fn get_flipflop_state(&self) -> Option<bool> {
        Some(self.is_on)
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    name: String,
    inputs_memory: HashMap<String, bool>,
    outputs: Vec<String>,
}

impl Conjunction {
    fn new(name: impl Into<String>, outputs: Vec<String>, inputs: HashMap<String, bool>) -> Self {
        Self {
            name: name.into(),
            inputs_memory: inputs,
            outputs,
        }
    }
}

impl Module for Conjunction {
    fn tick(&mut self, input: Message) -> Vec<Message> {
        if input.sender == self.name {
            return Vec::new();
        }
        if input.receiver != self.name {
            return Vec::new();
        }
        let _ = self
            .inputs_memory
            .insert(input.sender.clone(), input.is_high); // Just overwrite the old value.
        let is_high = self.inputs_memory.values().all(|&x| x);

        let mut messages = Vec::new();
        for output in &self.outputs {
            messages.push(Message {
                sender: self.name.clone(),
                receiver: output.clone(),
                is_high: !is_high,
            });
        }
        messages
    }

    fn get_conjunction_state(&self) -> Option<bool> {
        Some(self.inputs_memory.values().all(|&x| x))
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let text = include_str!("TEMP.txt");
    // let text = include_str!("TEMP2.txt");
    // let text = include_str!("TEMP3.txt");

    let mut state = State::new();

    // Precompute inputs for the conjunctions.
    let mut conj_inputs = HashMap::new();

    for line in text.trim().lines() {
        let (name, outputs) = line.split(" -> ").collect_tuple().unwrap();
        let outputs = outputs.split(", ").map(|x| x.to_string()).collect_vec();
        let name_without_prefix = &name[1..];
        match &name[0..=0] {
            "%" | "&" => conj_inputs.insert(name_without_prefix.to_string(), outputs),
            _ => conj_inputs.insert(name.to_string(), outputs),
        };
    }

    // "transpose" the hashmap, ie. create a hashmap where the keys are the outputs and the values are the inputs.
    let mut conj_inputs_transposed = HashMap::new();
    for (input, outputs) in conj_inputs.clone() {
        for output in outputs {
            conj_inputs_transposed
                .entry(output)
                .or_insert_with(HashMap::new)
                .insert(input.clone(), false);
        }
    }

    // dbg!(conj_inputs_transposed.clone());

    for line in text.trim().lines() {
        let (name, outputs) = line.split(" -> ").collect_tuple().unwrap();
        let outputs = outputs.split(", ").map(|x| x.to_string()).collect_vec();
        let name_without_prefix = &name[1..];
        match &name[0..=0] {
            "%" => state.add_module(FlipFlop::new(name_without_prefix, outputs)),
            "&" => state.add_module(Conjunction::new(
                name_without_prefix,
                outputs,
                conj_inputs_transposed
                    .get(name_without_prefix)
                    .unwrap()
                    .clone(),
            )),
            _ => state.add_module(Broadcaster::new(name, outputs)),
        }
    }

    //DEBUG
    // // dbg!(&state);
    // println!("{:?}",state.tick());
    // println!("{:?}",state.tick());
    // println!("{:?}",state.tick());
    // println!("{:?}",state.tick());
    // println!("{:?}",state.tick());
    // println!("{:?}",state.tick());
    // println!("{:?}",state.tick());
    // println!("{:?}",state.tick());
    // println!("{:?}",state.tick());
    // // dbg!(state);

    // let mut num_messages = (0, 0);
    // for i in 0..1000 {
    //     let result = state.tick();
    //     num_messages.0 += result.0;
    //     num_messages.1 += result.1;
    //     if result.2 {
    //         dbg!(i);
    //     }
    // }
    // dbg!(num_messages.0 * num_messages.1);

    // for i in 1000.. {
    //     let result = state.tick();
    //     num_messages.0 += result.0;
    //     num_messages.1 += result.1;
    //     if result.2 {
    //         dbg!(i);
    //         break;
    //     }
    // }

    // Part 2: manual Mathematical analysis
    /*
    The conjunctions are the modules that send to rx.
    In my problem input specifically, cn -> rx.
    Interestingly enough, the conjunction cn is fed by four conjunctions, th, sv, gh, ch.
    That means that cn will trigger if all four are triggered, but none of them have all inputs triggered.


    Hm.....
    Interesting, all flipflops are inverters; that is they

    */

    // Testing fro P2:
    // print_ticks(&mut state, 100, 1);

    // Tries cycle detection:

    // Stores the current state of the modules.
    // If one switches, the time since the last switch is stored.

    // If one switches concsistenly for 16 times, it is assumed to be a cycle with that period.
    
    // let mut curr_cicle_lengths = HashMap::new();
    // let mut states = HashMap::new();
    //
    // for line in text.trim().lines() {
    //     let (name, _) = line.split(" -> ").collect_tuple().unwrap();
    //     let name_without_prefix = &name[1..];
    //     match &name[0..=0] {
    //         "%" | "&" => {
    //             states.insert(name_without_prefix.to_string(), Vec::new());
    //             curr_cicle_lengths.insert(name_without_prefix.to_string(), Vec::new())
    //         }
    //         _ => {
    //             states.insert(name.to_string(), Vec::new());
    //             curr_cicle_lengths.insert(name.to_string(), Vec::new())
    //         }
    //     };
    // }

    // const EMPTY_VEC: Vec<u64> = Vec::new();
    // let mut curr_cicle_lengths = [EMPTY_VEC; 60];
    // let mut states = [EMPTY_VEC; 60];

    // // dbg!(&states);

    // let mut curr_state = get_module_states(&state);
    // let mut prev_state = curr_state;

    // let mut done_states = 0u64;

    // loop {
    //     let result = state.tick();
    //     curr_state = get_module_states(&state);
    //     // dbg!(curr_state);
    //     let changed = curr_state ^ prev_state;
    //     let changed = changed & !done_states; // Mask out the ones that have already been done.

    //     // dbg!(changed);

    //     let one_positons = (0u64..60).filter(|x| changed & (1 << x) != 0).collect_vec();



    //     prev_state = curr_state;
    // }

    // Print the conj_inputs in such a way that a graph can be generated from it.
    // ie %vg --> &ch
    //    %vg --> %fm

    // let mut prefixes = HashMap::new();

    // for line in text.trim().lines() {
    //     let (name, _) = line.split(" -> ").collect_tuple().unwrap();
    //     let name_without_prefix = &name[1..];
    //     match &name[0..=0] {
    //         "%" | "&" => {
    //             prefixes.insert(name_without_prefix.to_string(), name[0..=0].to_string());
    //         }
    //         _ => {
    //             prefixes.insert(name.to_string(), "".to_string());
    //         }
    //     };
    // }

    // for c in conj_inputs {
    //     for o in c.1 {
    //         let first = prefixes.get(&c.0).unwrap_or(&"".to_string()).to_owned() + &c.0;
    //         let second = prefixes.get(&o).unwrap_or(&"".to_string()).to_owned() + &o;

    //         // Map the prefixes because Mermaid is stupid.
    //         let first = first.replace("%", ".");
    //         let first = first.replace("&", "#");
    //         let second = second.replace("%", ".");
    //         let second = second.replace("&", "#");
    //         println!("{} --> {}", first, second);
    //     }
    // }

    // Manual reading of cycles: Reading to the conjuctions is 1, and the timer starts at the 0th place
    /*
    pl: 100001011111
    fd: 110101101111
    jc: 101100101111
    hm: 111001101111
     */
    
    // let pl = 0b100001011111;
    // let fd = 0b110101101111;
    // let jc = 0b101100101111;
    // let hm = 0b111001101111;
    
    // println!("{}", lcm(pl, lcm(fd, lcm(jc, hm)))); // 77963286380945 is too low...
    
    // I flipped them :/
    // pl : 111110100001
    // fd : 111101101011
    // jc : 111101001101
    // hm : 111101100111

    let pl = 0b111110100001;
    let fd = 0b111101101011;
    let jc = 0b111101001101;
    let hm = 0b111101100111;

    println!("{}", lcm(pl, lcm(fd, lcm(jc, hm)))); // lets see if this works... 243902373381257 works! :D

    // This works because there is a specific pattern in the input.
    // There are four conjunctions feeding into the output.
    // Each has a chain of 12 flipflips that is feeding into the next flipflop, started by the broadcaster.
    // That means that the chain is counting in binary.
    // The conjunction is only connected to some of the flipflops, so when these bits are set, the conjunction is triggered.
    // Upon triggering, the conjunction also toggles all flipflops that aren't part of its input, but ARE part of its chain.
    // This means that when the conjunction triggers, the chain is reset to 0. This is where the 4 cycles come from.

    // The conjunctions have to be triggered all at the same time, so the cycle length is the lcm of the four cycle lengths.
    // The numbers I wrote down come from looking at the graph visualization. 
    // I looked at all 4 chains and noted where the fliflops input into the conjunctions.
    // Those are the ones, the others are zeroes. Note that the first in the chain is the zeroth place, so the furthest to the right.

    // That also means that an efficient algorithm needs to assume that this pattern exists, because otherwise you'd have to brute force it.
    // And the flipflops can act as NAND gates, which (the way they work here) could technically be turing complete (I'm pretty sure).
    // Everyone had 48 Flipflops and 9 conjunctions, so the flipflops can have 2^48 states, which is a lot.
    // Additionally, the conjunctions behave longterm in a way I haven't managed to abstract fully, but they remind me of petri nets, which also give a ton of computational power.
    // An algorithm that doesn't expect this underlaying structure would have to brute force it, which, in this case gives a maximum of 280856887234560 = 4096*4095*4093*4091;
    //because that is their lcm, as they have no common factors.

    // All in all, nice problem.
    // It definetly holds back LLM users :)
}

// Helper function for lcm
fn lcm(a: i128, b: i128) -> i128 {
    a * b / gcd(a, b)
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}


// Helper function that extracts the state of all modules and returns it as a number.
fn get_module_states(state: &State) -> u64 {
    let mut inverters = state
        .modules
        .iter()
        .filter_map(|x| x.get_flipflop_state())
        .collect_vec();
    let mut conjunctors = state
        .modules
        .iter()
        .filter_map(|x| x.get_conjunction_state())
        .collect_vec();

    // Generate binary number from bool vecs
    inverters.reverse();
    conjunctors.reverse();
    let mut num = 0;
    for (i, x) in inverters.iter().chain(conjunctors.iter()).enumerate() {
        if *x {
            num += 2u64.pow(i as u32);
        }
    }
    num
}

// Helper function that prints n ticks of the state, all m apart
fn print_ticks(state: &mut State, n: usize, m: usize) {
    for _ in 0..n {
        state.tick_many(m);
        println!(
            "{}",
            format!("{:b}", get_module_states(&state))
                .replace("1", "#")
                .replace("0", " ")
        );
    }
}
