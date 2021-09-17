use std::{any::Any, borrow::BorrowMut, cell::{RefCell, RefMut}};

pub mod components;

fn main() {}

pub struct EntityManager {
    entity_count: usize,
    components: Vec<Box<dyn Storage>>,

}

pub struct ComponentError {
    msg: String
}

impl EntityManager {


    fn add_entity(&mut self) -> usize {
        let entity_id = self.entity_count;
        for component in self.components.iter_mut() {
            component.allocate();
        }
        self.entity_count += 1;
        entity_id
    }

    fn create_component_store<T: 'static>(&mut self) -> RefCell<Vec<Option<T>>> {
        let mut new_component_store = Vec::<Option<T>>::with_capacity(self.entity_count);
        for _ in 0..self.entity_count {
            new_component_store.push(None);
        }
        RefCell::new(new_component_store)
    }

    fn register_component_store<T: 'static>(&mut self, component_store: RefCell<Vec<Option<T>>>) {
        self.components.push(Box::new(component_store));
    }

    fn borrow_component_store<T: 'static>(&self) -> Result<RefMut<Vec<Option<T>>>, ComponentError> {
        for component_vec in self.components.iter() {
            if let Some(component_vec) = component_vec.as_any().downcast_ref::<RefCell<Vec<Option<T>>>>()
            {
                return Ok(component_vec.borrow_mut());
            }
        }
        Err(ComponentError { msg: String::from("Component does not exist")})
    }

    pub fn register_component<T: 'static>(&mut self) {
        let new_component_store = self.create_component_store::<T>();
        self.register_component_store(new_component_store);
    }

    pub fn add_component_to_entity<T: 'static>(&mut self, entity: usize, component: T) {
        let component_store_result = self.borrow_component_store::<T>();
        match component_store_result {
            Ok(store) => store.borrow_mut()[entity] = Some(component),
            Err(_) => self.register_component::<T>()
        }
    }

    pub fn get_component_from_entity<T: 'static>(&mut self, entity: usize) -> Result<&T, ComponentError> {
        let component_store_result = self.borrow_component_store::<T>();
        match component_store_result {
            Ok(store) => {
                return match &store.get(entity) {
                    Some(c) => {
                        match c {
                            Some()
                        }
                    },
                    None => Err(ComponentError { msg: String::from("Entity does not have component") })
                }
            },
            Err(_) => {
                self.register_component::<T>();
                Err(ComponentError { msg: String::from("Entity does not have component") })
            }
        }
    }
}

trait Storage {
    fn allocate(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> Storage for RefCell<Vec<Option<T>>> {
    fn allocate(&mut self) {
        self.borrow().push(None);
    }
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

mod tests {
    use super::*;
    use crate::components::*;
    #[test]
    fn create_entity_manager() {
        let entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new(),
        };
    }

    #[test]
    fn create_entity() {
        let mut entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new(),
        };

        let ent = entity_manager.add_entity();
        assert_eq!(0, ent);
    }

    #[test]
    fn register_component() {
        let mut entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new(),
        };

        entity_manager.register_component::<Position>();
    }

    #[test]
    fn register_different_components() {
        let mut entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new(),
        };

        entity_manager.register_component::<Position>();
        entity_manager.register_component::<Velocity>();
    }

    #[test]
    fn add_component_to_entity() {
        let mut entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new(),
        };

        entity_manager.register_component::<Position>();

        let ent = entity_manager.add_entity();

        entity_manager.add_component_to_entity(ent, Position { x: 1, y: 2 });
    }

    #[test]
    fn get_component_from_entity() {
        let mut entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new(),
        };

        entity_manager.register_component::<Position>();

        let ent = entity_manager.add_entity();

        entity_manager.add_component_to_entity(ent, Position { x: 1, y: 2 });

        let pos = entity_manager.get_component_from_entity::<Position>(ent);
        match pos {
            Ok(p) => assert_eq!(1, p.x),
            Err(e) => assert_eq!("component error", e.msg)
        }
    }
}
