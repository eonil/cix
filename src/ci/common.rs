pub struct UUID {
    core: uuid::Uuid,
}

pub fn new_v4() -> UUID {
    UUID {
        core: uuid::Uuid::new_v4(),
    }
}

