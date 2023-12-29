use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{CurrentLod, LodSettings};

#[derive(Component)]
pub struct LodGroup<T: Bundle + Clone>(Box<[T]>);

impl<T: Bundle + Clone> LodGroup<T> {
    #[must_use]
    pub fn new(bundles: Box<[T]>) -> Self {
        Self(bundles)
    }

    #[must_use]
    pub fn get(&self, lod: u8) -> T {
        let index = (lod as usize).clamp(0, self.0.len() - 1);
        self.0[index].clone()
    }
}

fn update_lod_groups<T: Bundle + Clone>(
    mut commands: Commands,
    lod_settings: Res<LodSettings>,
    lod_groups: Query<(Entity, &CurrentLod, &LodGroup<T>), Changed<CurrentLod>>,
) {
    lod_groups.for_each(|(entity, current_lod, lod_group)| {
        let lod = current_lod.0.saturating_add_signed(lod_settings.bias);
        let bundle = lod_group.get(lod);

        commands.entity(entity).insert(bundle);
    });
}

pub struct LodGroupPlugin<T>(PhantomData<T>);

impl<T> Default for LodGroupPlugin<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Bundle + Clone> Plugin for LodGroupPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<LodSettings>()
            .add_systems(Update, update_lod_groups::<T>);
    }
}
