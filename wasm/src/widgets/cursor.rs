use crate::widgets::GeometryChangeState;
use crate::{
    widgets::{Draw, UiGlobalState},
    POINTER_SIZE,
};
use nalgebra::Point2;
use piet::{
    kurbo::{Circle, Line},
    Color, RenderContext,
};
use piet_web::WebRenderContext;
use std::f64;

#[derive(Default, Debug, Copy, Clone)]
pub struct Cursor {
    pub position: Option<Point2<f64>>,
    pub previous_position: Option<Point2<f64>>,
    pub down_start_position: Option<Point2<f64>>,
    pub is_active: bool,
}

impl Draw<()> for Cursor {
    fn update(&mut self, _ui_state: &mut UiGlobalState) {
        unimplemented!()
    }

    fn change(&mut self, _changes: GeometryChangeState) {
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
                Color::rgba8(0xAA, 0x00, 0x00, 0x77)
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
