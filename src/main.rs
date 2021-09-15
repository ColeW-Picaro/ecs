#[derive(Clone, Debug, PartialEq, Eq)]
struct Entity(usize);

fn main() {
    
}

struct EntityManager {
    entities: Vec<Entity>,
    containers: Vec<Box<dyn Storage>>
}

impl EntityManager {
    fn add_entity(&mut self) -> Entity {
        let entity: Entity = Entity(self.entities.len());
        self.entities.push(entity.clone());
        entity
    }

    fn register_component<T: Storage + Default + 'static>(&mut self) -> usize {
        let component: usize = self.containers.len();
        self.containers.push(Box::new(T::default()));
        component
    }
}

trait Storage {
    fn allocate(&mut self, entity: Entity) -> usize;
}

/*
 * Example Component: Position
 * Component are two parts, the component itself and the store
 * Component: Simply a data structure
 * Store: A Vec of the Component; Must implement Storage and Default
*/
struct Position {
    entity: Entity,
    x: i64,
    y: i64,
}

struct PositionStore {
    storage: Vec<Position>
}

impl Storage for PositionStore {
    fn allocate(&mut self, entity: Entity) -> usize {
        self.storage.push(Position {
            entity,
            x: 0,
            y: 0 
        });
        self.storage.len() - 1
    }
}

impl Default for PositionStore {
    fn default() -> PositionStore {
        PositionStore { storage: Vec::new() }
    }
}

mod tests {
    use super::*;
    #[test]
    fn create_entity_manager() {
        let entity_manager = EntityManager {
            entities: Vec::new(),
            containers: Vec::new()
        };
    }

    #[test]
    fn create_entity() {
        let mut entity_manager = EntityManager {
            entities: Vec::new(),
            containers: Vec::new()
        };

        let ent = entity_manager.add_entity();
        assert_eq!(Entity(0), ent);
    }

    #[test]
    fn register_component() {
        let mut entity_manager = EntityManager {
            entities: Vec::new(),
            containers: Vec::new()
        };

        let component = entity_manager.register_component::<PositionStore>();
        assert_eq!(0, component);
    }
}