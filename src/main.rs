use std::collections::HashMap;

#[derive(Debug)]
enum Node {
    Producer {
        out_buffer: Buffer,
        production_amount: u64,
        out_connections: Vec<ConnectionType>,
    },
    Consumer {
        in_buffer: Buffer,
        usage_amount: u64,
    },
    Intermediate {
        in_buffer: Buffer,
        out_buffer: Buffer,
        conversion_rate: u64,
        out_connections: Vec<ConnectionType>,
    },
}
impl Node {
    fn new_producer(
        buf_size: u64,
        production_amount: u64,
        out_connections: Vec<ConnectionType>,
    ) -> Node {
        Self::Producer {
            out_buffer: Buffer::new(buf_size),
            production_amount,
            out_connections,
        }
    }
    fn new_intermeddiate(
        in_buf_size: u64,
        out_buf_size: u64,
        conversion_rate: u64,
        out_connections: Vec<ConnectionType>,
    ) -> Node {
        Self::Intermediate {
            in_buffer: Buffer::new(in_buf_size),
            out_buffer: Buffer::new(out_buf_size),
            conversion_rate,
            out_connections,
        }
    }
    fn new_consumer(buf_size: u64, consumption_amount: u64) -> Node {
        Self::Consumer {
            in_buffer: Buffer::new(buf_size),
            usage_amount: consumption_amount,
        }
    }
}
#[derive(Debug)]
enum ConnectionType {
    End(u64),
    Intermediate(u64),
}
#[derive(Debug)]
struct Buffer {
    max: u64,
    amount: u64,
}
#[derive(Debug)]
struct Transaction {
    from: ConnectionType,
    to: ConnectionType,
    amount: u64,
}
impl Buffer {
    fn new(max: u64) -> Self {
        Self { max, amount: 0 }
    }

    fn can_afford(&mut self, removal: u64) -> bool {
        self.amount >= removal
    }

    fn 
    fn add(&mut self, amount: i64) {
        if amount < 0 {
            self.amount -= -amount as u64;
        } else {
            self.amount += amount as u64;
            self.amount = self.amount.clamp(0, self.max);
        }
    }
}

fn main() {
    let mut nodes = HashMap::<u64, Node>::new();
    nodes.insert(
        0,
        Node::new_producer(10, 3, vec![ConnectionType::Intermediate(1)]),
    );
    nodes.insert(
        1,
        Node::new_intermeddiate(
            10,
            10,
            2,
            vec![ConnectionType::End(2), ConnectionType::End(3)],
        ),
    );
    nodes.insert(2, Node::new_consumer(10, 3));
    nodes.insert(3, Node::new_consumer(10, 3));
    let transfer_amount = 3;
    let mut iteration = 0;
    loop {
        iteration += 1;
        for (_, node) in &mut nodes {
            match node {
                Node::Producer {
                    out_buffer,
                    production_amount,
                    ..
                } => out_buffer.add(*production_amount as i64),
                _ => {
                    todo!()
                }
            }
        }
        let mut transacts = vec![];
        for id in nodes.keys() {
            match nodes.get(id).unwrap() {
                Node::Producer {
                    out_buffer,
                    production_amount,
                    out_connections,
                } => {
                    for conn in out_connections {
                        transacts.push(Transaction {
                            from: ConnectionType::End(*id),
                            to: *conn,
                            amount: transfer_amount,
                        })
                    }
                }

                Node::Intermediate {
                    in_buffer,
                    out_buffer,
                    conversion_rate,
                    out_connections,
                } => {
                    for conn in out_connections {
                        transacts.push(Transaction {
                            from: ConnectionType::Intermediate(*id),
                            to: *conn,
                            amount: transfer_amount,
                        })
                    }
                }
                _ => {
                    todo!()
                }
            }
        }

        for trans in &transacts {
            // the hard part is determining which buffer to pull from, so just get references to the in and out buffers
            let mut from_buffer = match trans.from {
                ConnectionType::End(id) => match nodes.get_mut(&id).unwrap() {
                    Node::Producer { out_buffer, .. } => out_buffer,
                    _ => {
                        panic!()
                    }
                },
                ConnectionType::Intermediate(id) => match nodes.get_mut(&id).unwrap() {
                    Node::Intermediate { out_buffer, .. } => out_buffer,
                    _ => {
                        panic!()
                    }
                },
                _ => {
                    panic!()
                }
            };

            match trans.to {
                ConnectionType::End(id) => {
                    nodes.entry(id).and_modify(|x| match x {
                        Node::Consumer { in_buffer, .. } => {}
                        _ => {
                            panic!();
                        }
                    });
                }
                ConnectionType::Intermediate(id) => {}
            }
        }
        for (id, node) in &mut nodes {
            if node.amount < 0 && node.buffer.can_afford(-node.amount as u64) {
                node.buffer.add(node.amount);
            }
        }
        println!("{:?} {}", nodes, iteration);
        assert!(iteration <= 4);
    }
}
