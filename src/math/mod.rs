//! Math utilities: Transform2D, Rect, Circle, and helper functions.

pub mod circle;
pub mod rect;
pub mod transform2d;
pub mod utils;

// Re-export glam types as engine math primitives
pub use glam::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4};

pub use circle::Circle;
pub use rect::Rect;
pub use transform2d::Transform2D;
pub use utils::*;
