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

        let mut num_messages = (0,1); // (high, low); The first message is low and not counted explicitly.
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

trait Module: Debug{
    /// The module receives a message, may updates it's internal state and returns a list of messages to send.
    fn tick(&mut self, input: Message) -> Vec<Message>;
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
    for (input, outputs) in conj_inputs {
        for output in outputs {
            conj_inputs_transposed
                .entry(output)
                .or_insert_with(HashMap::new)
                .insert(input.clone(), false);
        }
    }

    for line in text.trim().lines() {
        let (name, outputs) = line.split(" -> ").collect_tuple().unwrap();
        let outputs = outputs.split(", ").map(|x| x.to_string()).collect_vec();
        let name_without_prefix = &name[1..];
        match &name[0..=0] {
            "%" => state.add_module(FlipFlop::new(name_without_prefix, outputs)),
            "&" => state.add_module(Conjunction::new(name_without_prefix, outputs, conj_inputs_transposed.get(name_without_prefix).unwrap().clone())),
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

    let mut num_messages = (0, 0);
    for i in 0..1000 {
        let result = state.tick();
        num_messages.0 += result.0;
        num_messages.1 += result.1;
        if result.2 {
            dbg!(i);
        }
    }
    dbg!(num_messages.0 * num_messages.1);

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
    
    
    
    
    */





}

// Helper function that extracts the state of all inverters and returns it as a number.
// fn get_inverters(state: &State) -> u32 {
//     let mut inverters = state.modules.iter().filter_map(|x| match **x {
//         FlipFlop { name, is_on, .. } => Some((name, is_on)),
//         _ => None,
//     }).collect_vec();



// }