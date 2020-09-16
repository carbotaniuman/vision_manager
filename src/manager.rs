use std::collections::HashMap;

use vision_traits::{schema::Function, Node, NodeProcessable, NodeCreationError};

pub struct Manager {
    type_map: HashMap<String, &'static dyn Fn(&str) -> Result<Box<dyn NodeProcessable>, NodeCreationError>>,
    schema_map: HashMap<String, Function>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            type_map: HashMap::new(),
            schema_map: HashMap::new(),
        }
    }

    fn register_raw(
        &mut self,
        name: String,
        schema: Function,
        constructor: &'static dyn Fn(&str) -> Result<Box<dyn NodeProcessable>, NodeCreationError>,
    ) {
        self.type_map.insert(name.to_owned(), constructor);
        self.schema_map.insert(name, schema);
    }

    pub fn register<T: Node>(&mut self) {
        self.register_raw(
            format!("{}", T::NAME),
            T::get_schema(),
            &<T as NodeProcessable>::make,
        );
    }

    pub fn make_node(&self, type_name: &str, json: &str) -> Box<dyn NodeProcessable> {
        let func = self.type_map[type_name];
        func(json).unwrap()
    }

    // pub fn create_pipeline(&self, input: &str) -> Pipeline {
    //     Pipeline::new()
    // }
}
