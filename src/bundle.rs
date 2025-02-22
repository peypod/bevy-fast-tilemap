use super::prelude::*;
use bevy::prelude::*;
use bevy::sprite::{Material2d, Material2dPlugin, MeshMaterial2d};

// Bundle of components you should typically have for a map.
#[derive(Bundle, Clone)]
pub struct MapBundleUnmanaged<C: Customization> {
    pub loading: MapLoading,
    pub attributes: MapAttributes,

    pub material: MapHandle<C>,
    pub mesh: Mesh2d,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

impl<C: Customization> Default for MapBundleUnmanaged<C> {
    fn default() -> Self {
        Self {
            loading: Default::default(),
            attributes: Default::default(),
            material: Default::default(),
            mesh: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
        }
    }
}

impl<C: Customization> MapBundleUnmanaged<C> {
    pub fn new(map: Map<C>, materials: &mut Assets<Map<C>>) -> Self {
        Self {
            material: MapHandle(materials.add(map)),
            ..default()
        }
    }
}

// Bundle of components you should typically have for a map.
#[derive(Bundle, Clone)]
pub struct MapBundleManaged<C: Customization> {
    pub loading: MapLoading,
    pub attributes: MapAttributes,

    pub material: MapHandle<C>,
    pub mesh: Mesh2d,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,

    pub mesh_managed_by_map: MeshManagedByMap,
}

impl<C: Customization> Default for MapBundleManaged<C> {
    fn default() -> Self {
        Self {
            loading: Default::default(),
            attributes: Default::default(),
            material: Default::default(),
            mesh: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            inherited_visibility: Default::default(),
            view_visibility: Default::default(),
            mesh_managed_by_map: Default::default(),
        }
    }
}

impl<C: Customization> MapBundleManaged<C> {
    pub fn new(map: Map<C>, materials: &mut Assets<Map<C>>) -> Self {
        Self {
            material: MapHandle(materials.add(map)),
            ..default()
        }
    }
}
