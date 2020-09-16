mod toposort;

use std::error::Error;
use std::{
    any::Any,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    rc::Rc,
};
use toposort::toposort;
use uuid::Uuid;
use vision_traits::{NodeProcessable, NodeProcessingError};

pub struct Connection {
    pub output_uuid: Uuid,
    pub output_name: String,
}

struct UuidNode {
    node: Box<dyn NodeProcessable>,
    links: HashMap<String, Connection>,
}

impl UuidNode {
    fn process(
        &mut self,
        input: &HashMap<String, &dyn Any>,
    ) -> Result<HashMap<String, Box<dyn Any>>, NodeProcessingError> {
        self.node.process(input)
    }
}

pub struct Pipeline {
    map: HashMap<Uuid, UuidNode>,
    adj_list: HashMap<Uuid, HashSet<Uuid>>,
    run_order: Vec<Uuid>,
}

impl Pipeline {
    pub fn new() -> Pipeline {
        Pipeline {
            map: HashMap::new(),
            adj_list: HashMap::new(),
            run_order: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Box<dyn NodeProcessable>) -> Uuid {
        let uuid = Uuid::new_v4();

        self.map.insert(
            uuid,
            UuidNode {
                node: node,
                links: HashMap::new(),
            },
        );
        self.adj_list.insert(uuid, HashSet::new());
        self.run_order.clear();

        uuid
    }

    pub fn connect(&mut self, node_id: Uuid, input_field: String, connection: Connection) {
        let node = self.map.get_mut(&node_id).unwrap();

        self.adj_list
            .get_mut(&connection.output_uuid)
            .unwrap()
            .insert(node_id);
        node.links.insert(input_field, connection);

        self.run_order.clear();
    }

    pub fn clear_connections(&mut self, node_id: Uuid) {
        self.map.get_mut(&node_id).unwrap().links.clear();
        self.run_order.clear();
    }

    pub fn run_iteration(&mut self) {
        if self.run_order.is_empty() {
            self.run_order = toposort(&self.adj_list).unwrap();
        }

        println!("{:?} = RUN_ORDER", self.run_order);

        let mut storage: HashMap<Uuid, HashMap<String, Box<dyn Any>>> = HashMap::new();

        for node_id in &mut self.run_order {
            let mut input_map: HashMap<String, &dyn Any> = HashMap::new();
            let node = self.map.get_mut(node_id).unwrap();

            for (field_name, connection) in &node.links {
                let field = &storage[&connection.output_uuid][&*connection.output_name];
                input_map.insert(field_name.to_owned(), field.as_ref());
            }

            let output = node.process(&input_map).unwrap();
            storage.insert(*node_id, output);
        }
    }
}
