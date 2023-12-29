use std::marker::PhantomData;

use bevy::{math::Vec3A, prelude::*};

use crate::CurrentLod;

pub trait LodResolver: Send + Sync + 'static {
    const RESOLUTION: f32;
    fn resolve_lod(distance_squared: f32) -> u8;
}

fn update_lods<C: Component, R: LodResolver>(
    mut last_update_position: Local<Option<Vec3A>>,
    camera_query: Query<&GlobalTransform, (With<C>, Changed<GlobalTransform>)>,
    mut lods: Query<(&GlobalTransform, &mut CurrentLod)>,
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };
    let camera_position = camera_transform.translation_vec3a();

    if let Some(last_position_update) = last_update_position.as_mut() {
        if camera_position.distance_squared(*last_position_update) < R::RESOLUTION {
            return;
        }
    }

    *last_update_position = Some(camera_position);

    lods.for_each_mut(|(object_transform, mut lod)| {
        let object_position = object_transform.translation_vec3a();
        let distance_squared = camera_position.distance_squared(object_position);

        lod.0 = R::resolve_lod(distance_squared);
    });
}

fn obj_update_lods<C: Component, R: LodResolver>(
    camera_query: Query<&GlobalTransform, With<C>>,
    mut lods: Query<(&GlobalTransform, &mut CurrentLod), Changed<GlobalTransform>>,
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        return;
    };
    let camera_position = camera_transform.translation_vec3a();

    lods.for_each_mut(|(object_transform, mut lod)| {
        let object_position = object_transform.translation_vec3a();
        let distance_squared = camera_position.distance_squared(object_position);

        lod.0 = R::resolve_lod(distance_squared);
    });
}

pub struct LodResolverPlugin<C: Component, R: LodResolver>(PhantomData<(C, R)>);

impl<C: Component, R: LodResolver> Default for LodResolverPlugin<C, R> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C: Component, R: LodResolver> Plugin for LodResolverPlugin<C, R> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_lods::<C, R>, obj_update_lods::<C, R>));
    }
}
