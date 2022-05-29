use crate::{
    api::use_context,
    component::{Camera, Transform},
    ui::UIElement,
};
use legion::{Entity, EntityStore};
use std::collections::{btree_map::Entry, BTreeMap};

#[derive(Debug)]
pub struct UIManager {
    elements: Vec<UIElement>,
    entities: Vec<Entity>,
    last_order_indices: Vec<u32>,
    ordered_indices: BTreeMap<u32, Vec<u32>>,
    removed_indices: Vec<u32>,
}

impl UIManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn alloc(&mut self, entity: Entity) -> u32 {
        if let Some(index) = self.removed_indices.pop() {
            self.elements[index as usize] = UIElement::default();
            self.entities[index as usize] = entity;
            self.last_order_indices[index as usize] = 0;
            return index;
        }

        let index = self.elements.len() as u32;
        self.elements.push(UIElement::default());
        self.entities.push(entity);
        self.last_order_indices.push(0);
        index
    }

    pub fn dealloc(&mut self, index: u32) {
        if let Entry::Occupied(mut entry) = self
            .ordered_indices
            .entry(self.last_order_indices[index as usize])
        {
            if let Some(index) = entry.get().iter().position(|&element| element == index) {
                entry.get_mut().swap_remove(index);
            }

            if entry.get().is_empty() {
                entry.remove_entry();
            }
        }

        self.elements[index as usize].reset_flags();
        self.removed_indices.push(index);
    }

    pub fn element(&self, index: u32) -> &UIElement {
        &self.elements[index as usize]
    }

    pub fn element_mut(&mut self, index: u32) -> &mut UIElement {
        &mut self.elements[index as usize]
    }

    pub fn entity(&self, index: u32) -> Entity {
        self.entities[index as usize]
    }

    pub fn raycast_element(
        &self,
        x: f32,
        y: f32,
        camera: Option<(&Transform, &Camera)>,
    ) -> Option<Entity> {
        let context = use_context();
        let world = context.world();
        let screen_mgr = context.screen_mgr();
        let transform_mgr = context.transform_mgr();

        let camera_x = x - screen_mgr.width() as f32 * 0.5f32;
        let camera_y = -y + screen_mgr.height() as f32 * 0.5f32;

        // TODO: Capsulate below matrix calculations.

        let camera_to_world = match camera {
            Some(camera) => {
                let camera_transform_index = camera.0.index();
                transform_mgr
                    .transform_world_matrix(camera_transform_index)
                    .clone()
            }
            None => [
                1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32,
            ],
        };

        for (_, indices) in &self.ordered_indices {
            for &index in indices {
                let element = &self.elements[index as usize];
                if !element.is_interactible() {
                    continue;
                }

                let entry = match world.entry_ref(self.entities[index as usize]) {
                    Ok(entry) => entry,
                    Err(_) => continue,
                };
                let transform = match entry.get_component::<Transform>() {
                    Ok(transform) => transform,
                    Err(_) => continue,
                };
                let transform = transform_mgr.transform(transform.index());
                let mut world_to_local = [0f32; 9];
                let mut camera_to_local = [0f32; 6];

                transform.to_matrix_inverse(&mut world_to_local);
                camera_to_local[0] = camera_to_world[0] * world_to_local[0]
                    + camera_to_world[1] * world_to_local[3]
                    + camera_to_world[2] * world_to_local[6];
                camera_to_local[1] = camera_to_world[0] * world_to_local[1]
                    + camera_to_world[1] * world_to_local[4]
                    + camera_to_world[2] * world_to_local[7];
                camera_to_local[2] = camera_to_world[3] * world_to_local[0]
                    + camera_to_world[4] * world_to_local[3]
                    + camera_to_world[5] * world_to_local[6];
                camera_to_local[3] = camera_to_world[3] * world_to_local[1]
                    + camera_to_world[4] * world_to_local[4]
                    + camera_to_world[5] * world_to_local[7];
                camera_to_local[4] = camera_to_world[6] * world_to_local[0]
                    + camera_to_world[7] * world_to_local[3]
                    + camera_to_world[8] * world_to_local[6];
                camera_to_local[5] = camera_to_world[6] * world_to_local[1]
                    + camera_to_world[7] * world_to_local[4]
                    + camera_to_world[8] * world_to_local[7];

                let local_x = camera_x * camera_to_local[0]
                    + camera_y * camera_to_local[2]
                    + camera_to_local[4];
                let local_y = camera_x * camera_to_local[1]
                    + camera_y * camera_to_local[3]
                    + camera_to_local[5];

                if 0f32 <= local_x
                    && local_x <= element.size.x
                    && -element.size.y <= local_y
                    && local_y <= 0f32
                {
                    return Some(self.entities[index as usize]);
                }
            }
        }

        None
    }

    pub fn update_elements(&mut self) {
        for (index, element) in self.elements.iter_mut().enumerate() {
            if !element.is_dirty() {
                continue;
            }

            if let Entry::Occupied(mut entry) =
                self.ordered_indices.entry(self.last_order_indices[index])
            {
                if let Some(index) = entry
                    .get()
                    .iter()
                    .position(|&element| element == index as u32)
                {
                    entry.get_mut().swap_remove(index);
                }

                if entry.get().is_empty() {
                    entry.remove_entry();
                }
            }

            let order_index = element.order_index();
            self.ordered_indices
                .entry(order_index)
                .or_default()
                .push(index as u32);
            self.last_order_indices[index] = order_index;
        }
    }
}

impl Default for UIManager {
    fn default() -> Self {
        Self {
            elements: Vec::with_capacity(1024),
            entities: Vec::with_capacity(1024),
            last_order_indices: Vec::with_capacity(1024),
            ordered_indices: BTreeMap::new(),
            removed_indices: Vec::with_capacity(1024),
        }
    }
}