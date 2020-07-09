use super::{RectGeometry, UiGlobalState, WidgetState};
use crate::widgets::Draw;
use nalgebra::geometry::Point2;
use ncollide2d::bounding_volume::{BoundingSphere, BoundingVolume};
use piet::{
    kurbo::{Arc, Circle, Point},
    Color, RenderContext,
};
use piet_web::WebRenderContext;
use serde::{Deserialize, Serialize};
use std::f64;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Smiley {
    pub name: String,
    pub uuid: String,
    pub visible: bool,
    pub geometry: RectGeometry,
    initial_geometry: RectGeometry,
}

impl Smiley {
    pub(crate) fn new(name: &str, center: (f64, f64), radius: f64) -> Self {
        let geometry = RectGeometry {
            x: center.0 - radius,
            y: center.1 - radius,
            width: radius * 2.,
            height: radius * 2.,
        };

        Smiley {
            name: String::from(name),
            uuid: Uuid::new_v4().to_string(),
            visible: true,
            geometry,
            initial_geometry: geometry,
        }
    }
}

impl Draw<WidgetState> for Smiley {
    fn update(&mut self, ui_state: &mut UiGlobalState) -> WidgetState {
        let mut state = WidgetState {
            uuid: String::from(&self.uuid),
            ..WidgetState::default()
        };

        // This is behavior with the cursor, only do if the widget is visible
        if self.visible {
            // Geometry
            let (x, y, radius) = get_widget_geometry(&self.geometry);

            // TODO not generic
            let current_widget_bounding = BoundingSphere::new(Point2::new(x, y), radius);

            // Hovered: Test with current pointer position
            if let Some(cursor_position) = &ui_state.cursor.position {
                let current_cursor_bounding = BoundingSphere::new(*cursor_position, 1.0);
                state.hovered = current_widget_bounding.intersects(&current_cursor_bounding)
                    && !ui_state.cursor.is_active;
            }

            // Selected
            if let Some(state_uuid) = &ui_state.active_widget_uuid {
                // Set selected if it was already selected
                state.selected = self.uuid.eq(state_uuid);
            }

            if let Some(pos) = &ui_state.cursor.down_start_position {
                // TODO not generic
                let (x, y, radius) = get_widget_geometry(&self.initial_geometry);
                let initial_widget_bounding = BoundingSphere::new(Point2::new(x, y), radius);
                let initial_cursor_bounding = BoundingSphere::new(*pos, 1.0);

                let is_initial_mouse_over =
                    initial_widget_bounding.intersects(&initial_cursor_bounding);

                if is_initial_mouse_over && !ui_state.cursor.is_active {
                    state.selected = true;

                    if let Some(cursor_position) = &ui_state.cursor.position {
                        if let Some(uuid) = &ui_state.active_widget_uuid {
                            if self.uuid == *uuid {
                                self.geometry.x =
                                    self.initial_geometry.x + (cursor_position.x - pos.x);
                                self.geometry.y =
                                    self.initial_geometry.y + (cursor_position.y - pos.y);
                            }
                        }
                    }
                }
            } else {
                // mouseout or mouseup
                self.initial_geometry = self.geometry;
            }
        }

        state.geometry = self.geometry;
        state
    }

    fn change(&mut self, changes: &RectGeometry) {
        self.geometry = RectGeometry {
            x: self.geometry.x + changes.x,
            y: self.geometry.y + changes.y,
            width: self.geometry.width + changes.width,
            height: self.geometry.height + changes.height,
        };
    }

    fn draw(&self, context: &mut WebRenderContext, _ui_state: &UiGlobalState) {
        let (x, y, radius) = get_widget_geometry(&self.geometry);

        let stroke_brush = context.solid_brush(Color::rgb8(0x8b, 0x69, 0x14));
        let fill_brush = context.solid_brush(Color::rgb8(0xff, 0xd7, 0x00));

        // Draw the background
        context.fill(Circle::new((x, y), radius - 1.), &fill_brush);

        // Draw the outer circle
        context.stroke(Circle::new((x, y), radius - 1.), &stroke_brush, 2.);

        // Draw the mouth
        let mouth_radius = (radius - radius * 0.3).floor();
        let path = Arc {
            center: Point::new(x, y),
            radii: Point::new(mouth_radius, mouth_radius).to_vec2(),
            start_angle: 0.0,
            sweep_angle: f64::consts::PI,
            x_rotation: 0.0,
        };
        context.stroke(path, &stroke_brush, 2.0);

        // eyes
        let eye_radius = radius * 0.1;
        let eye_y = (y - radius * 0.2).floor();

        // Draw the left eye
        let eye_x = (x - radius * 0.3).floor();
        context.stroke(Circle::new((eye_x, eye_y), eye_radius), &stroke_brush, 2.0);

        // Draw the right eye
        let eye_x = (x + radius * 0.3).floor();
        context.stroke(Circle::new((eye_x, eye_y), eye_radius), &stroke_brush, 2.0);
    }
}

fn get_widget_geometry(geometry: &RectGeometry) -> (f64, f64, f64) {
    let radius = (geometry.width / 2.).min(geometry.height / 2.);
    let x = geometry.x + radius + (geometry.width / 2. - radius) + 0.5;
    let y = geometry.y + radius + (geometry.height / 2. - radius) + 0.5;

    (x, y, radius)
}
