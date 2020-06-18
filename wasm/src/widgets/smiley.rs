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
use std::f64;
use uuid::Uuid;
use wasm_bindgen::convert::FromWasmAbi;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geometry {
    pub center: Point2<f64>,
    pub radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Smiley {
    pub geometry: Geometry,
    pub name: String,
    pub uuid: String,
    initial_geometry: Geometry,
    state: WidgetState,
}

impl Smiley {
    pub(crate) fn new(name: &str, center: (f64, f64), radius: f64) -> Self {
        let geometry = Geometry {
            center: Point2::new(center.0, center.1),
            radius,
        };

        Smiley {
            geometry: geometry.clone(),
            name: String::from(name),
            uuid: Uuid::new_v4().to_string(),
            initial_geometry: geometry,
            state: WidgetState::default(),
        }
    }
}

impl Draw<WidgetState> for Smiley {
    fn update(&mut self, ui_state: &mut UiGlobalState) -> WidgetState {
        // Geometry
        let radius = self.geometry.radius.floor();
        let x = self.geometry.center.x;
        let y = self.geometry.center.y;

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
        if let Some(state_uuid) = &ui_state.select_widget_uuid {
            // Set selected if it was already selected
            state.selected = self.uuid.eq(state_uuid);
        }

        if let Some(pos) = &ui_state.cursor.down_start_position {
            let mousedown_widget_bounding = BoundingSphere::new(
                Point2::new(
                    self.initial_geometry.center.x,
                    self.initial_geometry.center.y,
                ),
                radius,
            );
            let initial_cursor_bounding = BoundingSphere::new(pos.clone(), 1.0);

            let is_initial_mouse_over =
                mousedown_widget_bounding.intersects(&initial_cursor_bounding);

            if is_initial_mouse_over && !ui_state.cursor.is_active {
                state.selected = true;

                if let Some(cursor_position) = &ui_state.cursor.position {
                    if let Some(uuid) = &ui_state.select_widget_uuid {
                        if self.uuid == *uuid {
                            self.geometry.center = Point2::new(
                                self.initial_geometry.center.x + (cursor_position.x - pos.x),
                                self.initial_geometry.center.y + (cursor_position.y - pos.y),
                            );
                        }
                    }
                }
            }
        } else {
            // mouseout or mouseup
            self.initial_geometry.center = self.geometry.center.clone();
        }

        state.geometry = calculate_geometry(&self.geometry);
        self.state = state.clone();
        state
    }

    fn change(&mut self, changes: GeometryChangeState) -> WidgetState {
        self.geometry = Geometry {
            radius: self.geometry.radius + changes.geometry.x,
            // center: Point2::new(
            //     widget.geometry.center.x + change_state.geometry.x,
            //     widget.geometry.center.y + change_state.geometry.y,
            // ),
            ..self.geometry.clone()
        };

        self.state.geometry = calculate_geometry(&self.geometry);

        self.state.clone()
    }

    fn draw(&self, context: &mut WebRenderContext, _ui_state: &UiGlobalState) {
        let x = self.geometry.center.x;
        let y = self.geometry.center.y;
        let radius = self.geometry.radius;

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

fn calculate_geometry(geometry: &Geometry) -> RectGeometry {
    RectGeometry {
        x: geometry.center.x - geometry.radius,
        y: geometry.center.y - geometry.radius,
        width: geometry.radius * 2.0,
        height: geometry.radius * 2.0,
    }
}
