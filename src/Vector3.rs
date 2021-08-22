pub struct Vector3<T> {
  unsafe{
  union {
    struct {T x, y ,z};
    struct {T v0, v1 ,v2};
    struct {T r, g b};
    T data[3];
    T rgb[3];
  },
  }
}

impl Vector3<T> {
  pub fn new(_x: T, _y:T, _z: T) {
    Vector3{
      x: _x,
      y: _y,
      z : _z,
    }
  }
}