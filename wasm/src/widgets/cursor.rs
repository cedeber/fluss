use crate::widgets::RectGeometry;
use crate::widgets::{rect::Anchor, Draw, UiGlobalState};
use nalgebra::Point2;
use piet::{
    kurbo::{Circle, Line},
    Color, RenderContext,
};
use piet_web::WebRenderContext;
use serde::{Deserialize, Serialize};
use std::f64;

pub static POINTER_SIZE: f64 = 8.0;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Cursor {
    pub position: Option<Point2<f64>>,
    pub previous_position: Option<Point2<f64>>,
    pub down_start_position: Option<Point2<f64>>,
    pub is_active: bool,
    pub active_anchor: Anchor,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            position: None,
            previous_position: None,
            down_start_position: None,
            is_active: false,
            active_anchor: Anchor::None,
        }
    }
}

impl Draw<()> for Cursor {
    fn update(&mut self, _ui_state: &mut UiGlobalState) {
        unimplemented!()
    }

    fn change(&mut self, _changes: &RectGeometry) {
        unimplemented!()
    }

    fn draw(&self, context: &mut WebRenderContext, ui_state: &UiGlobalState) {
        if let Some(position) = &self.position {
            // Add 0.5px to keep sharpness
            let x = position.x;
            let y = position.y;
            let width = ui_state.canvas_geometry.width;
            let height = ui_state.canvas_geometry.height;

            let color = if ui_state.cursor.is_active {
                Color::rgba8(0xB8, 0x2E, 0xE5, 0x77) // PINK
            } else {
                Color::rgba8(0x00, 0x00, 0x00, 0x77)
            };

            // Pointer
            let brush = context.solid_brush(color);
            context.fill(Circle::new((x, y), POINTER_SIZE), &brush);

            if !ui_state.cursor.is_active {
                // Lines
                let brush = context.solid_brush(Color::rgba8(0x00, 0x00, 0x00, 0x10));

                // vertical lines
                context.stroke(
                    Line::new((x + 0.5, 0.0), (x + 0.5, y - POINTER_SIZE - 1.0)),
                    &brush,
                    1.0,
                );

                context.stroke(
                    Line::new((x + 0.5, y + POINTER_SIZE + 1.0), (x + 0.5, height)),
                    &brush,
                    1.0,
                );

                // horizontal lines
                context.stroke(
                    Line::new((0.0, y + 0.5), (x - POINTER_SIZE - 1.0, y + 0.5)),
                    &brush,
                    1.0,
                );

                context.stroke(
                    Line::new((x + POINTER_SIZE + 1.0, y + 0.5), (width, y + 0.5)),
                    &brush,
                    1.0,
                );
            }
        }
    }
}
