use bevy::prelude::*;
use bevy::ecs::component::{ComponentHooks,StorageType};

/// A component that when added to an entity, will clone the inner "child_bundle" and insert it as a child of the entity.
/// The WithChild component remains on the entity. Future versions of this component may allow for the removal of the WithChild component after the child has been added.
#[derive(Debug, Clone, Default)]
pub struct WithChild<B: Bundle + Clone>{
    child_bundle: B
}
impl <B: Bundle + Clone> WithChild<B> {
    pub fn new(child_bundle: B) -> Self {
        Self {
            child_bundle
        }
    }
}
impl<B: Bundle + Clone> Component for WithChild<B> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity_id, _id| {
            let entity_mut = world.entity_mut(entity_id);
            let with_child = entity_mut.get_ref::<WithChild<B>>().unwrap();
            let child_bundle = with_child.child_bundle.clone();
            let mut cmds = world.commands();
            let mut enity_cmds = cmds.entity(entity_id);
            enity_cmds.with_child(child_bundle);
        });
    }
}

/// A component that when added to an entity, will clone all inner "child_bundles" and insert them as a child of the entity.
#[derive(Debug, Clone, Default)]
pub struct WithChildren<B: Bundle + Clone>{
    child_bundles: Vec<B>
}
impl <B: Bundle + Clone> WithChildren<B> {
    pub fn new(child_bundles: Vec<B>) -> Self {
        Self {
            child_bundles
        }
    }
}
impl<B: Bundle + Clone> Component for WithChildren<B> {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity_id, _id| {
            let entity_mut = world.entity_mut(entity_id);
            let with_children = entity_mut.get_ref::<WithChildren<B>>().unwrap();
            let child_bundles = with_children.child_bundles.clone();
            let mut cmds = world.commands();
            let mut enity_cmds = cmds.entity(entity_id);
            for child_bundle in child_bundles {
                enity_cmds.with_child(child_bundle);
            }
        });
    }
}