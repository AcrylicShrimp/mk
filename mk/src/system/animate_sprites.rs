use crate::component::*;
use crate::time::*;
use legion::*;

pub fn animate_sprites(world: &mut World, time_manager: &TimeManager) {
	for renderer in <&mut SpriteRenderer>::query().iter_mut(world) {
		match &mut renderer.sprite {
			SpriteType::Sprite(..) => {}
			SpriteType::Animated(animation) => {
				animation.update(time_manager);
			}
		}
	}
}
