use super::{RectGeometry, UiGlobalState, WidgetState};
use crate::widgets::{Draw, GeometryChangeState};
use nalgebra::geometry::Point2;
use ncollide2d::bounding_volume::{BoundingSphere, BoundingVolume};
use piet::{
    kurbo::{Arc, Circle, Point},
    Color, RenderContext,
};
use piet_web::WebRenderContext;
use serde::{Deserialize, Serialize};
use std::cmp::min;
use std::f64;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geometry {
    pub center: Point2<f64>,
    pub radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Smiley {
    pub name: String,
    pub uuid: String,
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
            geometry: geometry.clone(),
            initial_geometry: geometry,
        }
    }
}

impl Draw<WidgetState> for Smiley {
    fn update(&mut self, ui_state: &mut UiGlobalState) -> WidgetState {
        // Geometry
        let radius = self.geometry.width.floor() / 2.;
        let x = self.geometry.x + radius;
        let y = self.geometry.y + radius;

        // State
        let mut state = WidgetState::default();
        state.uuid = String::from(&self.uuid);
        state.hovered = false;
        state.selected = false;

        // Widget bounding sphere
        let current_widget_bounding = BoundingSphere::new(Point2::new(x, y), radius);

        // Hovered: Test with current pointer position
        if let Some(cursor_position) = &ui_state.cursor.position {
            let current_cursor_bounding = BoundingSphere::new(cursor_position.clone(), 1.0);
            state.hovered = current_widget_bounding.intersects(&current_cursor_bounding)
                && !ui_state.cursor.is_active;
        }

        // Selected
        if let Some(state_uuid) = &ui_state.active_widget_uuid {
            // Set selected if it was already selected
            state.selected = self.uuid.eq(state_uuid);
        }

        if let Some(pos) = &ui_state.cursor.down_start_position {
            let radius = self.initial_geometry.width / 2.;
            let mousedown_widget_bounding = BoundingSphere::new(
                Point2::new(
                    self.initial_geometry.x + radius,
                    self.initial_geometry.y + radius,
                ),
                radius,
            );
            let initial_cursor_bounding = BoundingSphere::new(pos.clone(), 1.0);

            let is_initial_mouse_over =
                mousedown_widget_bounding.intersects(&initial_cursor_bounding);

            if is_initial_mouse_over && !ui_state.cursor.is_active {
                state.selected = true;

                if let Some(cursor_position) = &ui_state.cursor.position {
                    if let Some(uuid) = &ui_state.active_widget_uuid {
                        if self.uuid == *uuid {
                            self.geometry.x = self.initial_geometry.x + (cursor_position.x - pos.x);
                            self.geometry.y = self.initial_geometry.y + (cursor_position.y - pos.y);
                        }
                    }
                }
            }
        } else {
            // mouseout or mouseup
            self.initial_geometry = self.geometry.clone();
        }

        state.geometry = self.geometry.clone();
        // self.state = state.clone();
        state
    }

    // TODO: Need return?
    fn change(&mut self, changes: GeometryChangeState) -> WidgetState {
        self.geometry = RectGeometry {
            x: self.geometry.x + changes.geometry.x,
            y: self.geometry.y + changes.geometry.y,
            width: self.geometry.width + changes.geometry.width,
            height: self.geometry.height + changes.geometry.height,
        };

        //self.state.geometry = self.geometry.clone();

        //self.state.clone()
        WidgetState {
            uuid: self.uuid.clone(),
            selected: false,
            hovered: false,
            geometry: self.geometry.clone(), // !important
        }
    }

    fn draw(&self, context: &mut WebRenderContext, _ui_state: &UiGlobalState) {
        let radius = (self.geometry.width / 2.).min(self.geometry.height / 2.);
        let x = self.geometry.x + radius + (self.geometry.width / 2. - radius);
        let y = self.geometry.y + radius + (self.geometry.height / 2. - radius);

        let stroke_brush = context.solid_brush(Color::rgb8(0x8b, 0x69, 0x14));
        let fill_brush = context.solid_brush(Color::rgb8(0xff, 0xd7, 0x00));

        // Draw the background
        context.fill(Circle::new((x, y), radius), &fill_brush);

        // Draw the outer circle
        context.stroke(Circle::new((x, y), radius), &stroke_brush, 2.0);

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
