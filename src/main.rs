pub mod components;

fn main() {

}

pub struct EntityManager {
    entity_count: usize,
    components: Vec<Box<dyn Storage>>
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

    fn register_component<T: Storage + Default + 'static>(&mut self) -> usize {
        let component: usize = self.components.len();
        self.components.push(Box::new(T::default()));
        component
    }

    fn add_component<T: Storate + Default + 'static>(&mut self, entity: usize, component: T) {

    }
}

trait Storage {
    fn allocate(&mut self);
}

mod tests {
    use crate::components::*;
    use super::*;
    #[test]
    fn create_entity_manager() {
        let entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new()
        };
    }

    #[test]
    fn create_entity() {
        let mut entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new()
        };

        let ent = entity_manager.add_entity();
        assert_eq!(0, ent);
    }

    #[test]
    fn register_component() {
        let mut entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new()
        };

        let component = entity_manager.register_component::<PositionStore>();
        assert_eq!(0, component);
    }

    #[test]
    fn register_different_components() {
        let mut entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new()
        };

        let pos = entity_manager.register_component::<PositionStore>();
        let vel = entity_manager.register_component::<VelocityStore>();
        assert_eq!(0, pos);
        assert_eq!(1, vel);
    }

    #[test]
    fn add_component_to_entity() {
        let mut entity_manager = EntityManager {
            entity_count: 0,
            components: Vec::new()
        };

        let pos = entity_manager.register_component::<PositionStore>();

        let ent = entity_manager.add_entity();

        entity_manager.add_component(entity, Position { x: 1, y: 2 });
    }
}
