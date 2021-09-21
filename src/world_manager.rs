use std::{cell::{RefCell, RefMut}, iter::{Copied, FilterMap, Zip}, slice::Iter};

use crate::{ComponentError, Storage};
pub struct EntityManager {
    pub entity_count: usize,
    pub components: Vec<Box<dyn Storage>>,
}

impl EntityManager {
    pub fn add_entity(&mut self) -> usize {
        let entity_id = self.entity_count;
        for component in self.components.iter_mut() {
            component.allocate();
        }
        self.entity_count += 1;
        entity_id
    }

    pub fn borrow_component_store<T: 'static>(
        &self,
    ) -> Result<RefMut<Vec<Option<T>>>, ComponentError> {
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

    pub fn get_component<T: 'static + Clone>(&self, entity: usize) -> Option<T> {
        let component_store: Result<RefMut<Vec<Option<T>>>, ComponentError> =
            self.borrow_component_store();
        return match component_store {
            Ok(store) => store[entity].clone(),
            Err(_) => None,
        };
    }

    pub fn iter_over_entities_with_components<T: 'static, U: 'static>(
        &self
    ) -> Copied<impl Iterator<Item = (&T, &U)>> {
        let store1 = self.borrow_component_store::<T>().unwrap();
        let store2 = self.borrow_component_store::<U>().unwrap();
        let zip = store1.iter().zip(store2.iter());
        let copy = zip.filter_map(|(c1, c2)| Some((c1.as_ref()?.clone(), c2.as_ref()?.clone()))).copied();
        copy
    }
}
