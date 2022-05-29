use crate::api::use_context;
use crate::component::{Camera, Transform};
use crate::structure::Vec2;
use glutin::event::MouseButton;
use legion::*;

#[derive(Debug)]
struct MouseDown {
    entity: Option<Entity>,
    button: MouseButton,
}

#[derive(Debug)]
struct MouseDrag {
    entity: Entity,
    button: MouseButton,
}

#[derive(Default, Debug)]
pub struct UIEventManager {
    pub camera: Option<Entity>,
    pub focus: Option<Entity>,
    mouse_in: Option<Entity>,
    mouse_down: Option<MouseDown>,
    mouse_drag: Option<MouseDrag>,
    last_mouse_position: Option<Vec2>,
}

impl UIEventManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_mouse_exit(&mut self) {
        if let Some(mouse_in_entity) = self.mouse_in.take() {
            // TODO: Send mouse out event to the mouse_in_entity.
        }
        self.last_mouse_position = None;
    }

    pub fn handle_mouse_move(&mut self, x: f32, y: f32) {
        let context = use_context();
        let mut world = context.world_mut();
        let entry = self.camera.and_then(|camera| world.entry(camera));
        let entity = entry
            .and_then(|entry| {
                match (
                    entry.get_component::<Transform>(),
                    entry.get_component::<Camera>(),
                ) {
                    (Ok(transform), Ok(camera)) => Some(context.ui_mgr().raycast_element(
                        x,
                        y,
                        Some((transform, camera)),
                    )),
                    _ => None,
                }
            })
            .unwrap_or_else(|| context.ui_mgr().raycast_element(x, y, None));

        match entity {
            Some(entity) => {
                if let Some(mouse_in_entity) = self.mouse_in {
                    if entity != mouse_in_entity {
                        // TODO: Send mouse out event to the mouse_in_entity.
                        // TODO: Send mouse in event to the entity.
                        self.mouse_in = Some(entity);
                    }
                }

                // TODO: Send mouse move event to the entity.
            }
            None => {
                if let Some(mouse_in_entity) = self.mouse_in.take() {
                    // TODO: Send mouse out event to the mouse_in_entity.
                }
            }
        }

        match self.mouse_down.as_ref().and_then(|mouse_down| {
            mouse_down
                .entity
                .and_then(|entity| Some((entity, mouse_down.button)))
        }) {
            Some((entity, mouse_down)) if self.mouse_drag.is_none() => {
                // TODO: Send mouse drag begin event to the entity.
                self.mouse_drag = Some(MouseDrag {
                    entity,
                    button: mouse_down,
                });
            }
            _ => {}
        }

        self.last_mouse_position = Some(Vec2::new(x, y));
    }

    pub fn handle_mouse_button_down(&mut self, button: MouseButton) {
        self.mouse_down = None;

        if let Some(mouse_drag) = self.mouse_drag.take() {
            // TODO: Send drag end event to the mouse_drag.entity.
        }

        let last_mouse_position = match &self.last_mouse_position {
            Some(last_mouse_position) => last_mouse_position,
            None => {
                if let Some(focus_entity) = self.focus.take() {
                    // TODO: Send focus out event to the focus_entity.
                }
                return;
            }
        };
        let context = use_context();
        let mut world = context.world_mut();
        let entry = self.camera.and_then(|camera| world.entry(camera));
        let entity = entry
            .and_then(|entry| {
                match (
                    entry.get_component::<Transform>(),
                    entry.get_component::<Camera>(),
                ) {
                    (Ok(transform), Ok(camera)) => Some(context.ui_mgr().raycast_element(
                        last_mouse_position.x,
                        last_mouse_position.y,
                        Some((transform, camera)),
                    )),
                    _ => None,
                }
            })
            .unwrap_or_else(|| {
                context
                    .ui_mgr()
                    .raycast_element(last_mouse_position.x, last_mouse_position.y, None)
            });

        match entity {
            Some(entity) => {
                self.mouse_down = Some(MouseDown {
                    entity: Some(entity),
                    button,
                });

                if let Some(focus_entity) = self.focus {
                    if entity != focus_entity {
                        // TODO: Send focus out event to the focus_entity.
                        // TODO: Send focus in event to the entity.
                        self.focus = Some(entity);
                    }
                }

                // TODO: Send mouse down to the entity.
            }
            None => {
                self.mouse_down = Some(MouseDown {
                    entity: None,
                    button,
                });

                if let Some(focus) = self.focus.take() {
                    // TODO: Send focus out event to the focus.
                }
            }
        }
    }

    pub fn handle_mouse_button_up(&mut self, button: MouseButton) {
        self.mouse_down = None;

        let last_mouse_position = match &self.last_mouse_position {
            Some(last_mouse_position) => last_mouse_position,
            None => {
                if let Some(mouse_drag) = self.mouse_drag.take() {
                    // TODO: Send drag end event to the mouse_drag.entity.
                }
                return;
            }
        };
        let context = use_context();
        let mut world = context.world_mut();
        let entry = self.camera.and_then(|camera| world.entry(camera));
        let entity = entry
            .and_then(|entry| {
                match (
                    entry.get_component::<Transform>(),
                    entry.get_component::<Camera>(),
                ) {
                    (Ok(transform), Ok(camera)) => Some(context.ui_mgr().raycast_element(
                        last_mouse_position.x,
                        last_mouse_position.y,
                        Some((transform, camera)),
                    )),
                    _ => None,
                }
            })
            .unwrap_or_else(|| {
                context
                    .ui_mgr()
                    .raycast_element(last_mouse_position.x, last_mouse_position.y, None)
            });

        match entity {
            Some(entity) => {
                if let Some(mouse_drag) = self.mouse_drag.take() {
                    // TODO: Send drop event to the entity.
                    // TODO: Send drag end event to the mouse_drag.entity.
                }

                // TODO: Send mouse up to the entity.
            }
            None => {
                if let Some(mouse_drag) = self.mouse_drag.take() {
                    // TODO: Send drag end event to the mouse_drag.entity.
                }
            }
        }
    }
}
