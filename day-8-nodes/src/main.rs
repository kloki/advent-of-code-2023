use instructions::{
    Instruction,
    Instructions,
};
use nodes::{
    Node,
    Nodes,
};
use toolkit::input::get_input;
mod instructions;
mod nodes;

pub struct Runner {
    instructions: Instructions,
    nodes: Nodes,
}

impl Runner {
    pub fn new(instructions: Instructions, nodes: Nodes) -> Self {
        Runner {
            instructions,
            nodes,
        }
    }

    pub fn resolve(&mut self, start: Node, ghost: bool) -> usize {
        let mut current = start.clone();
        let mut counter: usize = 0;
        self.instructions.reset();

        loop {
            if !ghost && current.is_end() || ghost && current.is_ghost_end() {
                return counter;
            }
            counter += 1;
            let (left, right) = self.nodes.nodes.get(&current).unwrap();

            let instruction = self.instructions.next().unwrap();
            match instruction {
                Instruction::Left => current = *left,
                Instruction::Right => current = *right,
            }
        }
    }
}

pub fn gcd(a: usize, b: usize) -> usize {
    let mut b = b;
    let mut a = a;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

pub fn lcm(a: usize, b: usize) -> usize {
    return a * b / gcd(a, b);
}

fn main() {
    let contents = get_input();
    let (instructions, nodes) = contents.trim().split_once("\n\n").unwrap();
    let instructions: Instructions = instructions.parse().unwrap();
    let nodes: Nodes = nodes.parse().unwrap();

    let mut runner = Runner::new(instructions.clone(), nodes.clone());
    let result = runner.resolve(Node::start(), false);
    println!("run 1:{}", result);
    let scores = nodes
        .nodes
        .keys()
        .filter(|n| n.is_ghost_start())
        .map(|node| runner.resolve(*node, true))
        .collect::<Vec<_>>();
    let lcm_result = scores.iter().fold(1, |acc, x| lcm(acc, *x));
    println!("run 2:{}", lcm_result);
}
