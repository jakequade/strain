use amethyst::core::math::Vector2;

//#region GenericBox

#[derive(Clone)]
pub struct GenericBox {
  pub half_size: Vector2<f32>,
  pub old_position: Vector2<f32>,
  pub position: Vector2<f32>,
}

impl Default for GenericBox {
  fn default() -> Self {
    Self {
      half_size: Vector2::new(0., 0.),
      old_position: Vector2::new(0., 0.),
      position: Vector2::new(0., 0.),
    }
  }
}

impl GenericBox {
  pub fn new(width: f32, height: f32) -> Self {
    GenericBox {
      half_size: Vector2::new(width / 2., height / 2.),
      old_position: Vector2::new(0., 0.),
      position: Vector2::new(0., 0.),
    }
  }
}

//#endregion GenericBox

//#region Collider

//#endregion
