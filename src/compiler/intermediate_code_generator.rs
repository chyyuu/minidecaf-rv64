use crate::compiler::*;

pub fn generate_intermediate_code(ast: &Vec<Option<Node>>) -> Vec<String> {
    let mut mid_commands: Vec<String> = Vec::new();
    for node in ast.iter() {
        match node {
            Some(node) => {
                traverse(&node, &mut mid_commands);
            }
            _ => {}
        }
    }
    mid_commands
}

fn traverse(node: &Node, mid_commands: &mut Vec<String>) {
    match &node.left {
        Some(node_left) => {
            traverse(&node_left, mid_commands);
        }
        _ => {}
    }

    match &node.right {
        Some(node_right) => {
            traverse(&node_right, mid_commands);
        }
        _ => {}
    }

    match node.kind {
        NodeKind::NdNum => {
            mid_commands.push(format!("PUSH {}", node.val));
        }
        NodeKind::NdOperator => match &node.val[..] {
            "+" => {
                mid_commands.push(String::from("ADD"));
            }
            "-" => {
                mid_commands.push(String::from("SUB"));
            }
            "*" => {
                mid_commands.push(String::from("MUL"));
            }
            "/" => {
                mid_commands.push(String::from("DIV"));
            }
            _ => {
                panic!("Unexpected operator: {}", node.val);
            }
        },
        NodeKind::NdComparisonOperator => match &node.val[..] {
            "==" => {
                mid_commands.push(String::from("EQUAL"));
            }
            "!=" => {
                mid_commands.push(String::from("NONEQUAL"));
            }
            "<" => {
                mid_commands.push(String::from("LT"));
            }
            "<=" => {
                mid_commands.push(String::from("ELT"));
            }
            _ => {
                panic!("Unexpected operator: {}", node.val);
            }
        },
        NodeKind::NdVariable => {
            mid_commands.push(String::from(format!("LOAD {}", node.val)));
        }
    }
}
