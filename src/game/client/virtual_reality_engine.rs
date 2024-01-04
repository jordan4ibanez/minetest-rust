use glam::{DVec3, Quat};

///
/// I have no hardware to test VR so this is purely speculation.
///
/// Consider this the "dream high, set expectations normal" part
/// of the minetest-rust engine.
///
/// I do not know what I would do for this, but here are some ideas.
///
pub struct VirtualRealityEngine {
  // No idea what kind of struct this would be.
  headset: i32,
  // No idea what kind of struct this would be.
  controllers: i32,

  // So this is where the vr headset's "root" position is.
  center_pos: DVec3,
  // And this is the current offset from the "root position".
  current_offset_pos: DVec3,
  // The rotation of the headset in 3D space.
  current_rotation: Quat,
}
