#![allow(dead_code)]
#![allow(unused_macros)]
use std::{any::{Any, type_name}, collections::HashMap};


// Макрос для создания сущности с компонентами
#[macro_export]
macro_rules! entity {
    ($id:expr, $($component:ident),*) => {{
        let mut entity = Entity::new($id);
        $(
            entity.add_component($component::default());
        )*
        entity
    }};
}

pub struct Entity {
    pub id: usize,
    components: HashMap<String, Box<dyn Any>>,
}

impl Entity {
    pub fn new(id: usize) -> Self {
        Entity {
            id,
            components: HashMap::new(),
        }
    }

    pub fn add_component<T: 'static>(&mut self, component: T) {
        self.components.insert(type_name::<T>().to_string(), Box::new(component));
    }

    pub fn get_component<T: 'static>(&self) -> Option<&T> {
        self.components.get(&type_name::<T>().to_string()).and_then(|c| c.downcast_ref())
    }

    pub fn get_component_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.components.get_mut(&type_name::<T>().to_string()).and_then(|c| c.downcast_mut())
    }
}