use std::{
    collections::HashMap,
    fmt,
    str::FromStr,
};

const START: [char; 3] = ['A'; 3];
const END: [char; 3] = ['Z'; 3];
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Node([char; 3]);

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.0[0], self.0[1], self.0[2])
    }
}

impl Node {
    pub fn start() -> Self {
        Node(START)
    }
    pub fn is_ghost_start(&self) -> bool {
        self.0[2] == START[2]
    }
    pub fn is_ghost_end(&self) -> bool {
        self.0[2] == END[2]
    }
    pub fn is_end(&self) -> bool {
        self.0 == END
    }
}

#[derive(Clone)]
pub struct Nodes {
    pub nodes: HashMap<Node, (Node, Node)>,
}
#[derive(Debug)]
pub struct ParseNodeError;
impl FromStr for Nodes {
    type Err = ParseNodeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut nodes: HashMap<Node, (Node, Node)> = HashMap::new();
        for line in input.trim().split("\n") {
            let c = line.chars().collect::<Vec<_>>();
            let source = Node([c[0], c[1], c[2]]);
            let left = Node([c[7], c[8], c[9]]);
            let right = Node([c[12], c[13], c[14]]);
            nodes.insert(source, (left, right));
        }
        Ok(Nodes { nodes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    #[test]
    fn test_parse() {
        let nodes: Nodes = TEST_INPUT.parse().unwrap();
        assert_eq!(nodes.nodes.len(), 3)
    }

    #[test]
    fn test_start() {
        let node = Node::start();
        assert_eq!(node, Node(['A', 'A', 'A']))
    }
}
