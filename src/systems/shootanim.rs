use crate::animation::InOutAnimation;
use crate::renderer::rendering_info::RenderingInfo;
use crate::util::lerp;

pub fn shootanim(
    shoot_animation: &mut InOutAnimation,
    rendering_info: &mut RenderingInfo,
    delta_time: f32,
) {
    shoot_animation.update(delta_time);
    rendering_info.fov_shootanim.y =
        lerp(0.0f32, -20.0f32.to_radians(), shoot_animation.get_value());
}
