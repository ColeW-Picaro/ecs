use std::{any::Any, cell::{RefCell, RefMut}};

use components::*;

pub mod components;

fn main() {
    let mut entity_manager = EntityManager {
        entity_count: 0,
        components: Vec::new(),
    };

    let place_ent = entity_manager.add_entity();
    entity_manager.add_component_to_entity(place_ent, Position { x: 1, y: 1 });

    let moving_ent = entity_manager.add_entity();
    entity_manager.add_component_to_entity(moving_ent, Velocity { vel: 1.0 });

    let place_and_moving_ent = entity_manager.add_entity();
    entity_manager.add_component_to_entity(place_and_moving_ent, Position { x: 1, y: 1 });
    entity_manager.add_component_to_entity(place_and_moving_ent, Velocity { vel: 1.0 });

    let pos_store = entity_manager.borrow_component_store::<Position>().unwrap();
    let vel_store = entity_manager.borrow_component_store::<Velocity>().unwrap();

    let zip = pos_store.iter().zip(vel_store.iter());
    for (pos, vel) in zip.filter_map(|(pos, vel)| Some((pos.as_ref()?, vel.as_ref()?))) {
        println!("Position: x: {}, y: {}", pos.x, pos.y);
        println!("Velocity: {}", vel.vel);
    }
}

pub struct EntityManager {
    entity_count: usize,
    components: Vec<Box<dyn Storage>>,
}

#[derive(Debug)]
pub struct ComponentError {
    msg: String,
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

    fn borrow_component_store<T: 'static>(&self) -> Result<RefMut<Vec<Option<T>>>, ComponentError> {
        for component_vec in self.components.iter() {
            if let Some(component_vec) = component_vec
                .as_any()
                .downcast_ref::<RefCell<Vec<Option<T>>>>()
            {
                return Ok(component_vec.borrow_mut());
            }
        }
        Err(ComponentError {
            msg: String::from("Component does not exist"),
        })
    }

    pub fn add_component_to_entity<T: 'static>(&mut self, entity: usize, component: T) {
        for component_store in self.components.iter_mut() {
            if let Some(component_store) = component_store
                .as_any_mut()
                .downcast_mut::<RefCell<Vec<Option<T>>>>()
            {
                component_store.get_mut()[entity] = Some(component);
                return;
            }
        }

        let mut new_component_store = Vec::<Option<T>>::with_capacity(self.entity_count);
        for _ in 0..self.entity_count {
            new_component_store.push(None);
        }
        new_component_store[entity] = Some(component);
        self.components
            .push(Box::new(RefCell::new(new_component_store)));
    }

    pub fn get_component<T: 'static + Clone>(&self, entity: usize) -> Option<T>{
        let component_store : Result<RefMut<Vec<Option<T>>>, ComponentError> =
            self.borrow_component_store();
        return match component_store {
            Ok(store) => store[entity].clone(),
            Err(_) => None
        };
    }
}

trait Storage {
    fn allocate(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> Storage for RefCell<Vec<Option<T>>> {
    fn allocate(&mut self) {
        self.get_mut().push(None);
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
    fn add_component_to_entity() {
        let mut entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new(),
        };

        let ent = entity_manager.add_entity();

        entity_manager.add_component_to_entity(ent, Position { x: 1, y: 2 });
    }

    #[test]
    fn get_component_from_entity() {
        let mut entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new(),
        };

        let place_ent = entity_manager.add_entity();
        entity_manager.add_component_to_entity(place_ent, Position { x: 1, y: 1 });

        let component : Position = entity_manager.get_component(place_ent).unwrap();
        assert_eq!(1, component.x);
        assert_eq!(1, component.y);
    }

    #[test]
    fn get_component_from_entity_err() {
        let mut entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new(),
        };

        let place_ent = entity_manager.add_entity();
        entity_manager.add_component_to_entity(place_ent, Position { x: 1, y: 1 });

        assert!(entity_manager.get_component::<Velocity>(place_ent).is_none());
    }

    #[test]
    fn update_component() {
        let mut entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new(),
        };

        let ent = entity_manager.add_entity();
        let first_pos = Position { x: 1, y: 2 };
        let second_pos = Position { x: 2, y: 3 };

        entity_manager.add_component_to_entity(ent, first_pos.clone());
        assert_eq!(first_pos, entity_manager.get_component(ent).unwrap());

        entity_manager.add_component_to_entity(ent, second_pos.clone());
        assert_eq!(second_pos, entity_manager.get_component(ent).unwrap());
    }


}
