use bevy::{
    asset::{Assets, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        query::{Changed, Or, With, Without},
        system::{Commands, Query, ResMut},
    },
    math::{vec2, Vec2, Vec3},
};

use bevy::render::mesh::Mesh;
use bevy::sprite::Mesh2dHandle;

use crate::svg::Svg;

#[derive(Clone, Component, Copy, Debug, Default, PartialEq)]
/// Origin of the coordinate system.
pub enum Origin {
    /// Bottom left of the image or viewbox.
    BottomLeft,
    /// Bottom right of the image or viewbox.
    BottomRight,
    /// Center of the image or viewbox.
    Center,
    #[default]
    /// Top left of the image or viewbox, this is the default for a SVG.
    TopLeft,
    /// Top right of the image or viewbox.
    TopRight,
    /// Custom origin, top left is (0, 0), bottom right is (1, 1)
    Custom((f32, f32)),
}

impl Origin {
    /// Computes the translation for an origin. The resulting translation needs to be added
    /// to the translation of the SVG.
    pub fn compute_translation(&self, scaled_size: Vec2) -> Vec3 {
        match self {
            Origin::BottomLeft => Vec3::new(0.0, scaled_size.y, 0.0),
            Origin::BottomRight => Vec3::new(-scaled_size.x, scaled_size.y, 0.0),
            Origin::Center => Vec3::new(-scaled_size.x * 0.5, scaled_size.y * 0.5, 0.0),
            // Standard SVG origin is top left, so we don't need to do anything
            Origin::TopLeft => Vec3::ZERO,
            Origin::TopRight => Vec3::new(-scaled_size.x, 0.0, 0.0),
            Origin::Custom(coord) => {
                Vec3::new(-scaled_size.x * coord.0, scaled_size.y * coord.1, 0.0)
            }
        }
    }
    /// aaa
    pub fn get_relative_offset(&self) -> Vec2 {
        match self {
            Origin::Custom(coord) => vec2(coord.0, coord.1),
            _ => vec2(0.0, 0.0),
        }
    }
}

impl From<&Origin> for Vec2 {
    fn from(value: &Origin) -> Self {
        match value {
            Origin::Custom(coord) => vec2(coord.0, coord.1),
            _ => vec2(0.0, 0.0),
        }
    }
}

#[derive(Clone, Component, Copy, Debug, PartialEq)]
pub(crate) struct OriginState {
    previous: Origin,
}

#[cfg(feature = "2d")]
#[cfg(not(feature = "3d"))]
type WithMesh = With<Mesh2dHandle>;
#[cfg(not(feature = "2d"))]
#[cfg(feature = "3d")]
type WithMesh = With<Handle<Mesh>>;
#[cfg(all(feature = "2d", feature = "3d"))]
type WithMesh = Or<(With<Mesh2dHandle>, With<Handle<Mesh>>)>;

/// Checkes if a "new" SVG bundle was added by looking for a missing OriginState
/// and then adds it to the entity.
pub(crate) fn add_origin_state(
    mut commands: Commands,
    query: Query<Entity, (With<Handle<Svg>>, WithMesh, Without<OriginState>)>,
) {
    for entity in &query {
        commands.entity(entity).insert(OriginState {
            previous: Origin::default(),
        });
    }
}

#[cfg(feature = "2d")]
pub(crate) fn apply_origin_change_2d(
    mut svgs: ResMut<Assets<Svg>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut options: Query<(&Handle<Svg>, &mut Mesh2dHandle, &Origin, &mut OriginState)>,
) {
    for (svg, mut mesh, origin, mut prev) in options.iter_mut() {
        if prev.previous != *origin {
            if let Some(svg) = svgs.get_mut(svg) {
                let new_mesh = svg.tessellate(origin.get_relative_offset());
                let new_mesh_handle = meshes.add(new_mesh);
                *mesh = new_mesh_handle.into();
                prev.previous = *origin;
            }
        }
    }
}

#[cfg(feature = "3d")]
pub(crate) fn apply_origin_change_3d(
    mut svgs: ResMut<Assets<Svg>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut options: Query<(&Handle<Svg>, &mut Handle<Mesh>, &Origin, &mut OriginState)>,
) {
    for (svg, mut mesh, origin, mut prev) in options.iter_mut() {
        if prev.previous != *origin {
            if let Some(svg) = svgs.get_mut(svg) {
                let new_mesh = svg.tessellate(origin.get_relative_offset());
                let new_mesh_handle = meshes.add(new_mesh);
                *mesh = new_mesh_handle.into();
                prev.previous = *origin;
            }
        }
    }
}
