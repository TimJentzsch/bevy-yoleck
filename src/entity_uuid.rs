use bevy::prelude::*;

use bevy::utils::{HashMap, Uuid};

#[derive(Component, Debug)]
pub struct YoleckEntityUuid(pub(crate) Uuid);

impl YoleckEntityUuid {
    pub fn get(&self) -> Uuid {
        self.0
    }
}

#[derive(Resource)]
pub struct YoleckUuidRegistry(pub(crate) HashMap<Uuid, Entity>);

impl YoleckUuidRegistry {
    pub fn get(&self, uuid: Uuid) -> Option<Entity> {
        self.0.get(&uuid).copied()
    }
}