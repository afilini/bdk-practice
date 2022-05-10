use std::str::FromStr;

use crate::Node;

impl FromStr for Node {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let separator = s
            .chars()
            .position(|c| !c.is_alphabetic() && c != ' ')
            .ok_or("Invalid operand")?;
        let operand = &s[..separator];

        match s.chars().nth(separator).unwrap() {
            ':' => {
                let inner = Box::new(Self::from_str(&s[(separator + 1)..])?);
                match operand {
                    "v" => Ok(Node::Verify(inner)),
                    _ => Err("Invalid wrapper"),
                }
            }
            '(' => {
                let s = &s[separator..];

                let mut commas = vec![];
                let closing_paren = s
                    .char_indices()
                    .scan((0, &mut commas), |(count, commas), (i, c)| {
                        match c {
                            '(' => *count += 1,
                            ')' => *count -= 1,
                            ',' if *count == 1 => commas.push(i),
                            _ => {}
                        }

                        if *count == 0 {
                            commas.push(i);
                            None
                        } else {
                            Some(i)
                        }
                    })
                    .last()
                    .ok_or("Missing closing parenthesis")?;

                let content = &s[1..(closing_paren + 1)];
                match operand.trim() {
                    "pk" => return Ok(Node::Pk(content.to_string())),
                    "after" => {
                        return Ok(Node::After(
                            content.parse().map_err(|_| "Invalid timelock")?,
                        ))
                    }
                    _ => {}
                }

                let inners = commas
                    .into_iter()
                    .scan(1, |start, end| {
                        let v = &s[*start..end];
                        *start = end + 1;
                        Some(v)
                    })
                    .collect::<Vec<_>>();

                match (operand, inners.as_slice()) {
                    ("thresh", [threshold, ..]) if inners.len() >= 2 => {
                        let threshold = threshold.parse().map_err(|_| "Invalid threshold")?;
                        let nodes = inners
                            .iter()
                            .skip(1)
                            .map(|s| Node::from_str(s))
                            .collect::<Result<Vec<_>, _>>()?;
                        if threshold as usize > nodes.len() {
                            Err("Threshold too high")
                        } else {
                            Ok(Node::Thresh(threshold, nodes))
                        }
                    }
                    (op, args) => todo!("{} {:?}", op, args),
                }
            }
            _ => Err("Unexpected character"),
        }
    }
}
